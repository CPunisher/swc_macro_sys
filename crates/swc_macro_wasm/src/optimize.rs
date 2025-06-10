use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::{VisitMutWith, VisitMut};
use swc_ecma_ast::{Program, Expr, VarDecl, Pat, ObjectLit, PropOrSpread, Prop, PropName};
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{Emitter, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use webpack_graph::{WebpackBundleParser, TreeShaker};
use rustc_hash::FxHashSet;

pub fn optimize(source: String, config: serde_json::Value) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source);
        let comments = SingleThreadedComments::default();
        let program = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program()
        .unwrap();
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");

        parser.parse(&comments)
    };

    let program = {
        let mut transformer = condition_transform(config, macros);
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));

            perform_dce(&mut program, comments.clone(), unresolved_mark);

            // Tree shake webpack modules after removing unused imports
            perform_webpack_tree_shaking(&mut program, cm.clone(), &comments);

            program.mutate(fixer(Some(&comments)));

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
        emitter.emit_program(&program).unwrap();
        drop(emitter);

        unsafe { String::from_utf8_unchecked(buf) }
    };

    ret
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

/// Performs webpack module tree shaking after DCE
fn perform_webpack_tree_shaking(program: &mut Program, cm: Lrc<SourceMap>, comments: &SingleThreadedComments) {
    // Step 1: Emit current AST to string for analysis
    let current_code = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: Some(comments),
            cm: cm.clone(),
            wr,
        };
        if emitter.emit_program(program).is_err() {
            return; // Skip tree shaking if emit fails
        }
        drop(emitter);
        
        match String::from_utf8(buf) {
            Ok(code) => code,
            Err(_) => return, // Skip tree shaking if invalid UTF-8
        }
    };

    // Step 2: Parse with webpack_graph to analyze module dependencies
    let parser = match WebpackBundleParser::new() {
        Ok(p) => p,
        Err(_) => return, // Skip tree shaking if parser creation fails
    };

    let mut graph = match parser.parse_bundle(&current_code) {
        Ok(g) => g,
        Err(_) => return, // Skip tree shaking if parsing fails - maybe it's not a webpack bundle
    };

    // Step 3: Use TreeShaker to identify unreachable modules
    let unreachable_modules = TreeShaker::new(&mut graph).shake();
    
    if !unreachable_modules.is_empty() {
        println!("Tree shaking: Removing {} unreachable webpack modules: {:?}", 
                 unreachable_modules.len(), unreachable_modules);
        
        // Step 4: Remove unreachable modules from the AST
        let unreachable_set: FxHashSet<String> = unreachable_modules.into_iter().collect();
        let mut module_remover = WebpackModuleRemover::new(unreachable_set);
        program.visit_mut_with(&mut module_remover);
    }
}

/// AST visitor that removes specified webpack modules from __webpack_modules__ objects
struct WebpackModuleRemover {
    modules_to_remove: FxHashSet<String>,
}

impl WebpackModuleRemover {
    fn new(modules_to_remove: FxHashSet<String>) -> Self {
        Self { modules_to_remove }
    }

    /// Check if a property key matches a module ID that should be removed
    fn should_remove_property(&self, prop: &PropOrSpread) -> bool {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                let module_id = match &kv.key {
                    PropName::Num(num) => num.value.to_string().split('.').next().unwrap_or("").to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(ident) => ident.sym.to_string(),
                    _ => return false,
                };
                return self.modules_to_remove.contains(&module_id);
            }
        }
        false
    }

    /// Remove modules from object literals in expressions
    fn remove_modules_from_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Object(obj) => {
                self.remove_modules_from_object(obj);
            }
            Expr::Paren(paren) => {
                if let Expr::Object(obj) = paren.expr.as_mut() {
                    self.remove_modules_from_object(obj);
                }
            }
            _ => {}
        }
    }

    /// Remove specified modules from an object literal
    fn remove_modules_from_object(&mut self, obj: &mut ObjectLit) {
        obj.props.retain(|prop| !self.should_remove_property(prop));
    }
}

impl VisitMut for WebpackModuleRemover {
    /// Visit variable declarations to find and modify __webpack_modules__
    fn visit_mut_var_decl(&mut self, node: &mut VarDecl) {
        for declarator in &mut node.decls {
            if let Pat::Ident(ident) = &declarator.name {
                if ident.sym == "__webpack_modules__" {
                    if let Some(init) = &mut declarator.init {
                        self.remove_modules_from_expr(init);
                    }
                }
            }
        }
        // Continue visiting children
        node.visit_mut_children_with(self);
    }

    /// Visit assignment expressions to find and modify __webpack_modules__
    fn visit_mut_assign_expr(&mut self, node: &mut swc_ecma_ast::AssignExpr) {
        if let swc_ecma_ast::AssignTarget::Simple(swc_ecma_ast::SimpleAssignTarget::Ident(ident)) = &node.left {
            if ident.sym == "__webpack_modules__" {
                self.remove_modules_from_expr(&mut node.right);
            }
        }
        // Continue visiting children
        node.visit_mut_children_with(self);
    }
}
