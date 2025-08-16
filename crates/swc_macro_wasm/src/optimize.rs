use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{Expr, ExprOrSpread, Program, Prop, PropName};
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{Emitter, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use crate::webpack_parser::WebpackChunkParser;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PruneResult {
    pub kept_modules: Vec<String>,
    pub removed_modules: Vec<String>,
    pub original_count: usize,
    pub pruned_count: usize,
    pub skip_reason: Option<String>,
}

impl PruneResult {
    pub fn new_skipped(reason: String, original_count: usize) -> Self {
        Self {
            kept_modules: Vec::new(),
            removed_modules: Vec::new(),
            original_count,
            pruned_count: 0,
            skip_reason: Some(reason),
        }
    }
    
    pub fn new_pruned(kept: Vec<String>, removed: Vec<String>, original_count: usize) -> Self {
        let pruned_count = removed.len();
        Self {
            kept_modules: kept,
            removed_modules: removed,
            original_count,
            pruned_count,
            skip_reason: None,
        }
    }
}

pub fn optimize(source: String, config: serde_json::Value) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source.clone());
        let comments = SingleThreadedComments::default();
        
        // Handle parsing errors gracefully without panicking
        let program = match Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program() {
            Ok(program) => program,
            Err(e) => {
                eprintln!("SWC parsing failed: {:?}", e);
                eprintln!("Returning original source due to parsing error");
                // Return the original source if parsing fails
                return source;
            }
        };
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");

        parser.parse(&comments)
    };

    // Clone config so we can still access it after passing into the transformer
    let config_clone = config.clone();

    let program = {
        let mut transformer = condition_transform(config, macros);
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));

            perform_dce(&mut program, comments.clone(), unresolved_mark);

            program.mutate(fixer(Some(&comments)));

            // After DCE, run webpack parser to build dependency graph for pruning
            // Emit current program (post-DCE) to string and analyze
            let mut intermediate_buf = vec![];
            {
                let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut intermediate_buf, None))
                    as Box<dyn WriteJs>;
                let mut emitter = Emitter {
                    cfg: codegen::Config::default().with_minify(false),
                    comments: Some(&comments),
                    cm: cm.clone(),
                    wr,
                };
                // If emit fails here, continue with original flow without analysis
                if emitter.emit_program(&program).is_err() {
                    return program;
                }
            }
            let dce_output = match String::from_utf8(intermediate_buf) {
                Ok(s) => s,
                Err(_) => return program,
            };

            // Parse webpack chunk and compute reachable set from entry
            if let Ok(parser) = WebpackChunkParser::new() {
                if let Ok(chunk) = parser.parse_chunk_file(&dce_output) {
                    let graph = parser.build_dependency_graph(&chunk);
                    eprintln!("DEBUG: Config passed to optimize: {:?}", config_clone.to_string());
                    
                    // Extract entry_module_id from the SINGLE library config passed
                    // The config has structure: { treeShake: { "library_name": { ...exports, chunk_characteristics: { entry_module_id: "..." } } } }
                    let entry_module_id = config_clone
                        .get("treeShake")
                        .and_then(|ts| ts.as_object())
                        .and_then(|obj| {
                            // Get the first (and should be only) library config
                            obj.values().next()
                        })
                        .and_then(|lib_config| lib_config.get("chunk_characteristics"))
                        .and_then(|cc| cc.get("entry_module_id"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    if let Some(entry) = entry_module_id {
                        eprintln!("DEBUG: Looking for entry module: {}", entry);
                        eprintln!("DEBUG: First 5 module IDs in chunk: {:?}", 
                            chunk.modules.keys().take(5).collect::<Vec<_>>());
                        // Only prune if entry exists in this chunk to avoid removing everything
                        if chunk.modules.contains_key(&entry) {
                            let reachable = compute_reachable(&graph, &entry);
                            let keep: HashSet<String> = reachable
                                .into_iter()
                                .filter(|id| chunk.modules.contains_key(id))
                                .collect();

                            if !keep.is_empty() {
                                // Mutate AST to remove unreachable module entries in the modules object
                                let mut pruner = PruneModulesVisitor { keep };
                                let mut program_mut = program;
                                program_mut.visit_mut_with(&mut pruner);
                                return program_mut;
                            }
                        }
                    }
                }
            }
 
            program
        })
    };

    let ret = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        if let Err(e) = emitter.emit_program(&program) {
            eprintln!("Failed to emit program: {:?}", e);
            return source;
        }
        drop(emitter);

        match String::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to convert to UTF-8: {:?}", e);
                return source;
            }
        }
    };

    ret
}

/// Wrapper around optimize that also returns detailed pruning information
pub fn optimize_with_prune_result(source: String, config: serde_json::Value) -> (String, PruneResult) {
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source.clone());
        let comments = SingleThreadedComments::default();
        
        // Handle parsing errors gracefully without panicking
        let program = match Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program() {
            Ok(program) => program,
            Err(e) => {
                eprintln!("SWC parsing failed: {:?}", e);
                eprintln!("Returning original source due to parsing error");
                // Return the original source if parsing fails
                return (source, PruneResult::new_skipped("Parsing failed".to_string(), 0));
            }
        };
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");
        parser.parse(&comments)
    };

    // Clone config so we can still access it after passing into the transformer
    let config_clone = config.clone();
    let mut prune_result: Option<PruneResult> = None;

    let program = {
        let mut transformer = condition_transform(config, macros);
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));

            perform_dce(&mut program, comments.clone(), unresolved_mark);

            program.mutate(fixer(Some(&comments)));

            // After DCE, run webpack parser to build dependency graph for pruning
            // Emit current program (post-DCE) to string and analyze
            let mut intermediate_buf = vec![];
            {
                let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut intermediate_buf, None))
                    as Box<dyn WriteJs>;
                let mut emitter = Emitter {
                    cfg: codegen::Config::default().with_minify(false),
                    comments: Some(&comments),
                    cm: cm.clone(),
                    wr,
                };
                // If emit fails here, continue with original flow without analysis
                if emitter.emit_program(&program).is_err() {
                    prune_result = Some(PruneResult::new_skipped("Emit failed".to_string(), 0));
                    return program;
                }
            }
            let dce_output = match String::from_utf8(intermediate_buf) {
                Ok(s) => s,
                Err(_) => {
                    prune_result = Some(PruneResult::new_skipped("UTF-8 conversion failed".to_string(), 0));
                    return program;
                }
            };

            // Parse webpack chunk and compute reachable set from entry
            if let Ok(parser) = WebpackChunkParser::new() {
                if let Ok(chunk) = parser.parse_chunk_file(&dce_output) {
                    let original_count = chunk.modules.len();
                    let graph = parser.build_dependency_graph(&chunk);
                    
                    // Extract entry_module_id from the SINGLE library config passed
                    // The config has structure: { treeShake: { "library_name": { ...exports, chunk_characteristics: { entry_module_id: "..." } } } }
                    let entry_module_id = config_clone
                        .get("treeShake")
                        .and_then(|ts| ts.as_object())
                        .and_then(|obj| {
                            // Get the first (and should be only) library config
                            obj.values().next()
                        })
                        .and_then(|lib_config| lib_config.get("chunk_characteristics"))
                        .and_then(|cc| cc.get("entry_module_id"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    if let Some(entry) = entry_module_id {
                        eprintln!("DEBUG: Looking for entry module: {}", entry);
                        eprintln!("DEBUG: First 5 module IDs in chunk: {:?}", 
                            chunk.modules.keys().take(5).collect::<Vec<_>>());
                        // Only prune if entry exists in this chunk to avoid removing everything
                        if chunk.modules.contains_key(&entry) {
                            let reachable = compute_reachable(&graph, &entry);
                            let keep: HashSet<String> = reachable
                                .into_iter()
                                .filter(|id| chunk.modules.contains_key(id))
                                .collect();

                            if !keep.is_empty() {
                                let all_modules: HashSet<String> = chunk.modules.keys().cloned().collect();
                                let removed: Vec<String> = all_modules.difference(&keep).cloned().collect();
                                let kept: Vec<String> = keep.into_iter().collect();
                                
                                prune_result = Some(PruneResult::new_pruned(kept, removed, original_count));
                                
                                // Mutate AST to remove unreachable module entries in the modules object
                                let mut pruner = PruneModulesVisitor { keep: prune_result.as_ref().unwrap().kept_modules.iter().cloned().collect() };
                                let mut program_mut = program;
                                program_mut.visit_mut_with(&mut pruner);
                                return program_mut;
                            } else {
                                prune_result = Some(PruneResult::new_skipped("No reachable modules found".to_string(), original_count));
                            }
                        } else {
                            // Log what modules ARE in the chunk to debug
                            let module_ids: Vec<String> = chunk.modules.keys().take(5).cloned().collect();
                            prune_result = Some(PruneResult::new_skipped(
                                format!("Entry module '{}' not found in chunk. Sample module IDs: {:?}", entry, module_ids), 
                                original_count
                            ));
                        }
                    } else {
                        prune_result = Some(PruneResult::new_skipped("No entry module ID configured".to_string(), original_count));
                    }
                } else {
                    prune_result = Some(PruneResult::new_skipped("Failed to parse webpack chunk".to_string(), 0));
                }
            } else {
                prune_result = Some(PruneResult::new_skipped("Failed to create webpack parser".to_string(), 0));
            }
 
            program
        })
    };

    let ret = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        if let Err(e) = emitter.emit_program(&program) {
            eprintln!("Failed to emit program: {:?}", e);
            return (source, prune_result.unwrap_or_else(|| PruneResult::new_skipped("Emit failed".to_string(), 0)));
        }
        drop(emitter);

        match String::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to convert to UTF-8: {:?}", e);
                return (source, prune_result.unwrap_or_else(|| PruneResult::new_skipped("UTF-8 conversion failed".to_string(), 0)));
            }
        }
    };

    (ret, prune_result.unwrap_or_else(|| PruneResult::new_skipped("No pruning performed".to_string(), 0)))
}

fn perform_dce(m: &mut Program, comments: SingleThreadedComments, unresolved_mark: Mark) {
    let mut visitor = crate::dce::dce(
        comments,
        crate::dce::Config {
            module_mark: None,
            top_level: true,
            top_retain: Default::default(),
            preserve_imports_with_side_effects: true,
        },
        unresolved_mark,
    );

    loop {
        m.visit_mut_with(&mut visitor);

        if !visitor.changed() {
            break;
        }

        visitor.reset();
    }
}

fn compute_reachable(graph: &HashMap<String, Vec<String>>, start: &str) -> HashSet<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = vec![start.to_string()];
    while let Some(node) = stack.pop() {
        if visited.insert(node.clone()) {
            if let Some(deps) = graph.get(&node) {
                for dep in deps {
                    // Push dependency even if it's not present in the graph yet (graph ensures key exists)
                    stack.push(dep.clone());
                }
            }
        }
    }
    visited
}

struct PruneModulesVisitor {
    keep: HashSet<String>,
}

impl VisitMut for PruneModulesVisitor {
    fn visit_mut_call_expr(&mut self, call: &mut swc_ecma_ast::CallExpr) {
        // Look for something.push([...])
        if let swc_ecma_ast::Callee::Expr(callee_expr) = &call.callee {
            if let Expr::Member(member) = &**callee_expr {
                if let swc_ecma_ast::MemberProp::Ident(ident) = &member.prop {
                    if ident.sym.as_ref() == "push" {
                        // Expect first argument to be an array like [ [chunkName], { modules }, ... ]
                        if let Some(first_arg) = call.args.get_mut(0) {
                            if let swc_ecma_ast::ExprOrSpread { expr, .. } = first_arg {
                                if let Expr::Array(arr) = expr.as_mut() {
                                    if let Some(Some(second)) = arr.elems.get_mut(1) {
                                        if let Expr::Object(obj) = second.expr.as_mut() {
                                            // Filter module properties based on keep set
                                            obj.props.retain(|prop_or_spread| {
                                                if let swc_ecma_ast::PropOrSpread::Prop(p) = prop_or_spread {
                                                    if let Prop::KeyValue(kv) = &**p {
                                                        // Accept string or numeric keys only
                                                        match &kv.key {
                                                            PropName::Str(s) => self.keep.contains(&s.value.to_string()),
                                                            PropName::Num(n) => self.keep.contains(&n.value.to_string()),
                                                            _ => true, // Keep unknown patterns to be safe
                                                        }
                                                    } else {
                                                        true
                                                    }
                                                } else {
                                                    true
                                                }
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        call.visit_mut_children_with(self);
    }
}
