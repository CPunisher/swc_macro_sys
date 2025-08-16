use std::collections::HashMap;

#[test]
fn test_dependency_graph_basic_extraction() {
    let content = r#"
(self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["test"], {
  "main.js": function(module, exports, __webpack_require__) {
    __webpack_require__("a.js");
  },
  "a.js": function(module, exports, __webpack_require__) {
    __webpack_require__("c.js");
  },
  "b.js": function(module, exports, __webpack_require__) {},
  "c.js": function(module, exports, __webpack_require__) {}
}]);
"#;

    let graph_json = swc_macro_wasm::get_webpack_dependency_graph(content);

    let graph: HashMap<String, Vec<String>> = serde_json::from_str(&graph_json)
        .expect(&format!("graph JSON should parse, got: {}", graph_json));

    // Verify expected nodes exist
    assert!(graph.contains_key("main.js"));
    assert!(graph.contains_key("a.js"));
    assert!(graph.contains_key("b.js"));
    assert!(graph.contains_key("c.js"));

    // Verify adjacency
    assert!(graph.get("main.js").unwrap().iter().any(|d| d == "a.js"));
    assert!(graph.get("a.js").unwrap().iter().any(|d| d == "c.js"));

    // Leaf nodes should have empty dependency lists
    assert!(graph.get("b.js").unwrap().is_empty());
    assert!(graph.get("c.js").unwrap().is_empty());
}

#[test]
fn test_dependency_graph_invalid_chunk_error_json() {
    let content = "console.log('hello');";
    let graph_json = swc_macro_wasm::get_webpack_dependency_graph(content);
    let v: serde_json::Value = serde_json::from_str(&graph_json).expect("should be JSON");
    assert!(v.get("error").is_some(), "expected error JSON for invalid chunk, got: {}", graph_json);
}