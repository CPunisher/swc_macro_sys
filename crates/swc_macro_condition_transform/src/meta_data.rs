use serde_json::Value;
use swc_core::{
    atoms::Atom,
    common::DUMMY_SP,
    ecma::ast::{
        ArrayLit, Bool, Expr, ExprOrSpread, KeyValueProp, Lit, Null, Number, ObjectLit, Prop,
        PropName, PropOrSpread, Str,
    },
};

pub trait Metadata {
    fn query(&self, path: &str) -> Option<&Value>;
    fn evaluate_bool(&self, path: &str) -> bool;
}

impl Metadata for Value {
    fn query(&self, path: &str) -> Option<&Value> {
        let mut v = Some(self);
        for seg in path.split('.') {
            v = v?.get(seg);
        }
        v
    }

    fn evaluate_bool(&self, path: &str) -> bool {
        let Some(value) = self.query(path) else {
            return false;
        };

        // For simplication, we only evaluate values of bool type.
        // We may evaluate other types like javascript
        if let Some(bool) = value.as_bool() {
            return bool;
        }

        false
    }
}

pub trait ToSwcAst {
    fn to_ast(self) -> Expr;
}

impl ToSwcAst for Value {
    fn to_ast(self) -> Expr {
        match self {
            Value::Null => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
            Value::Bool(b) => Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value: b,
            })),
            Value::Number(number) => Expr::Lit(Lit::Num(Number {
                span: DUMMY_SP,
                value: number.as_f64().unwrap(),
                raw: None,
            })),
            Value::String(s) => Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: Atom::new(s),
                raw: None,
            })),
            Value::Array(values) => Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: values
                    .into_iter()
                    .map(|v| {
                        Some(ExprOrSpread {
                            spread: None,
                            expr: Box::new(v.to_ast()),
                        })
                    })
                    .collect(),
            }),
            Value::Object(map) => Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: map
                    .into_iter()
                    .map(|(k, v)| {
                        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Str(k.into()),
                            value: Box::new(v.to_ast()),
                        })))
                    })
                    .collect(),
            }),
        }
    }
}

impl ToSwcAst for String {
    fn to_ast(self) -> Expr {
        Expr::Lit(Lit::Str(Str {
            span: DUMMY_SP,
            value: Atom::new(self),
            raw: None,
        }))
    }
}
