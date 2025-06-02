use swc_core::common::{BytePos, Span};

#[derive(Debug)]
pub enum Directive {
    If(IfDirective),
    DefineInline(DefineInlineDirective),
}

#[derive(Debug)]
pub struct IfDirective {
    pub range: Span,
    pub condition: String,
}

#[derive(Debug)]
pub struct DefineInlineDirective {
    pub pos: BytePos,
    pub value: String,
    pub default: Option<String>,
}
