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
fn test_antd_icons_chunk_optimization() {
    let source = r#"
__webpack_require__.d(__webpack_exports__, {
  AccountBookFilled: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookFilled"] */ _AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
  AccountBookOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookOutlined"] */ _AccountBookOutlined__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
  UserOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */ _UserOutlined__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */),
  DeleteOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.DeleteOutlined"] */ _DeleteOutlined__WEBPACK_IMPORTED_MODULE_3__["default"] /* @common:endif */)
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

    // Create config where only UserOutlined and DeleteOutlined are kept
    let config = serde_json::json!({
        "treeShake": {
            "@ant-design/icons": {
                "AccountBookFilled": false,
                "AccountBookOutlined": false,
                "UserOutlined": true,
                "DeleteOutlined": true
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

    println!("Transformed output:\n{}", result);

    // Test macro evaluation results
    // False conditions should be replaced with null
    assert!(result.contains("AccountBookFilled: ()=>(null)") || result.contains("AccountBookFilled: ()=>null"), 
            "AccountBookFilled should be null when condition is false. Result: {}", result);
    assert!(result.contains("AccountBookOutlined: ()=>(null)") || result.contains("AccountBookOutlined: ()=>null"), 
            "AccountBookOutlined should be null when condition is false. Result: {}", result);
    
    // True conditions should keep the original content
    assert!(result.contains("UserOutlined: ()=>(_UserOutlined__WEBPACK_IMPORTED_MODULE_2__") || result.contains("UserOutlined: ()=>_UserOutlined__WEBPACK_IMPORTED_MODULE_2__"), 
            "UserOutlined should keep module reference when condition is true. Result: {}", result);
    assert!(result.contains("DeleteOutlined: ()=>(_DeleteOutlined__WEBPACK_IMPORTED_MODULE_3__") || result.contains("DeleteOutlined: ()=>_DeleteOutlined__WEBPACK_IMPORTED_MODULE_3__"), 
            "DeleteOutlined should keep module reference when condition is true. Result: {}", result);
}

#[test]
fn test_macro_if_endif_removal() {
    let source = r#"
function test() {
    /* @common:if [condition="treeShake.@ant-design/icons.AccountBookFilled"] */
    return _AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__["default"];
    /* @common:endif */
}
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

    let config = serde_json::json!({
        "treeShake": {
            "@ant-design/icons": {
                "AccountBookFilled": false
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

    println!("Transformed output:\n{}", result);

    // When condition is false, the content should be replaced with empty statements
    // The return statement should be gone
    assert!(!result.contains("_AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__"), 
            "Content between false condition should be removed. Result: {}", result);
    assert!(!result.contains("@common:if"), "Macro comments should be removed");
    assert!(!result.contains("@common:endif"), "Macro comments should be removed");
}

#[test]
fn test_macro_true_condition_preservation() {
    let source = r#"
export const test = () => (/* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */ _UserOutlined__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */);
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

    let config = serde_json::json!({
        "treeShake": {
            "@ant-design/icons": {
                "UserOutlined": true
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

    println!("Transformed output:\n{}", result);

    // When condition is true, the content should be preserved but macro comments removed
    assert!(result.contains("_UserOutlined__WEBPACK_IMPORTED_MODULE_0__"), 
            "Content between true condition should be preserved. Result: {}", result);
    assert!(!result.contains("@common:if"), "Macro comments should be removed");
    assert!(!result.contains("@common:endif"), "Macro comments should be removed");
}