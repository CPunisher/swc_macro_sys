use std::collections::HashSet;

use swc_core::common::util::take::Take;
use swc_core::ecma::ast::ModuleItem;
use swc_core::{
    common::{BytePos, Span, Spanned},
    ecma::{
        ast::{Expr, Stmt},
        visit::{VisitMut, VisitMutPass, VisitMutWith, visit_mut_pass},
    },
};
use swc_macro_parser::MacroNode;

use crate::{
    directive::{DefineInlineDirective, Directive, IfDirective},
    meta_data::{Metadata, ToSwcAst},
};

mod directive;
mod meta_data;

pub fn condition_transform(
    meta_data: serde_json::Value,
    mut macros: Vec<(BytePos, MacroNode)>,
) -> VisitMutPass<RemoveReplaceTransformer> {
    macros.sort_by_key(|m| m.0);

    // parse untyped macro nodes to directives
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

    // evaluate directives and generate an remove/replace list
    let mut remove_list = HashSet::new();
    let mut replace_expr_list = Vec::new();
    for directive in directives {
        match directive {
            Directive::If(if_directive) => {
                if !meta_data.evaluate_bool(&if_directive.condition) {
                    remove_list.insert(if_directive.range);
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

    visit_mut_pass(RemoveReplaceTransformer {
        remove_list,
        replace_expr_list,
    })
}

pub struct RemoveReplaceTransformer {
    remove_list: HashSet<Span>,
    replace_expr_list: Vec<(BytePos, Expr)>,
}

impl VisitMut for RemoveReplaceTransformer {
    fn visit_mut_module_item(&mut self, node: &mut ModuleItem) {
        for remove in self.remove_list.iter() {
            if remove.contains(node.span()) {
                node.take();
                return;
            }
        }

        node.visit_mut_children_with(self);
    }

    fn visit_mut_stmt(&mut self, node: &mut Stmt) {
        for remove in self.remove_list.iter() {
            if remove.contains(node.span()) {
                node.take();
                return;
            }
        }

        node.visit_mut_children_with(self);
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        for remove in self.remove_list.iter() {
            if remove.contains(node.span()) {
                node.take();
                return;
            }
        }

        for (pos, replacement) in self.replace_expr_list.iter() {
            if node.span_lo() == *pos {
                *node = replacement.clone();
                return;
            }
        }

        node.visit_mut_children_with(self);
    }
}
