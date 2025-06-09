use rustc_hash::FxHashSet;
use swc_core::ecma::ast::{ModuleItem, Expr, Stmt};
use swc_core::{
    common::{BytePos, Span, Spanned},
    ecma::{
        visit::{VisitMut, VisitMutPass, VisitMutWith, visit_mut_pass},
    },
};
use swc_macro_parser::MacroNode;

use crate::{
    directive::{DefineInlineDirective, Directive, IfDirective},
    meta_data::{Metadata, ToSwcAst},
};

mod directive;
pub mod meta_data;
pub mod webpack_module_graph;
pub mod webpack_tree_shaker;
pub mod mutation_tracker;
pub mod feature_analyzer;
pub mod optimization_pipeline;

pub fn condition_transform(
    meta_data: serde_json::Value,
    mut macros: Vec<(BytePos, MacroNode)>,
) -> VisitMutPass<RemoveReplaceTransformer> {
    macros.sort_by_key(|m| m.0);

    // Parse untyped macro nodes to directives
    let mut directives = Vec::new();
    let mut if_stack = Vec::new();
    for (ast_pos, macro_node) in macros {
        match macro_node.directive.as_str() {
            "if" => if_stack.push((
                ast_pos,
                macro_node
                    .attrs
                    .get("condition")
                    .expect("No `condition` attr in if directive")
                    .clone(),
            )),
            "endif" => {
                let (start_pos, condition) = if_stack.pop().expect("Unpaired :if directive");
                directives.push(Directive::If(IfDirective {
                    range: Span::new(start_pos, ast_pos),
                    condition,
                }));
            }
            "define-inline" => directives.push(Directive::DefineInline(DefineInlineDirective {
                pos: ast_pos,
                value: macro_node
                    .attrs
                    .get("value")
                    .expect("No `value` attr in define-inline directive")
                    .clone(),
                default: macro_node.attrs.get("default").cloned(),
            })),
            _ => continue,
        }
    }

    // Evaluate directives and generate an remove/replace list
    let mut remove_list = FxHashSet::default();
    let mut replace_expr_list = Vec::new();
    for directive in directives {
        match directive {
            Directive::If(if_directive) => {
                let condition_result = meta_data.evaluate_bool(&if_directive.condition);
                web_sys::console::log_1(&format!("🎯 Evaluating condition '{}': {}", if_directive.condition, condition_result).into());
                
                if !condition_result {
                    web_sys::console::log_1(&format!("❌ Marking span for removal: {:?} (condition '{}' is false)", if_directive.range, if_directive.condition).into());
                    remove_list.insert(if_directive.range);
                } else {
                    web_sys::console::log_1(&format!("✅ Keeping span: {:?} (condition '{}' is true)", if_directive.range, if_directive.condition).into());
                }
            }
            Directive::DefineInline(define_inline_directive) => {
                let replacement = meta_data
                    .query(&define_inline_directive.value)
                    .map(|value| value.clone().to_ast())
                    .or_else(|| define_inline_directive.default.map(|d| d.to_ast()))
                    .expect("`value` or `default` is invalid");
                replace_expr_list.push((define_inline_directive.pos, replacement));
            }
        }
    }
    
    web_sys::console::log_1(&format!("🔧 Final remove_list has {} spans to remove", remove_list.len()).into());

    visit_mut_pass(RemoveReplaceTransformer {
        remove_list,
        replace_expr_list,
    })
}

/// Remove or replace the ast nodes by traversing the ast.
/// We only focus on three types of ast: `ModuleItem`, `Stmt` and `Expr`, which covers most use cases.
pub struct RemoveReplaceTransformer {
    /// `remove_list` contains a set of ranges.
    /// If a visited ast is in one of the ranges, it will be removed.
    remove_list: FxHashSet<Span>,
    /// `replace_expr_list` contains a position and a replacement.
    /// If the start of an ast node is on the position, it will be replaced.
    replace_expr_list: Vec<(BytePos, Expr)>,
}

impl VisitMut for RemoveReplaceTransformer {
    fn visit_mut_module_item(&mut self, node: &mut ModuleItem) {
        // Check if this node should be removed
        for remove in self.remove_list.iter() {
            if remove.contains(node.span()) {
                // Replace with an empty export statement instead of invalid token
                *node = ModuleItem::Stmt(Stmt::Empty(swc_core::ecma::ast::EmptyStmt {
                    span: swc_core::common::DUMMY_SP,
                }));
                return;
            }
        }

        node.visit_mut_children_with(self);
    }

    fn visit_mut_stmt(&mut self, node: &mut Stmt) {
        // Check if this statement should be removed
        for remove in self.remove_list.iter() {
            if remove.contains(node.span()) {
                // Create an empty statement instead of invalid token
                *node = Stmt::Empty(swc_core::ecma::ast::EmptyStmt {
                    span: swc_core::common::DUMMY_SP,
                });
                return;
            }
        }

        node.visit_mut_children_with(self);
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        // Check if this expression should be replaced first
        for (pos, replacement) in self.replace_expr_list.iter() {
            if node.span_lo() == *pos {
                *node = replacement.clone();
                return;
            }
        }

        // Check if this expression should be removed
        for remove in self.remove_list.iter() {
            if remove.contains(node.span()) {
                // Replace with a null literal instead of invalid token
                *node = Expr::Lit(swc_core::ecma::ast::Lit::Null(swc_core::ecma::ast::Null {
                    span: swc_core::common::DUMMY_SP,
                }));
                return;
            }
        }

        node.visit_mut_children_with(self);
    }
}
