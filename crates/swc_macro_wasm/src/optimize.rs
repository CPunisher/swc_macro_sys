use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::{Visit, VisitMut, VisitMutWith, VisitWith};
use swc_ecma_ast::*;
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{Emitter, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
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

// Extract literal-ish key from an Object property
fn prop_key(k: &PropName) -> Option<String> {
    match k {
        PropName::Ident(i) => Some(i.sym.to_string()),
        PropName::Str(s) => Some(s.value.to_string()),
        PropName::Num(n) => Some(n.value.to_string()),
        PropName::Computed(ComputedPropName { expr, .. }) => {
            match &**expr {
                Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
                _ => None, // unknown => keep
            }
        }
        _ => None,
    }
}

// Collect __webpack_require__(...) literal arguments within a factory function
struct RequireCollector {
    out: HashSet<String>,
}

impl RequireCollector {
    fn new() -> Self { 
        Self { out: HashSet::new() } 
    }
}

impl Visit for RequireCollector {
    fn visit_call_expr(&mut self, n: &CallExpr) {
        if let Callee::Expr(callee) = &n.callee {
            if let Expr::Ident(id) = &**callee {
                if &*id.sym == "__webpack_require__" {
                    if let Some(a0) = n.args.get(0) {
                        match a0.expr.as_ref() {
                            Expr::Lit(Lit::Str(s)) => { self.out.insert(s.value.to_string()); }
                            Expr::Lit(Lit::Num(n)) => { self.out.insert(n.value.to_string()); }
                            Expr::Tpl(tpl) if tpl.exprs.is_empty() && tpl.quasis.len() == 1 => {
                                let q = &tpl.quasis[0];
                                let s = q.cooked
                                    .as_ref()
                                    .map(|a| a.to_string())
                                    .unwrap_or_else(|| q.raw.to_string());
                                self.out.insert(s);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        n.visit_children_with(self);
    }
}

// Compute reachability within a single modules object
fn reachable_from(entry: &str, graph: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut seen = HashSet::new();
    let mut stack = vec![entry.to_string()];
    seen.insert(entry.to_string());
    
    while let Some(cur) = stack.pop() {
        if let Some(neighbors) = graph.get(&cur) {
            for n in neighbors {
                if seen.insert(n.clone()) {
                    stack.push(n.clone());
                }
            }
        }
    }
    seen
}

// Prunes only the object at payload[1] or payload[2] in *.push([chunkIds, modulesObj, ...])
pub struct ModulesObjectPruner<'a> {
    // config JSON root
    pub config: &'a serde_json::Value,
    // Statistics
    pub total_pruned: usize,
    pub total_kept: usize,
    pub pushes_processed: usize,
    // Track actual module names
    pub kept_module_names: Vec<String>,
    pub removed_module_names: Vec<String>,
}

impl<'a> ModulesObjectPruner<'a> {
    pub fn new(config: &'a serde_json::Value) -> Self {
        Self {
            config,
            total_pruned: 0,
            total_kept: 0,
            pushes_processed: 0,
            kept_module_names: Vec::new(),
            removed_module_names: Vec::new(),
        }
    }

    // Pick the entry id that is actually present as a key in this modules object
    fn pick_entry_for_modules(&self, present: &HashSet<String>) -> Option<String> {
        let tree = self.config.get("treeShake")?.as_object()?;
        for lib in tree.values() {
            if let Some(id) = lib
                .get("chunk_characteristics")
                .and_then(|cc| cc.get("entry_module_id"))
                .and_then(|v| v.as_str())
            {
                if present.contains(id) {
                    eprintln!("Found matching entry '{}' in modules object", id);
                    return Some(id.to_string());
                }
            }
        }
        None
    }

    // Build intra-object graph: id -> deps where deps are limited to keys present in this object
    fn build_graph_for_modules(&self, obj: &ObjectLit) -> (HashMap<String, Vec<String>>, HashSet<String>) {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut present: HashSet<String> = HashSet::new();

        // First pass: collect keys
        for p in &obj.props {
            if let PropOrSpread::Prop(pp) = p {
                if let Prop::KeyValue(kv) = &**pp {
                    if let Some(k) = prop_key(&kv.key) {
                        present.insert(k);
                    }
                }
            }
        }

        // Second pass: collect __webpack_require__ deps per key if value is a function
        for p in &obj.props {
            if let PropOrSpread::Prop(pp) = p {
                if let Prop::KeyValue(kv) = &**pp {
                    if let Some(id) = prop_key(&kv.key) {
                        let mut deps = Vec::new();
                        
                        // Check if the value is a function and collect requires
                        match kv.value.as_ref() {
                            Expr::Fn(FnExpr { function, .. }) => {
                                let mut rc = RequireCollector::new();
                                function.visit_with(&mut rc);
                                for d in rc.out {
                                    if present.contains(&d) {
                                        deps.push(d);
                                    }
                                }
                            }
                            Expr::Arrow(arrow) => {
                                let mut rc = RequireCollector::new();
                                arrow.visit_with(&mut rc);
                                for d in rc.out {
                                    if present.contains(&d) {
                                        deps.push(d);
                                    }
                                }
                            }
                            _ => {
                                // Not a function - no dependencies to extract
                            }
                        }
                        graph.insert(id, deps);
                    }
                }
            }
        }

        // Ensure all nodes exist in graph
        for id in &present {
            graph.entry(id.clone()).or_default();
        }

        (graph, present)
    }

    fn prune_object(&mut self, obj: &mut ObjectLit) {
        let (graph, present) = self.build_graph_for_modules(obj);
        let original_count = present.len();
        
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Analyzing modules object with {} modules", original_count).into());
        
        let Some(entry) = self.pick_entry_for_modules(&present) else {
            #[cfg(target_arch = "wasm32")]
            {
                web_sys::console::log_1(&format!("No matching entry found in modules object with {} keys", original_count).into());
                web_sys::console::log_1(&"  Available entries in config:".into());
                if let Some(tree) = self.config.get("treeShake").and_then(|t| t.as_object()) {
                    for (lib_name, lib_config) in tree {
                        if let Some(id) = lib_config
                            .get("chunk_characteristics")
                            .and_then(|cc| cc.get("entry_module_id"))
                            .and_then(|v| v.as_str())
                        {
                            web_sys::console::log_1(&format!("    {} -> {}", lib_name, id).into());
                        }
                    }
                }
            }
            return; // entry not in this push => do not touch
        };
        
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(&format!("Using entry module: {}", entry).into());
            
            // Log some sample dependencies
            let mut sample_count = 0;
            for (module_id, deps) in &graph {
                if sample_count >= 3 { break; }
                if !deps.is_empty() {
                    web_sys::console::log_1(&format!("  Module {} requires: {:?}", module_id, deps).into());
                    sample_count += 1;
                }
            }
        }
        
        let keep = reachable_from(&entry, &graph);
        
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Reachability analysis: {} modules reachable from entry", keep.len()).into());
        
        if keep.len() == present.len() {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("All {} modules are reachable from entry", present.len()).into());
            self.total_kept += present.len();
            // Track all modules as kept
            for module_name in &present {
                self.kept_module_names.push(module_name.clone());
            }
            return; // nothing to remove
        }

        let to_prune = present.len() - keep.len();
        eprintln!("Pruning {} unreachable modules from {} total (keeping {})", 
            to_prune, present.len(), keep.len());

        obj.props.retain(|p| {
            match p {
                PropOrSpread::Prop(pp) => match &**pp {
                    Prop::KeyValue(kv) => {
                        if let Some(k) = prop_key(&kv.key) {
                            let should_keep = keep.contains(&k);
                            if should_keep {
                                self.total_kept += 1;
                                self.kept_module_names.push(k.clone());
                            } else {
                                self.total_pruned += 1;
                                self.removed_module_names.push(k.clone());
                            }
                            should_keep
                        } else {
                            self.total_kept += 1;
                            true // unknown/computed key: keep
                        }
                    }
                    _ => {
                        self.total_kept += 1;
                        true // not a KV prop: keep
                    }
                },
                PropOrSpread::Spread(_) => {
                    self.total_kept += 1;
                    true // keep spreads
                }
            }
        });
    }
}

impl<'a> VisitMut for ModulesObjectPruner<'a> {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        // Match *.push(...)
        let is_push = if let Callee::Expr(callee_expr) = &call.callee {
            if let Expr::Member(member) = &**callee_expr {
                if let MemberProp::Ident(ident) = &member.prop {
                    ident.sym == "push"
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        
        if !is_push {
            call.visit_mut_children_with(self);
            return;
        }

        if call.args.len() != 1 {
            call.visit_mut_children_with(self);
            return;
        }
        
        let Some(arg) = call.args.get_mut(0) else {
            call.visit_mut_children_with(self);
            return;
        };
        
        let Some(arr) = arg.expr.as_mut_array() else {
            call.visit_mut_children_with(self);
            return;
        };

        // Webpack push payload: [chunkIds, modulesObj, runtime?]
        let mut target_idx = None;
        
        // Check index 1 for modules object
        if arr.elems.len() > 1 {
            if let Some(Some(elem)) = arr.elems.get(1) {
                if matches!(&*elem.expr, Expr::Object(_)) {
                    target_idx = Some(1);
                }
            }
        }
        
        // Check index 2 if not found at 1
        if target_idx.is_none() && arr.elems.len() > 2 {
            if let Some(Some(elem)) = arr.elems.get(2) {
                if matches!(&*elem.expr, Expr::Object(_)) {
                    target_idx = Some(2);
                }
            }
        }
        
        let Some(idx) = target_idx else {
            call.visit_mut_children_with(self);
            return;
        };

        let Some(Some(elem)) = arr.elems.get_mut(idx) else {
            call.visit_mut_children_with(self);
            return;
        };
        
        let Some(obj) = elem.expr.as_mut_object() else {
            call.visit_mut_children_with(self);
            return;
        };

        self.pushes_processed += 1;
        self.prune_object(obj);

        call.visit_mut_children_with(self);
    }
}

pub fn optimize(source: String, config: serde_json::Value) -> String {
    let (optimized, _) = optimize_with_prune_result(source, config);
    optimized
}

pub fn optimize_with_prune_result(source: String, config: serde_json::Value) -> (String, PruneResult) {
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source.clone());
        let comments = SingleThreadedComments::default();
        
        let program = match Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program() {
            Ok(program) => program,
            Err(e) => {
                eprintln!("SWC parsing failed: {:?}", e);
                return (source, PruneResult::new_skipped("Parsing failed".to_string(), 0));
            }
        };
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");
        parser.parse(&comments)
    };

    let config_clone = config.clone();

    // 1) Macro evaluation
    let mut transformer = condition_transform(config, macros);
    program.visit_mut_with(&mut transformer);

    // 2) DCE
    let program = swc_common::GLOBALS.set(&Default::default(), || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();
        
        program.mutate(resolver(unresolved_mark, top_level_mark, false));
        perform_dce(&mut program, comments.clone(), unresolved_mark);
        program.mutate(fixer(Some(&comments)));
        
        program
    });

    // 3) Prune modules objects now (first analysis pass happens here)
    let mut pruner = ModulesObjectPruner::new(&config_clone);
    let mut program_mut = program;
    
    // Use web_sys to log to console since eprintln might not work in WASM
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&format!("Starting module pruning with config").into());
    
    program_mut.visit_mut_with(&mut pruner);
    
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&format!("Module pruning complete: {} pushes processed, {} modules pruned, {} modules kept",
        pruner.pushes_processed, pruner.total_pruned, pruner.total_kept).into());
    
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("Module pruning complete: {} pushes processed, {} modules pruned, {} modules kept",
        pruner.pushes_processed, pruner.total_pruned, pruner.total_kept);
    
    // Build prune result
    let prune_result = if pruner.total_pruned > 0 {
        PruneResult::new_pruned(
            pruner.kept_module_names,
            pruner.removed_module_names,
            pruner.total_kept + pruner.total_pruned
        )
    } else if pruner.pushes_processed > 0 {
        // No pruning happened, but we still have kept modules
        let mut result = PruneResult::new_skipped(
            format!("No modules pruned from {} pushes", pruner.pushes_processed),
            pruner.total_kept
        );
        result.kept_modules = pruner.kept_module_names;
        result
    } else {
        PruneResult::new_skipped(
            "No webpack push calls found".to_string(),
            0
        )
    };

    // Emit final program
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
        if let Err(e) = emitter.emit_program(&program_mut) {
            eprintln!("Failed to emit program: {:?}", e);
            return (source, prune_result);
        }
        drop(emitter);

        match String::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to convert to UTF-8: {:?}", e);
                return (source, prune_result);
            }
        }
    };

    (ret, prune_result)
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