use swc_macro_wasm::optimize;

// Ensure JSONP chunk reachability keeps transitive require target modules
// React index.js requires cjs/react.development.js; that second module must not be pruned.

#[test]
fn jsonp_reachability_keeps_react_development() {
    let source = r#"
        "use strict";
        (self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([
            ["vendors-node_modules_pnpm_react_18_3_1_node_modules_react_index_js"],
            {
                "../../../node_modules/.pnpm/react@18.3.1/node_modules/react/index.js": function(module, exports, __webpack_require__) {
                    if (false) {} else {
                        module.exports = __webpack_require__("../../../node_modules/.pnpm/react@18.3.1/node_modules/react/cjs/react.development.js");
                    }
                },
                "../../../node_modules/.pnpm/react@18.3.1/node_modules/react/cjs/react.development.js": function(module, exports, __webpack_require__) {
                    module.exports = { createElement: function() { return null; } };
                }
            }
        ]);
    "#;

    let config = serde_json::json!({
        "treeShake": {
            "react": {
                "chunk_characteristics": {
                    "entry_module_id": "../../../node_modules/.pnpm/react@18.3.1/node_modules/react/index.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "jsonp",
                    "chunk_loading_type": null,
                    "runtime_names": ["remote", "main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["vendors-node_modules_pnpm_react_18_3_1_node_modules_react_index_js.js"],
                    "is_shared_chunk": true,
                    "shared_modules": []
                }
            }
        }
    });

    let optimized = optimize::optimize(source.to_string(), config).expect("optimize react jsonp chunk");

    // Ensure the development module is still present (not pruned)
    assert!(optimized.contains("react/cjs/react.development.js"), "react.development.js must remain reachable");
    // Ensure JSONP push structure remains
    assert!(optimized.contains(".push(["));
}


