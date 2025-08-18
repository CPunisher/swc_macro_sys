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
fn test_undefined_conditions_default_to_true() {
    let source = r#"
    __webpack_require__.d(__webpack_exports__, {
        ExplicitFalse: () => (/* @common:if [condition="treeShake.@ant-design/icons.ExplicitFalse"] */ _ExplicitFalse__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
        ExplicitTrue: () => (/* @common:if [condition="treeShake.@ant-design/icons.ExplicitTrue"] */ _ExplicitTrue__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
        UndefinedIcon: () => (/* @common:if [condition="treeShake.@ant-design/icons.UndefinedIcon"] */ _UndefinedIcon__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */),
        AnotherUndefined: () => (/* @common:if [condition="treeShake.@ant-design/icons.AnotherUndefined"] */ _AnotherUndefined__WEBPACK_IMPORTED_MODULE_3__["default"] /* @common:endif */)
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

    // Config that only defines some icons (NOT UndefinedIcon or AnotherUndefined)
    let config = serde_json::json!({
        "treeShake": {
            "@ant-design/icons": {
                "ExplicitFalse": false,
                "ExplicitTrue": true
                // UndefinedIcon and AnotherUndefined are intentionally NOT defined
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

    // Test results
    // Explicit false should become null
    assert!(result.contains("ExplicitFalse: ()=>(null)") || result.contains("ExplicitFalse: ()=>null"), 
            "ExplicitFalse should be null when condition is false. Result: {}", result);
    
    // Explicit true should keep the module reference
    assert!(result.contains("ExplicitTrue: ()=>(_ExplicitTrue__WEBPACK_IMPORTED_MODULE_1__") || 
            result.contains("ExplicitTrue: ()=>_ExplicitTrue__WEBPACK_IMPORTED_MODULE_1__"), 
            "ExplicitTrue should keep module reference when condition is true. Result: {}", result);
    
    // CRITICAL: Undefined conditions should keep the module reference (default to true for safety)
    assert!(result.contains("UndefinedIcon: ()=>(_UndefinedIcon__WEBPACK_IMPORTED_MODULE_2__") || 
            result.contains("UndefinedIcon: ()=>_UndefinedIcon__WEBPACK_IMPORTED_MODULE_2__"), 
            "UndefinedIcon should keep module reference when condition is undefined (safe default). Result: {}", result);
    
    assert!(result.contains("AnotherUndefined: ()=>(_AnotherUndefined__WEBPACK_IMPORTED_MODULE_3__") || 
            result.contains("AnotherUndefined: ()=>_AnotherUndefined__WEBPACK_IMPORTED_MODULE_3__"), 
            "AnotherUndefined should keep module reference when condition is undefined (safe default). Result: {}", result);

    // Verify undefined conditions do NOT become null
    assert!(!result.contains("UndefinedIcon: ()=>(null)") && !result.contains("UndefinedIcon: ()=>null"), 
            "UndefinedIcon should NOT be null - undefined conditions must default to true for safety");
    assert!(!result.contains("AnotherUndefined: ()=>(null)") && !result.contains("AnotherUndefined: ()=>null"), 
            "AnotherUndefined should NOT be null - undefined conditions must default to true for safety");
}

#[test]
fn test_completely_missing_library_config() {
    let source = r#"
    __webpack_require__.d(__webpack_exports__, {
        SomeIcon: () => (/* @common:if [condition="treeShake.@missing-library/icons.SomeIcon"] */ _SomeIcon__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
        AnotherIcon: () => (/* @common:if [condition="treeShake.@missing-library/icons.AnotherIcon"] */ _AnotherIcon__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */)
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

    // Config that defines a different library (NOT @missing-library/icons)
    let config = serde_json::json!({
        "treeShake": {
            "@ant-design/icons": {
                "SomeOtherIcon": false
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

    println!("Transformed output (missing library config):\n{}", result);

    // When the entire library is missing from config, all conditions should default to true (safe)
    assert!(result.contains("SomeIcon: ()=>(_SomeIcon__WEBPACK_IMPORTED_MODULE_0__") || 
            result.contains("SomeIcon: ()=>_SomeIcon__WEBPACK_IMPORTED_MODULE_0__"), 
            "SomeIcon should keep module reference when library is not in config (safe default). Result: {}", result);
    
    assert!(result.contains("AnotherIcon: ()=>(_AnotherIcon__WEBPACK_IMPORTED_MODULE_1__") || 
            result.contains("AnotherIcon: ()=>_AnotherIcon__WEBPACK_IMPORTED_MODULE_1__"), 
            "AnotherIcon should keep module reference when library is not in config (safe default). Result: {}", result);

    // Verify they do NOT become null
    assert!(!result.contains("SomeIcon: ()=>(null)") && !result.contains("SomeIcon: ()=>null"), 
            "SomeIcon should NOT be null when library config is missing");
    assert!(!result.contains("AnotherIcon: ()=>(null)") && !result.contains("AnotherIcon: ()=>null"), 
            "AnotherIcon should NOT be null when library config is missing");
}

#[test]
fn test_empty_treeshake_config() {
    let source = r#"
    __webpack_require__.d(__webpack_exports__, {
        TestIcon: () => (/* @common:if [condition="treeShake.@ant-design/icons.TestIcon"] */ _TestIcon__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */)
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

    // Empty config
    let config = serde_json::json!({});

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

    println!("Transformed output (empty config):\n{}", result);

    // With completely empty config, all conditions should default to true (safe)
    assert!(result.contains("TestIcon: ()=>(_TestIcon__WEBPACK_IMPORTED_MODULE_0__") || 
            result.contains("TestIcon: ()=>_TestIcon__WEBPACK_IMPORTED_MODULE_0__"), 
            "TestIcon should keep module reference with empty config (safe default). Result: {}", result);

    // Verify it does NOT become null
    assert!(!result.contains("TestIcon: ()=>(null)") && !result.contains("TestIcon: ()=>null"), 
            "TestIcon should NOT be null with empty config - must default to safe behavior");
}