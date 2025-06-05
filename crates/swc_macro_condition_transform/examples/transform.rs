use std::fs;

use serde_json::json;
use swc_common::pass::Repeated;
use swc_core::{
    common::{FileName, Mark, SourceMap, comments::SingleThreadedComments, sync::Lrc},
    ecma::{
        codegen::{
            self, Emitter,
            text_writer::{self, WriteJs},
        },
        parser::{EsSyntax, Parser, StringInput, Syntax},
        visit::VisitMutWith,
    },
};
use swc_ecma_ast::Program;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;

pub fn main() {
    let path = std::env::args().nth(1).unwrap_or("test.js".to_owned());
    let source = fs::read_to_string(path).unwrap();

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
        let mut transformer = condition_transform(
            json!({
                "build": {
                    "target": "production"
                },
                "device": {
                    "isMobile": false
                },
                "user": {
                    "language": "en",
                    "isLoggedIn": true
                },
                "experiment": {
                    "group": "B"
                },
                "featureFlags": {
                    "newMobileUI": true,
                    "enableNewFeature": false,
                    "newUserProfile": false
                }
            }),
            macros,
        );
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();
            let mut program = program.apply(resolver(unresolved_mark, top_level_mark, false));

            perform_dce(&mut program, unresolved_mark);

            program
        })
    };

    let ret = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default(),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        emitter.emit_program(&program).unwrap();
        drop(emitter);

        unsafe { String::from_utf8_unchecked(buf) }
    };

    println!("{}", ret);
}

fn perform_dce(m: &mut Program, unresolved_mark: Mark) {
    let mut visitor = swc_ecma_transforms_optimization::simplify::dce::dce(
        swc_ecma_transforms_optimization::simplify::dce::Config {
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
