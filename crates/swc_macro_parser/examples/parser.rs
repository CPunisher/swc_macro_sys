use std::fs;

use swc_core::{
    common::{FileName, SourceMap, comments::SingleThreadedComments, sync::Lrc},
    ecma::parser::{EsSyntax, Parser, StringInput, Syntax},
};
use swc_macro_parser::MacroParser;

pub fn main() {
    let path = std::env::args().nth(1).unwrap_or("test.js".to_owned());
    let source = fs::read_to_string(path).unwrap();

    let (_program, comments) = {
        let cm: Lrc<SourceMap> = Default::default();
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

    let parser = MacroParser::new("swc");
    let macros = parser.parse(&comments);
    println!("{:?}", macros);
}
