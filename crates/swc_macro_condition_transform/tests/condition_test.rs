use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use swc_common::comments::SingleThreadedComments;
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_codegen::{Emitter, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_core::ecma::visit::VisitMutWith;
use swc_ecma_codegen::text_writer::WriteJs;

#[test]
fn test_export_removal_with_false_condition() {
    let source = r#"
    __webpack_require__.d(__webpack_exports__, {
        add: () => (/* @common:if [condition="treeShake.lodash-es.add"] */ _add_js__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
        delay: () => (/* @common:if [condition="treeShake.lodash-es.delay"] */ _delay_js__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */)
    });
    "#;

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source.to_string());
    let comments = SingleThreadedComments::default();

    let mut program = Parser::new(
        Syntax::Es(EsSyntax::default()),
        StringInput::from(&*fm),
        Some(&comments),
    )
    .parse_program()
    .unwrap();

    // Parse macros
    let macros = MacroParser::new("common").parse(&comments);

    // Create config where add is false and delay is true
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "add": false,
                "delay": true
            }
        }
    });

    // Apply transform
    let mut transformer = condition_transform(config, macros);
    program.visit_mut_with(&mut transformer);

    // Emit result
    let result = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None)) 
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config::default().with_minify(false),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        emitter.emit_program(&program).unwrap();
        drop(emitter);
        String::from_utf8(buf).unwrap()
    };

    println!("Result:\n{}", result);

    // Check that add export should have null
    assert!(result.contains("add: ()=>null"), "add export should be null but was not. Result: {}", result);
    
    // Check that delay export should still have the module reference
    assert!(result.contains("delay: ()=>_delay_js__WEBPACK_IMPORTED_MODULE_1__"), 
        "delay export should have module reference. Result: {}", result);
}