use webpack_analyzer_v2::{WebpackAnalyzer, ChunkCharacteristics};

fn make_jsonp_chunk(entry_code: &str) -> String {
    format!(
        r#"
        "use strict";
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([
            ["vendors-entry"],
            {{
                "entry.js": function(module, exports, __webpack_require__) {{
                    {entry}
                }},
                "a.js": function(module, exports, __webpack_require__) {{}}
            }}
        ]);
        "#,
        entry = entry_code
    )
}

fn jsonp_chars() -> ChunkCharacteristics {
    serde_json::from_value(serde_json::json!({
        "entry_module_id": "entry.js",
        "is_runtime_chunk": false,
        "has_runtime": false,
        "is_entrypoint": false,
        "can_be_initial": false,
        "is_only_initial": false,
        "chunk_format": "jsonp",
        "chunk_loading_type": null,
        "runtime_names": [],
        "entry_name": null,
        "has_async_chunks": false,
        "chunk_files": ["vendors-entry.js"],
        "is_shared_chunk": true,
        "shared_modules": []
    })).unwrap()
}

#[test]
fn detect_plain_require_in_jsonp() {
    let src = make_jsonp_chunk("__webpack_require__(\"a.js\");");
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(&src, jsonp_chars()).unwrap();
    let deps = chunk.modules.get(&"entry.js".into()).unwrap().get_dependencies();
    assert!(deps.iter().any(|d| d.as_ref() == "a.js"));
}

#[test]
fn detect_paren_wrapped_callee() {
    let src = make_jsonp_chunk("(__webpack_require__)(\"a.js\");");
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(&src, jsonp_chars()).unwrap();
    let deps = chunk.modules.get(&"entry.js".into()).unwrap().get_dependencies();
    assert!(deps.iter().any(|d| d.as_ref() == "a.js"));
}

#[test]
fn detect_sequence_callee() {
    let src = make_jsonp_chunk("(0, __webpack_require__)(\"a.js\");");
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(&src, jsonp_chars()).unwrap();
    let deps = chunk.modules.get(&"entry.js".into()).unwrap().get_dependencies();
    assert!(deps.iter().any(|d| d.as_ref() == "a.js"));
}

#[test]
fn detect_template_literal_no_expr() {
    let src = make_jsonp_chunk("__webpack_require__(`a.js`);");
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(&src, jsonp_chars()).unwrap();
    let deps = chunk.modules.get(&"entry.js".into()).unwrap().get_dependencies();
    assert!(deps.iter().any(|d| d.as_ref() == "a.js"));
}

#[test]
fn detect_require_inside_if_and_return() {
    let code = r#"
        if (true) { __webpack_require__("a.js"); }
        return __webpack_require__("a.js"), void 0;
    "#;
    let src = make_jsonp_chunk(code);
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(&src, jsonp_chars()).unwrap();
    let deps = chunk.modules.get(&"entry.js".into()).unwrap().get_dependencies();
    assert!(deps.iter().filter(|d| d.as_ref() == "a.js").count() >= 1);
}


