use swc_macro_wasm::optimize;

// This test simulates a vendor chunk module (react-chartjs-2) that uses
// macro-annotated export mappings and a conditional internal require.
// We assert that:
// - Macro processing removes disabled exports
// - The conditional require is eliminated when disabled
// - The orphaned helper module is pruned from exports.modules by the TreeShaker

#[test]
fn react_chartjs2_macro_removes_disabled_export_and_prunes_helper() {
    let source = r#"
        "use strict";
        exports.ids = ["vendors-react-chartjs-2"];
        exports.modules = {
          "react-chartjs-2/index.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
              Bar: () => (/* @common:if [condition="treeShake.react-chartjs-2.Bar"] */ Bar /* @common:endif */),
              Line: () => (/* @common:if [condition="treeShake.react-chartjs-2.Line"] */ Line /* @common:endif */)
            });

            /* @common:if [condition="treeShake.react-chartjs-2.Line"] */
            var _helper = __webpack_require__("./helper.js");
            /* @common:endif */

            function Bar() { return 1; }
            function Line() { return _helper.x(); }
          },

          "./helper.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, { x: () => x });
            function x() { return 42; }
          }
        };
    "#;

    let config = serde_json::json!({
        "treeShake": {
            "react-chartjs-2": {
                "Bar": true,
                "Line": false,
                "chunk_characteristics": {
                    "entry_module_id": "react-chartjs-2/index.js",
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
                    "chunk_files": ["vendors-react-chartjs-2.js"],
                    "is_shared_chunk": true,
                    "shared_modules": []
                }
            }
        }
    });

    let optimized = optimize::optimize(source.to_string(), config).unwrap();

    // Enabled implementation remains
    assert!(optimized.contains("function Bar()"), "Enabled export impl 'Bar' should remain");

    // Conditional require removed
    assert!(!optimized.contains("__webpack_require__(\"./helper.js\")"), "Conditional require to helper should be removed");

    // Orphan helper module pruned from exports.modules
    assert!(!optimized.contains("\"./helper.js\":"), "Helper module should be pruned from exports.modules");
}

#[test]
fn react_chartjs2_macro_keeps_line_and_helper_when_enabled() {
    let source = r#"
        "use strict";
        exports.ids = ["vendors-react-chartjs-2"];
        exports.modules = {
          "react-chartjs-2/index.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
              Bar: () => (/* @common:if [condition="treeShake.react-chartjs-2.Bar"] */ Bar /* @common:endif */),
              Line: () => (/* @common:if [condition="treeShake.react-chartjs-2.Line"] */ Line /* @common:endif */)
            });

            /* @common:if [condition="treeShake.react-chartjs-2.Line"] */
            var _helper = __webpack_require__("./helper.js");
            /* @common:endif */

            function Bar() { return 1; }
            function Line() { return _helper.x(); }
          },

          "./helper.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, { x: () => x });
            function x() { return 42; }
          }
        };
    "#;

    let config = serde_json::json!({
        "treeShake": {
            "react-chartjs-2": {
                "Bar": false,
                "Line": true,
                "chunk_characteristics": {
                    "entry_module_id": "react-chartjs-2/index.js",
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
                    "chunk_files": ["vendors-react-chartjs-2.js"],
                    "is_shared_chunk": true,
                    "shared_modules": []
                }
            }
        }
    });

    let optimized = optimize::optimize(source.to_string(), config).unwrap();

    // Enabled implementation remains
    assert!(optimized.contains("function Line()"), "Enabled export impl 'Line' should remain");

    // Conditional require retained
    assert!(optimized.contains("__webpack_require__(\"./helper.js\")"), "Conditional require to helper should remain");

    // Helper module must not be pruned in this scenario
    assert!(optimized.contains("\"./helper.js\":"), "Helper module should remain in exports.modules");
}


