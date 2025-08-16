use serde_json::json;
use swc_macro_wasm::optimize;

#[test]
fn react_redux_preserves_internal_dependency() {
    // Minimal CommonJS-style chunk with exports.modules and an internal dep used by react-redux
    let source = r#"
"use strict";
exports.modules = {
  "react-redux": function(module, exports, __webpack_require__) {
    var withSelector = __webpack_require__("use-sync-external-store/with-selector.js");
    exports.useSelector = function() { return withSelector; };
    exports.useDispatch = function() {};
    exports.Provider = function() {};
  },
  "use-sync-external-store/with-selector.js": function(module, exports) {
    exports.default = function(){};
  }
};
"#;

    // Keep primary exports on react-redux; ensure internal module remains after optimization
    let config = json!({
        "treeShake": {
            "react-redux": {
                "useSelector": true,
                "useDispatch": true,
                "Provider": true,
                "chunk_characteristics": {
                    "entry_module_id": "react-redux",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "require",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["vendors-react-redux.js"],
                    "is_shared_chunk": true,
                    "shared_modules": []
                }
            }
        }
    });

    let optimized = optimize::optimize(source.to_string(), config.clone())
        .expect("optimize should succeed");

    // The internal dependency module mapping must still be present
    assert!(optimized.contains("use-sync-external-store/with-selector.js"),
        "react-redux internal dep should not be pruned");

    // And the react-redux module must remain
    assert!(optimized.contains("react-redux"), "react-redux module should remain");
}


