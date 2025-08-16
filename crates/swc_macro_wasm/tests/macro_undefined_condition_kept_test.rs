use swc_macro_wasm::optimize;

// If a macro condition exists in the source but is not present in the config metadata,
// the conservative behavior is to KEEP the code (treat missing condition as true).

#[test]
fn macro_if_undefined_condition_keeps_code() {
    let source = r#"
        exports.modules = {
          "pkg/index.js": function(module, exports, __webpack_require__) {
            /* @common:if [condition="treeShake.pkg.feature"] */
            function Feature() { return 1; }
            /* @common:endif */

            exports.Feature = Feature;
          }
        };
    "#;

    // No treeShake.pkg.feature provided in config
    let config = serde_json::json!({
        "treeShake": {
            "pkg": {
                // intentionally empty: no 'feature' flag
                "chunk_characteristics": {
                    "entry_module_id": "pkg/index.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "require",
                    "chunk_loading_type": null,
                    "runtime_names": [],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["vendors-pkg.js"],
                    "is_shared_chunk": true,
                    "shared_modules": []
                }
            }
        }
    });

    let optimized = optimize::optimize(source.to_string(), config).unwrap();

    // The Feature implementation should be kept because the condition is undefined
    assert!(optimized.contains("function Feature()"), "Feature implementation should be retained when condition is undefined");
}


