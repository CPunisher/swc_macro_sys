#[cfg(test)]
mod full_optimization_flow_tests {
    use serde_json::json;
    use swc_macro_wasm::optimize_with_prune_result;

    fn create_webpack_chunk_with_macros() -> String {
        r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["test-chunk"],
    {
        "entry": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__),
                "getData": () => (getData),
                /* common:if exports.processData */ "processData": () => (processData),
                /* common:if exports.transform */ "transform": () => (transform)
            });
            
            // This module requires data-processor which should be pruned if processData is removed
            /* common:if exports.processData */ var _dataProcessor = __webpack_require__("data-processor");
            
            // This module requires transformer which should be pruned if transform is removed  
            /* common:if exports.transform */ var _transformer = __webpack_require__("transformer");
            
            // This module is always required
            var _utils = __webpack_require__("utils");
            
            function getData() {
                return _utils.fetchData();
            }
            
            /* common:if exports.processData */
            function processData(data) {
                return _dataProcessor.process(data);
            }
            
            /* common:if exports.transform */
            function transform(input) {
                return _transformer.transform(input);
            }
            
            const __WEBPACK_DEFAULT_EXPORT__ = {
                getData,
                /* common:if exports.processData */ processData,
                /* common:if exports.transform */ transform
            };
        },
        
        "data-processor": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "process": () => (process)
            });
            
            // This module requires heavy-lib which should also be pruned
            var _heavyLib = __webpack_require__("heavy-lib");
            
            function process(data) {
                return _heavyLib.doHeavyProcessing(data);
            }
        },
        
        "transformer": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "transform": () => (transform)
            });
            
            // This module requires parser which should also be pruned
            var _parser = __webpack_require__("parser");
            
            function transform(input) {
                return _parser.parse(input);
            }
        },
        
        "utils": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "fetchData": () => (fetchData)
            });
            
            function fetchData() {
                return { data: "test" };
            }
        },
        
        "heavy-lib": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "doHeavyProcessing": () => (doHeavyProcessing)
            });
            
            function doHeavyProcessing(data) {
                // Simulated heavy processing
                return data;
            }
        },
        
        "parser": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "parse": () => (parse)
            });
            
            function parse(input) {
                return JSON.parse(input);
            }
        }
    }
]);
"#.to_string()
    }

    #[test]
    fn test_full_optimization_removes_macros_and_prunes_modules() {
        let source = create_webpack_chunk_with_macros();
        
        // Config that only keeps getData, removing processData and transform
        let config = json!({
            "treeShake": {
                "test-lib": {
                    "exports": {
                        "getData": true,
                        "processData": false,
                        "transform": false
                    },
                    "chunk_characteristics": {
                        "entry_module_id": "entry"
                    }
                }
            }
        });

        let (optimized_source, prune_result) = optimize_with_prune_result(source.clone(), config);
        
        // Verify macro comments are removed
        assert!(!optimized_source.contains(&"common:if".to_string()), 
            "Macro comments should be removed from optimized source");
        
        // Verify pruning occurred
        assert!(prune_result.pruned_count > 0, 
            "Should have pruned modules");
        
        // Verify specific modules were pruned
        assert!(prune_result.removed_modules.contains(&"data-processor".to_string()),
            "data-processor should be pruned when processData is removed");
        assert!(prune_result.removed_modules.contains(&"heavy-lib".to_string()),
            "heavy-lib should be pruned as dependency of data-processor");
        assert!(prune_result.removed_modules.contains(&"transformer".to_string()),
            "transformer should be pruned when transform is removed");
        assert!(prune_result.removed_modules.contains(&"parser".to_string()),
            "parser should be pruned as dependency of transformer");
        
        // Verify essential modules are kept
        assert!(prune_result.kept_modules.contains(&"entry".to_string()),
            "entry module should be kept");
        assert!(prune_result.kept_modules.contains(&"utils".to_string()),
            "utils should be kept as it's used by getData");
    }

    #[test]
    fn test_optimization_with_all_exports_enabled() {
        let source = create_webpack_chunk_with_macros();
        
        // Config that keeps all exports
        let config = json!({
            "treeShake": {
                "test-lib": {
                    "exports": {
                        "getData": true,
                        "processData": true,
                        "transform": true
                    },
                    "chunk_characteristics": {
                        "entry_module_id": "entry"
                    }
                }
            }
        });

        let (optimized_source, prune_result) = optimize_with_prune_result(source.clone(), config);
        
        // Verify macro comments are still removed
        assert!(!optimized_source.contains(&"common:if".to_string()), 
            "Macro comments should be removed even when all exports are kept");
        
        // Verify no pruning occurred (all modules reachable)
        assert_eq!(prune_result.pruned_count, 0, 
            "Should not prune any modules when all exports are enabled");
        
        // Verify all modules are kept
        assert!(prune_result.kept_modules.contains(&"entry".to_string()));
        assert!(prune_result.kept_modules.contains(&"utils".to_string()));
        assert!(prune_result.kept_modules.contains(&"data-processor".to_string()));
        assert!(prune_result.kept_modules.contains(&"heavy-lib".to_string()));
        assert!(prune_result.kept_modules.contains(&"transformer".to_string()));
        assert!(prune_result.kept_modules.contains(&"parser".to_string()));
    }

    #[test]
    fn test_optimization_removes_only_unreachable_chains() {
        let source = create_webpack_chunk_with_macros();
        
        // Config that keeps processData but not transform
        let config = json!({
            "treeShake": {
                "test-lib": {
                    "exports": {
                        "getData": true,
                        "processData": true,
                        "transform": false
                    },
                    "chunk_characteristics": {
                        "entry_module_id": "entry"
                    }
                }
            }
        });

        let (optimized_source, prune_result) = optimize_with_prune_result(source.clone(), config);
        
        // Verify macro comments are removed
        assert!(!optimized_source.contains(&"common:if".to_string()));
        
        // Verify only transform chain is pruned
        assert!(prune_result.removed_modules.contains(&"transformer".to_string()),
            "transformer should be pruned");
        assert!(prune_result.removed_modules.contains(&"parser".to_string()),
            "parser should be pruned");
        
        // Verify processData chain is kept
        assert!(prune_result.kept_modules.contains(&"data-processor".to_string()),
            "data-processor should be kept");
        assert!(prune_result.kept_modules.contains(&"heavy-lib".to_string()),
            "heavy-lib should be kept");
    }

    #[test]
    fn test_macro_comment_removal_in_complex_scenarios() {
        let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["test"],
    {
        "complex": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                /* common:if exports.feature1 */ "feature1": () => (feature1),
                /* common:if exports.feature2 */ "feature2": () => (feature2),
                "base": () => (base)
            });
            
            // Nested macro comments
            /* common:if exports.feature1 */
            function feature1() {
                /* common:if exports.feature1 */
                const config = {
                    /* common:if exports.feature1 */ enabled: true
                };
                return config;
            }
            
            // Multiple conditions on same line
            /* common:if exports.feature2 */ function feature2() { /* common:if exports.feature2 */ return "feature2"; }
            
            function base() {
                return "base";
            }
        }
    }
]);
"#.to_string();

        let config = json!({
            "treeShake": {
                "test-lib": {
                    "exports": {
                        "feature1": false,
                        "feature2": true,
                        "base": true
                    },
                    "chunk_characteristics": {
                        "entry_module_id": "complex"
                    }
                }
            }
        });

        let (optimized_source, _prune_result) = optimize_with_prune_result(source, config);
        
        // Verify ALL macro comments are removed
        assert!(!optimized_source.contains(&"common:if".to_string()),
            "All macro comments should be removed");
        
        // Verify feature1 code is removed but feature2 and base are kept
        assert!(!optimized_source.contains(&"function feature1".to_string()),
            "feature1 function should be removed");
        assert!(optimized_source.contains(&"function feature2".to_string()),
            "feature2 function should be kept");
        assert!(optimized_source.contains(&"function base".to_string()),
            "base function should be kept");
    }

    #[test]
    fn test_optimization_handles_circular_dependencies() {
        let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["circular"],
    {
        "moduleA": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                /* common:if exports.funcA */ "funcA": () => (funcA)
            });
            
            /* common:if exports.funcA */ var _moduleB = __webpack_require__("moduleB");
            
            /* common:if exports.funcA */
            function funcA() {
                return _moduleB.funcB();
            }
        },
        
        "moduleB": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "funcB": () => (funcB)
            });
            
            var _moduleA = __webpack_require__("moduleA");
            
            function funcB() {
                // Circular reference back to moduleA
                return "B";
            }
        }
    }
]);
"#.to_string();

        let config = json!({
            "treeShake": {
                "test-lib": {
                    "exports": {
                        "funcA": false
                    },
                    "chunk_characteristics": {
                        "entry_module_id": "moduleA"
                    }
                }
            }
        });

        let (optimized_source, prune_result) = optimize_with_prune_result(source, config);
        
        // Verify macro comments are removed
        assert!(!optimized_source.contains(&"common:if".to_string()));
        
        // Since funcA is removed and moduleA is entry, moduleB becomes unreachable
        assert!(prune_result.removed_modules.contains(&"moduleB".to_string()),
            "moduleB should be pruned when funcA is removed");
    }
}