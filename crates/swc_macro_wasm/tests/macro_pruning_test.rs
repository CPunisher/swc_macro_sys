#[cfg(test)]
mod macro_pruning_tests {
    use serde_json::json;
    use swc_macro_wasm::optimize_with_prune_result;

    /// Create a webpack chunk with proper @common:if macro syntax
    fn create_chunk_with_proper_macros() -> String {
        r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["test-chunk"],
    {
        "entry": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__),
                "getData": () => (/* @common:if [condition="treeShake.test-lib.getData"] */ getData /* @common:endif */),
                "processData": () => (/* @common:if [condition="treeShake.test-lib.processData"] */ processData /* @common:endif */),
                "transformData": () => (/* @common:if [condition="treeShake.test-lib.transformData"] */ transformData /* @common:endif */)
            });
            
            // Always required modules
            var _utils = __webpack_require__("utils");
            var _constants = __webpack_require__("constants");
            
            // Conditionally required based on exports
            /* @common:if [condition="treeShake.test-lib.processData"] */
            var _processor = __webpack_require__("processor");
            /* @common:endif */
            
            /* @common:if [condition="treeShake.test-lib.transformData"] */
            var _transformer = __webpack_require__("transformer");
            /* @common:endif */
            
            /* @common:if [condition="treeShake.test-lib.getData"] */
            function getData() {
                return _utils.fetchData();
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.test-lib.processData"] */
            function processData(data) {
                return _processor.process(data);
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.test-lib.transformData"] */
            function transformData(data) {
                return _transformer.transform(data);
            }
            /* @common:endif */
            
            const __WEBPACK_DEFAULT_EXPORT__ = {
                /* @common:if [condition="treeShake.test-lib.getData"] */ getData, /* @common:endif */
                /* @common:if [condition="treeShake.test-lib.processData"] */ processData, /* @common:endif */
                /* @common:if [condition="treeShake.test-lib.transformData"] */ transformData /* @common:endif */
            };
        },
        
        "utils": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "fetchData": () => (fetchData),
                "formatData": () => (formatData)
            });
            
            function fetchData() {
                return { data: "test" };
            }
            
            function formatData(data) {
                return JSON.stringify(data);
            }
        },
        
        "constants": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "API_URL": () => (API_URL),
                "TIMEOUT": () => (TIMEOUT)
            });
            
            const API_URL = "https://api.example.com";
            const TIMEOUT = 5000;
        },
        
        "processor": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "process": () => (process)
            });
            
            // This module requires heavy-processor which should also be pruned
            var _heavyProcessor = __webpack_require__("heavy-processor");
            
            function process(data) {
                return _heavyProcessor.heavyProcess(data);
            }
        },
        
        "heavy-processor": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "heavyProcess": () => (heavyProcess)
            });
            
            function heavyProcess(data) {
                // Simulate heavy processing
                return { processed: data };
            }
        },
        
        "transformer": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "transform": () => (transform)
            });
            
            // This module requires parser which should also be pruned
            var _parser = __webpack_require__("parser");
            var _validator = __webpack_require__("validator");
            
            function transform(data) {
                const parsed = _parser.parse(data);
                return _validator.validate(parsed);
            }
        },
        
        "parser": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "parse": () => (parse)
            });
            
            function parse(input) {
                return JSON.parse(JSON.stringify(input));
            }
        },
        
        "validator": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "validate": () => (validate)
            });
            
            function validate(data) {
                return { valid: true, data };
            }
        }
    }
]);
"#.to_string()
    }

    #[test]
    fn test_macro_removal_and_pruning_with_only_getdata() {
        let source = create_chunk_with_proper_macros();
        
        // Config that only keeps getData, removing processData and transformData
        let config = json!({
            "treeShake": {
                "test-lib": {
                    "getData": true,
                    "processData": false,
                    "transformData": false,
                    "chunk_characteristics": {
                        "entry_module_id": "entry"
                    }
                }
            }
        });

        let (optimized_source, prune_result) = optimize_with_prune_result(source, config);
        
        // Verify all macro comments are removed
        assert!(!optimized_source.contains("@common:if"), 
            "All @common:if macro comments should be removed");
        assert!(!optimized_source.contains("@common:endif"), 
            "All @common:endif macro comments should be removed");
        
        // Verify pruning occurred
        assert!(prune_result.pruned_count > 0, 
            "Should have pruned unreachable modules");
        
        // Verify processor chain was pruned (not reachable when processData is false)
        assert!(prune_result.removed_modules.contains(&"processor".to_string()),
            "processor module should be pruned when processData is removed");
        assert!(prune_result.removed_modules.contains(&"heavy-processor".to_string()),
            "heavy-processor should be pruned as dependency of processor");
        
        // Verify transformer chain was pruned (not reachable when transformData is false)
        assert!(prune_result.removed_modules.contains(&"transformer".to_string()),
            "transformer module should be pruned when transformData is removed");
        assert!(prune_result.removed_modules.contains(&"parser".to_string()),
            "parser should be pruned as dependency of transformer");
        assert!(prune_result.removed_modules.contains(&"validator".to_string()),
            "validator should be pruned as dependency of transformer");
        
        // Verify essential modules are kept
        assert!(prune_result.kept_modules.contains(&"entry".to_string()),
            "entry module should always be kept");
        assert!(prune_result.kept_modules.contains(&"utils".to_string()),
            "utils should be kept as it's used by getData");
        assert!(prune_result.kept_modules.contains(&"constants".to_string()),
            "constants should be kept as it's always required");
        
        // Verify the functions were removed from the source
        assert!(!optimized_source.contains("function processData"),
            "processData function should be removed");
        assert!(!optimized_source.contains("function transformData"),
            "transformData function should be removed");
        assert!(optimized_source.contains("function getData"),
            "getData function should be kept");
    }

    #[test]
    fn test_all_exports_enabled_no_pruning() {
        let source = create_chunk_with_proper_macros();
        
        // Config that keeps all exports
        let config = json!({
            "treeShake": {
                "test-lib": {
                    "getData": true,
                    "processData": true,
                    "transformData": true,
                    "chunk_characteristics": {
                        "entry_module_id": "entry"
                    }
                }
            }
        });

        let (optimized_source, prune_result) = optimize_with_prune_result(source, config);
        
        // Verify macro comments are removed even when all exports are kept
        assert!(!optimized_source.contains("@common:if"), 
            "Macro comments should be removed even when all exports are kept");
        
        // Verify no pruning occurred (all modules reachable)
        assert_eq!(prune_result.pruned_count, 0, 
            "Should not prune any modules when all exports are enabled");
        
        // Verify all modules are kept
        assert!(prune_result.kept_modules.contains(&"entry".to_string()));
        assert!(prune_result.kept_modules.contains(&"utils".to_string()));
        assert!(prune_result.kept_modules.contains(&"constants".to_string()));
        assert!(prune_result.kept_modules.contains(&"processor".to_string()));
        assert!(prune_result.kept_modules.contains(&"heavy-processor".to_string()));
        assert!(prune_result.kept_modules.contains(&"transformer".to_string()));
        assert!(prune_result.kept_modules.contains(&"parser".to_string()));
        assert!(prune_result.kept_modules.contains(&"validator".to_string()));
        
        // Verify all functions are present
        assert!(optimized_source.contains("function getData"));
        assert!(optimized_source.contains("function processData"));
        assert!(optimized_source.contains("function transformData"));
    }

    #[test]
    fn test_partial_exports_partial_pruning() {
        let source = create_chunk_with_proper_macros();
        
        // Config that keeps processData but not transformData
        let config = json!({
            "treeShake": {
                "test-lib": {
                    "getData": true,
                    "processData": true,
                    "transformData": false,
                    "chunk_characteristics": {
                        "entry_module_id": "entry"
                    }
                }
            }
        });

        let (optimized_source, prune_result) = optimize_with_prune_result(source, config);
        
        // Verify macro comments are removed
        assert!(!optimized_source.contains("@common:"));
        
        // Verify only transformer chain is pruned
        assert!(prune_result.removed_modules.contains(&"transformer".to_string()),
            "transformer should be pruned");
        assert!(prune_result.removed_modules.contains(&"parser".to_string()),
            "parser should be pruned");
        assert!(prune_result.removed_modules.contains(&"validator".to_string()),
            "validator should be pruned");
        
        // Verify processor chain is kept
        assert!(prune_result.kept_modules.contains(&"processor".to_string()),
            "processor should be kept");
        assert!(prune_result.kept_modules.contains(&"heavy-processor".to_string()),
            "heavy-processor should be kept");
        
        // Verify function presence
        assert!(optimized_source.contains("function getData"));
        assert!(optimized_source.contains("function processData"));
        assert!(!optimized_source.contains("function transformData"));
    }

    #[test]
    fn test_inline_macro_conditions() {
        let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["inline-test"],
    {
        "module-with-inline-macros": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "feature1": () => (/* @common:if [condition="treeShake.lib.feature1"] */ feature1 /* @common:endif */),
                "feature2": () => (/* @common:if [condition="treeShake.lib.feature2"] */ feature2 /* @common:endif */),
                "always": () => (always)
            });
            
            // Inline conditional code
            /* @common:if [condition="treeShake.lib.feature1"] */ function feature1() { return "f1"; } /* @common:endif */
            
            // Multi-line conditional
            /* @common:if [condition="treeShake.lib.feature2"] */
            function feature2() {
                const result = "feature2";
                return result;
            }
            /* @common:endif */
            
            function always() {
                return "always";
            }
            
            // Conditional object properties
            const config = {
                /* @common:if [condition="treeShake.lib.feature1"] */ enableFeature1: true, /* @common:endif */
                /* @common:if [condition="treeShake.lib.feature2"] */ enableFeature2: true, /* @common:endif */
                base: "config"
            };
        }
    }
]);
"#.to_string();

        let config = json!({
            "treeShake": {
                "lib": {
                    "feature1": false,
                    "feature2": true,
                    "chunk_characteristics": {
                        "entry_module_id": "module-with-inline-macros"
                    }
                }
            }
        });

        let (optimized_source, _prune_result) = optimize_with_prune_result(source, config);
        
        // Debug: Print a snippet to see what's happening
        if optimized_source.contains("config") {
            let start = optimized_source.find("const config").unwrap_or(0);
            let end = (start + 200).min(optimized_source.len());
            eprintln!("Config object area: {}", &optimized_source[start..end]);
        }
        
        // Verify ALL macro comments are removed
        assert!(!optimized_source.contains("@common:if"),
            "All macro comments should be removed");
        assert!(!optimized_source.contains("@common:endif"),
            "All endif comments should be removed");
        
        // Verify feature1 code is removed but feature2 is kept
        assert!(!optimized_source.contains("function feature1"),
            "feature1 function should be removed");
        assert!(optimized_source.contains("function feature2"),
            "feature2 function should be kept");
        assert!(optimized_source.contains("function always"),
            "always function should be kept");
        
        // The optimizer successfully removes the conditional properties
        // We mainly care that the macro comments are gone and the right functions are kept/removed
        assert!(!optimized_source.contains("enableFeature1"),
            "enableFeature1 property should be removed");
    }

    #[test]
    fn test_nested_dependencies_pruning() {
        let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["nested"],
    {
        "entry": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "useFeatureA": () => (/* @common:if [condition="treeShake.nested.useFeatureA"] */ useFeatureA /* @common:endif */),
                "useFeatureB": () => (/* @common:if [condition="treeShake.nested.useFeatureB"] */ useFeatureB /* @common:endif */)
            });
            
            /* @common:if [condition="treeShake.nested.useFeatureA"] */
            var _featureA = __webpack_require__("featureA");
            function useFeatureA() {
                return _featureA.doFeatureA();
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.nested.useFeatureB"] */
            var _featureB = __webpack_require__("featureB");
            function useFeatureB() {
                return _featureB.doFeatureB();
            }
            /* @common:endif */
        },
        
        "featureA": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "doFeatureA": () => (doFeatureA)
            });
            
            var _depA1 = __webpack_require__("depA1");
            var _depA2 = __webpack_require__("depA2");
            
            function doFeatureA() {
                return _depA1.a1() + _depA2.a2();
            }
        },
        
        "featureB": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "doFeatureB": () => (doFeatureB)
            });
            
            var _depB1 = __webpack_require__("depB1");
            
            function doFeatureB() {
                return _depB1.b1();
            }
        },
        
        "depA1": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "a1": () => (a1)
            });
            
            var _depA1_1 = __webpack_require__("depA1-1");
            
            function a1() {
                return _depA1_1.deep() + "a1";
            }
        },
        
        "depA1-1": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "deep": () => (deep)
            });
            
            function deep() {
                return "deep";
            }
        },
        
        "depA2": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "a2": () => (a2)
            });
            
            function a2() {
                return "a2";
            }
        },
        
        "depB1": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "b1": () => (b1)
            });
            
            function b1() {
                return "b1";
            }
        }
    }
]);
"#.to_string();

        let config = json!({
            "treeShake": {
                "nested": {
                    "useFeatureA": false,
                    "useFeatureB": true,
                    "chunk_characteristics": {
                        "entry_module_id": "entry"
                    }
                }
            }
        });

        let (optimized_source, prune_result) = optimize_with_prune_result(source, config);
        
        // Verify macro comments are removed
        assert!(!optimized_source.contains("@common:"));
        
        // Verify entire featureA dependency tree is pruned
        assert!(prune_result.removed_modules.contains(&"featureA".to_string()),
            "featureA should be pruned");
        assert!(prune_result.removed_modules.contains(&"depA1".to_string()),
            "depA1 should be pruned");
        assert!(prune_result.removed_modules.contains(&"depA1-1".to_string()),
            "depA1-1 (nested dep) should be pruned");
        assert!(prune_result.removed_modules.contains(&"depA2".to_string()),
            "depA2 should be pruned");
        
        // Verify featureB dependency tree is kept
        assert!(prune_result.kept_modules.contains(&"featureB".to_string()),
            "featureB should be kept");
        assert!(prune_result.kept_modules.contains(&"depB1".to_string()),
            "depB1 should be kept");
        
        // Verify entry is kept
        assert!(prune_result.kept_modules.contains(&"entry".to_string()),
            "entry should be kept");
        
        // Verify pruning count
        assert_eq!(prune_result.pruned_count, 4,
            "Should have pruned exactly 4 modules (featureA tree)");
    }
}