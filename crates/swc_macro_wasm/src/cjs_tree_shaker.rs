use crate::import_map::ImportMap;
use rustc_hash::FxHashSet;
use swc_atoms::Atom;
use swc_common::{
    Spanned,
    comments::{Comments, SingleThreadedComments},
};
use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

pub fn tree_shake_cjs(
    program: &mut Program,
    comments: &SingleThreadedComments,
    dropped_exports: &FxHashSet<Atom>,
) {
    let imports = ImportMap::analyze(&*program);
    let mut visitor = CjsTreeShakerVisitor {
        dropped_exports,
        imports: &imports,
        comments,
    };
    program.visit_mut_with(&mut visitor);
}

struct CjsTreeShakerVisitor<'a> {
    dropped_exports: &'a FxHashSet<Atom>,
    imports: &'a ImportMap,
    comments: &'a SingleThreadedComments,
}

impl<'a> VisitMut for CjsTreeShakerVisitor<'a> {
    fn visit_mut_stmt(&mut self, s: &mut Stmt) {
        if let Stmt::Expr(e) = s {
            // Drop Object.defineProperty on exports
            if let Some(property_name) = find_define_property_on_exports(&e.expr) {
                if self.dropped_exports.contains(&property_name) {
                    *s = Stmt::Empty(EmptyStmt { span: s.span() });
                    return;
                }
            }
        }

        s.visit_mut_children_with(self);
    }

    fn visit_mut_var_declarator(&mut self, v: &mut VarDeclarator) {
        v.visit_mut_children_with(self);

        if let Some(init) = &v.init {
            // This is a require, and it's very likely that it has no side effect.
            if self.imports.get_require_path(init).is_some() {
                let loc = init.span().lo;
                self.comments.add_pure_comment(loc);
            }
        }
    }
}

fn find_define_property_on_exports(expr: &Expr) -> Option<Atom> {
    match expr {
        Expr::Call(CallExpr {
            callee:
                Callee::Expr(box Expr::Member(MemberExpr {
                    obj: box Expr::Ident(object),
                    prop: MemberProp::Ident(prop),
                    ..
                })),
            args,
            ..
        }) => {
            if object.sym == "Object" && prop.sym == "defineProperty" {
                // (exports, property_name, definition)

                if args.len() < 2 {
                    return None;
                }

                if let Expr::Ident(ident) = &*args[0].expr {
                    if ident.sym == "exports" {
                        if let Expr::Lit(Lit::Str(property_name)) = &*args[1].expr {
                            return Some(property_name.value.clone());
                        }
                    }
                }
            }
        }
        _ => {}
    }

    None
}
