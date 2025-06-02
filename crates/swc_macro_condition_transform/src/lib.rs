use std::collections::HashMap;

use swc_core::{
    common::{BytePos, Spanned},
    ecma::{
        ast::Expr,
        visit::{VisitMut, VisitMutPass, VisitMutWith, visit_mut_pass},
    },
};
use swc_macro_parser::MacroNode;

use crate::meta_data::{Metadata, ToSwcAst};

pub mod meta_data;

pub fn condition_transform(
    meta_data: serde_json::Value,
    macros: HashMap<BytePos, Vec<MacroNode>>,
) -> VisitMutPass<ConditionTransform> {
    visit_mut_pass(ConditionTransform { meta_data, macros })
}

pub struct ConditionTransform {
    meta_data: serde_json::Value,
    macros: HashMap<BytePos, Vec<MacroNode>>,
}

impl ConditionTransform {}

impl VisitMut for ConditionTransform {
    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);
        let Some(macros) = self.macros.get(&node.span_lo()) else {
            return;
        };

        for m in macros {
            if m.directive == "define-inline" {
                let queried = m
                    .attrs
                    .get("value")
                    .and_then(|key| self.meta_data.query(key))
                    .map(|value| value.clone().to_ast())
                    .or_else(|| m.attrs.get("default").map(|d| d.clone().to_ast()))
                    .expect("`value` or `default` is invalid");

                *node = queried;
            }
        }
    }
}
