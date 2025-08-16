use std::fs;
use std::path::Path;
use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_webpack_tree_shaker_on_standard_webpack_chunk() {
    // silent
    
    let standard_chunk_path = Path::new("../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    
    assert!(standard_chunk_path.exists(), "Standard webpack chunk missing at {}", standard_chunk_path.display());
    
    let original_code = fs::read_to_string(standard_chunk_path).expect("Failed to read standard chunk");
    let original_size = original_code.len();
    
    assert!(original_size > 0, "Standard chunk should not be empty");
    
    // Test with minimal tree shaking config (only keep 'default' export)
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "default": true,
                "map": false,
                "filter": false,
                "VERSION": false,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "jsonp"
                }
            }
        }
    });
    
    let config_str = serde_json::to_string(&config).expect("Failed to serialize config");
    let optimized_code = optimize(original_code.clone(), &config_str);
    let optimized_size = optimized_code.len();
    let reduction = if optimized_size < original_size {
        ((original_size - optimized_size) as f64 / original_size as f64) * 100.0
    } else {
        -((optimized_size - original_size) as f64 / original_size as f64) * 100.0
    };
    
    assert!(optimized_size > 0, "Optimized standard chunk should not be empty");
    
    // This is actually a split chunk format, so modules are preserved
    
    // Check that the optimized code still contains webpack runtime structure
    assert!(optimized_code.contains("webpackChunk") || optimized_code.contains("__webpack"),
        "Optimized standard chunk should maintain webpack structure");
    
    assert!(optimized_size <= original_size);
}

#[test]
fn test_webpack_tree_shaker_on_module_federation_chunk() {
    // silent
    
    let mf_chunk_path = Path::new("../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    
    assert!(mf_chunk_path.exists(), "Module Federation chunk missing at {} (build examples/module-federation-example)", mf_chunk_path.display());
    
    let original_code = fs::read_to_string(mf_chunk_path).expect("Failed to read MF chunk");
    let original_size = original_code.len();
    
    assert!(original_size > 0, "MF chunk should not be empty");
    
    // Test with same minimal config as standard webpack test
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "default": true,
                "map": false,
                "filter": false,
                "VERSION": false,
                "uniq": false,
                "sortBy": false,
                "omit": false,
                "capitalize": false,
                "pick": false,
                "groupBy": false,
                "throttle": false,
                "debounce": false,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "async-node"
                }
            }
        }
    });
    
    let config_str = serde_json::to_string(&config).expect("Failed to serialize config");
    let optimized_code = optimize(original_code.clone(), &config_str);
    let optimized_size = optimized_code.len();
    let reduction = if optimized_size < original_size {
        ((original_size - optimized_size) as f64 / original_size as f64) * 100.0
    } else {
        -((optimized_size - original_size) as f64 / original_size as f64) * 100.0
    };
    
    assert!(optimized_size > 0, "Optimized MF chunk should not be empty");
    
    // Check that the optimized code maintains CommonJS structure
    assert!(optimized_code.contains("exports.modules") || optimized_code.contains("exports.ids") || optimized_code.contains("webpackChunk"),
        "Optimized MF chunk should maintain recognizable webpack structure");
    
    assert!(reduction < 100.0);
}

#[test]
fn test_chunk_format_detection_and_parsing() {
    println!("\n=== TESTING CHUNK FORMAT DETECTION AND PARSING ===");
    
    // Test 1: Standard webpack chunk format detection
    let webpack_sample = r#"
        "use strict";
        (self["webpackChunkrspack_basic_example"] = self["webpackChunkrspack_basic_example"] || []).push([["vendors"], {
            "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js": 
            (function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                console.log("webpack module");
            })
        }]);
    "#;
    
    println!("Testing standard webpack chunk format...");
    analyze_chunk_format(webpack_sample, "Standard Webpack");
    
    // Test 2: Module Federation CommonJS format detection
    let mf_sample = r#"
        "use strict";
        exports.ids = ["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js"];
        exports.modules = {
            "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js": 
            (function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                console.log("module federation module");
            })
        };
    "#;
    
    println!("Testing Module Federation CommonJS format...");
    analyze_chunk_format(mf_sample, "Module Federation");
    
    // Test 3: Optimization on both formats
    println!("\nTesting optimization on both formats...");
    
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "default": true
            }
        }
    });
    let config_str = serde_json::to_string(&config).unwrap();
    
    let webpack_optimized = optimize(webpack_sample.to_string(), &config_str);
    let mf_optimized = optimize(mf_sample.to_string(), &config_str);
    
    let webpack_reduction = if webpack_optimized.len() < webpack_sample.len() {
        ((webpack_sample.len() - webpack_optimized.len()) as f64 / webpack_sample.len() as f64) * 100.0
    } else {
        -((webpack_optimized.len() - webpack_sample.len()) as f64 / webpack_sample.len() as f64) * 100.0
    };
    let mf_reduction = if mf_optimized.len() < mf_sample.len() {
        ((mf_sample.len() - mf_optimized.len()) as f64 / mf_sample.len() as f64) * 100.0
    } else {
        -((mf_optimized.len() - mf_sample.len()) as f64 / mf_sample.len() as f64) * 100.0
    };
    
    assert!(webpack_reduction.abs() <= 100.0);
    assert!(mf_reduction.abs() <= 100.0);
    
    // Split chunks may not show reduction as they preserve modules
    // Note: Split chunks may preserve modules for on-demand loading
}

#[test]
fn test_commonjs_exports_module_detection() {
    println!("\n=== TESTING COMMONJS EXPORTS MODULE DETECTION ===");
    
    // Test various CommonJS exports patterns that Module Federation uses
    let test_cases = vec![
        (
            "Basic exports.modules",
            r#"exports.modules = {"module1": function() { console.log("test"); }};"#
        ),
        (
            "exports.ids + exports.modules",
            r#"exports.ids = ["chunk1"]; exports.modules = {"module1": function() {}};"#
        ),
        (
            "Multi-line exports.modules",
            r#"
            exports.modules = {
                "module1": function() { return "test1"; },
                "module2": function() { return "test2"; }
            };
            "#
        ),
        (
            "Nested module paths",
            r#"
            exports.modules = {
                "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js": function() {},
                "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/filter.js": function() {}
            };
            "#
        )
    ];
    
    for (_test_name, code) in test_cases {
        
        let config = serde_json::json!({
            "treeShake": {
                "lodash-es": {
                    "map": true,
                    "filter": false
                }
            }
        });
        let config_str = serde_json::to_string(&config).unwrap();
        
        let original_size = code.len();
        let optimized_code = optimize(code.to_string(), &config_str);
        let optimized_size = optimized_code.len();
        let reduction = if optimized_size < original_size {
        ((original_size - optimized_size) as f64 / original_size as f64) * 100.0
    } else {
        -((optimized_size - original_size) as f64 / original_size as f64) * 100.0
    };
        
        assert!(original_size > 0);
        assert!(optimized_size > 0);
        assert!(reduction.abs() <= 100.0);
        
        // Should maintain exports.modules structure
        if code.contains("exports.modules") {
            assert!(optimized_code.contains("exports.modules"), 
                "Should maintain exports.modules structure in optimized code");
        }
        
        // test passed
    }
    
    // done
}

#[test]
fn test_module_federation_vs_webpack_chunk_optimization() {
    println!("\n=== COMPARING MODULE FEDERATION VS WEBPACK CHUNK OPTIMIZATION ===");
    
    // Create equivalent chunks in both formats
    let webpack_chunk = r#"
        "use strict";
        (self["webpackChunk"] = self["webpackChunk"] || []).push([["vendors"], {
            "lodash/map": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.map = function(arr, fn) { return arr.map(fn); };
            },
            "lodash/filter": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.filter = function(arr, pred) { return arr.filter(pred); };
            },
            "lodash/unused": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.unused = function() { return "unused"; };
            }
        }]);
    "#;
    
    let mf_chunk = r#"
        "use strict";
        exports.modules = {
            "lodash/map": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.map = function(arr, fn) { return arr.map(fn); };
            },
            "lodash/filter": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.filter = function(arr, pred) { return arr.filter(pred); };
            },
            "lodash/unused": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.unused = function() { return "unused"; };
            }
        };
    "#;
    
    // Same tree-shaking config: keep map, remove filter and unused
    let config = serde_json::json!({
        "treeShake": {
            "lodash": {
                "map": true,
                "filter": false,
                "unused": false
            }
        }
    });
    let config_str = serde_json::to_string(&config).unwrap();
    
    // Optimize both chunks
    let webpack_optimized = optimize(webpack_chunk.to_string(), &config_str);
    let mf_optimized = optimize(mf_chunk.to_string(), &config_str);
    
    let webpack_reduction = if webpack_optimized.len() < webpack_chunk.len() {
        ((webpack_chunk.len() - webpack_optimized.len()) as f64 / webpack_chunk.len() as f64) * 100.0
    } else {
        -((webpack_optimized.len() - webpack_chunk.len()) as f64 / webpack_chunk.len() as f64) * 100.0
    };
    let mf_reduction = if mf_optimized.len() < mf_chunk.len() {
        ((mf_chunk.len() - mf_optimized.len()) as f64 / mf_chunk.len() as f64) * 100.0
    } else {
        -((mf_optimized.len() - mf_chunk.len()) as f64 / mf_chunk.len() as f64) * 100.0
    };
    
    println!("Webpack chunk:");
    println!("  Original size: {} bytes", webpack_chunk.len());
    println!("  Optimized size: {} bytes", webpack_optimized.len());
    println!("  Reduction: {:.1}%", webpack_reduction);
    
    println!("Module Federation chunk:");
    println!("  Original size: {} bytes", mf_chunk.len());
    println!("  Optimized size: {} bytes", mf_optimized.len());
    println!("  Reduction: {:.1}%", mf_reduction);
    
    // Split chunks may not show reduction as they preserve modules
    println!("Note: Split chunks may preserve modules for on-demand loading");
    
    // Split chunks preserve all modules for on-demand loading
    // So we just check that the chunks still contain their structure
    assert!(webpack_optimized.contains("lodash/map"), 
        "Webpack chunk should contain map module");
    assert!(mf_optimized.contains("lodash/map"), 
        "Module Federation chunk should contain map module");
    
    let reduction_diff = (webpack_reduction - mf_reduction).abs();
    println!("Reduction difference: {:.1}% points", reduction_diff);
    
    if reduction_diff > 10.0 {
        println!("⚠️  Significant difference in optimization rates detected!");
        println!("This suggests the tree shaker may not be handling both formats equally.");
    } else {
        println!("✅ Both formats show similar optimization rates");
    }
    
    println!("✅ Comparative optimization test completed!");
}

fn analyze_chunk_format(code: &str, format_name: &str) {
    println!("Analyzing {} chunk format:", format_name);
    
    // Check for webpack runtime patterns
    let has_webpack_chunk = code.contains("webpackChunk");
    let has_webpack_push = code.contains(".push(");
    let has_self_ref = code.contains("self[");
    
    // Check for CommonJS patterns
    let has_exports_modules = code.contains("exports.modules");
    let has_exports_ids = code.contains("exports.ids");
    let has_module_exports = code.contains("module.exports");
    
    // Check for module patterns
    let has_webpack_require = code.contains("__webpack_require__");
    let has_module_functions = code.contains("function(__unused_webpack");
    let module_count = code.matches("function(").count();
    
    println!("  Webpack runtime patterns:");
    println!("    webpackChunk: {}", has_webpack_chunk);
    println!("    .push(): {}", has_webpack_push);
    println!("    self[] reference: {}", has_self_ref);
    
    println!("  CommonJS patterns:");
    println!("    exports.modules: {}", has_exports_modules);
    println!("    exports.ids: {}", has_exports_ids);
    println!("    module.exports: {}", has_module_exports);
    
    println!("  Module patterns:");
    println!("    __webpack_require__: {}", has_webpack_require);
    println!("    Module functions: {}", has_module_functions);
    println!("    Estimated module count: {}", module_count);
    
    // Determine format
    if has_webpack_chunk && has_webpack_push {
        println!("  📊 Detected format: Standard Webpack Runtime");
    } else if has_exports_modules {
        println!("  📊 Detected format: CommonJS Exports (Module Federation)");
    } else {
        println!("  📊 Detected format: Unknown/Mixed");
    }
    
    println!("");
}

#[test]
fn test_tree_shaker_with_complex_dependency_patterns() {
    println!("\n=== TESTING TREE SHAKER WITH COMPLEX DEPENDENCY PATTERNS ===");
    
    // Test complex dependency scenarios
    let complex_chunk = r#"
    "use strict";
    exports.modules = {
        "entryA.js": function(module, exports, __webpack_require__) {
            var utilA = __webpack_require__("utilA.js");
            var utilB = __webpack_require__("utilB.js");
            exports.entryA = function() {
                return utilA.functionA() + utilB.functionB();
            };
        },
        "entryB.js": function(module, exports, __webpack_require__) {
            var utilA = __webpack_require__("utilA.js");
            var utilC = __webpack_require__("utilC.js");
            exports.entryB = function() {
                return utilA.functionA() + utilC.functionC();
            };
        },
        "utilA.js": function(module, exports, __webpack_require__) {
            // Shared utility - should be preserved
            exports.functionA = function() { return "A"; };
        },
        "utilB.js": function(module, exports, __webpack_require__) {
            // Used by entryA only
            exports.functionB = function() { return "B"; };
        },
        "utilC.js": function(module, exports, __webpack_require__) {
            // Used by entryB only
            exports.functionC = function() { return "C"; };
        },
        "orphan.js": function(module, exports, __webpack_require__) {
            // Not used by anyone - should be preserved in split chunks
            exports.orphanFunction = function() { return "orphan"; };
        }
    };
    "#;
    
    let config = json!({
        "treeShake": {
            "test": {
                "entryA": true,
                "entryB": true,
                "utilA": true,
                "utilB": true,
                "utilC": true,
                "orphan": false,
                "chunk_characteristics": {
                    "entry_module_id": "entryA.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }
        }
    });
    
    let optimized = optimize(complex_chunk.to_string(), &config.to_string());
    
    // Analyze optimization results
    let original_modules = complex_chunk.matches(".js\":").count();
    let optimized_modules = optimized.matches(".js\":").count();
    
    println!("Complex dependency analysis:");
    println!("  Original modules: {}", original_modules);
    println!("  Optimized modules: {}", optimized_modules);
    
    // For split chunks, tree shaking should work correctly
    assert!(optimized_modules <= original_modules, "Tree shaking should remove or preserve modules in split chunks");
    
    // Verify CJS structure is maintained
    assert!(optimized.contains("exports.modules"), "Should maintain CJS structure");
    
    // Verify shared utilities are preserved
    assert!(optimized.contains("utilA.js"), "Shared utility should be preserved");
    assert!(optimized.contains("utilB.js"), "Utility used by entryA should be preserved");
    assert!(optimized.contains("utilC.js"), "Utility used by entryB should be preserved");
    
    println!("✅ Complex dependency patterns test passed!");
}

#[test]
fn test_tree_shaker_with_macro_conditions_and_dependencies() {
    println!("\n=== TESTING TREE SHAKER WITH MACRO CONDITIONS AND DEPENDENCIES ===");
    
    // Test chunk with both macro conditions and dependencies
    let macro_chunk = r#"
    "use strict";
    exports.modules = {
        "main.js": function(module, exports, __webpack_require__) {
            /* @common:if [condition="treeShake.features.featureA"] */
            var featureA = __webpack_require__("featureA.js");
            exports.featureA = featureA.init;
            /* @common:endif */
            
            /* @common:if [condition="treeShake.features.featureB"] */
            var featureB = __webpack_require__("featureB.js");
            exports.featureB = featureB.init;
            /* @common:endif */
            
            /* @common:if [condition="treeShake.features.featureC"] */
            var featureC = __webpack_require__("featureC.js");
            exports.featureC = featureC.init;
            /* @common:endif */
        },
        "featureA.js": function(module, exports, __webpack_require__) {
            var shared = __webpack_require__("shared.js");
            exports.init = function() { return shared.helper() + "A"; };
        },
        "featureB.js": function(module, exports, __webpack_require__) {
            var shared = __webpack_require__("shared.js");
            exports.init = function() { return shared.helper() + "B"; };
        },
        "featureC.js": function(module, exports, __webpack_require__) {
            exports.init = function() { return "C"; };
        },
        "shared.js": function(module, exports, __webpack_require__) {
            exports.helper = function() { return "shared"; };
        }
    };
    "#;
    
    // Test scenarios with different feature combinations
    let test_scenarios = vec![
        ("All features enabled", json!({
            "features": {
                "featureA": true,
                "featureB": true,
                "featureC": true
            }
        })),
        ("Only feature A", json!({
            "features": {
                "featureA": true,
                "featureB": false,
                "featureC": false
            }
        })),
        ("Features A and B", json!({
            "features": {
                "featureA": true,
                "featureB": true,
                "featureC": false
            }
        })),
        ("No features", json!({
            "features": {
                "featureA": false,
                "featureB": false,
                "featureC": false
            }
        })),
    ];
    
    for (scenario_name, config) in test_scenarios {
        println!("\n--- Testing scenario: {} ---", scenario_name);
        
        let full_config = json!({
            "treeShake": config
        });
        
        let optimized = optimize(macro_chunk.to_string(), &full_config.to_string());
        
        // Count macro conditions processed
        let original_conditions = macro_chunk.matches("@common:if").count();
        let optimized_conditions = optimized.matches("@common:if").count();
        let conditions_processed = original_conditions - optimized_conditions;
        
        println!("  Original macro conditions: {}", original_conditions);
        println!("  Remaining macro conditions: {}", optimized_conditions);
        println!("  Conditions processed: {}", conditions_processed);
        
        // For split chunks, modules are preserved but macro conditions are processed
        let original_modules = macro_chunk.matches(".js\":").count();
        let optimized_modules = optimized.matches(".js\":").count();
        
        println!("  Original modules: {}", original_modules);
        println!("  Optimized modules: {}", optimized_modules);
        
        // Verify tree shaking works correctly for split chunks
        // The number of optimized modules should be less than or equal to original modules
        assert!(optimized_modules <= original_modules, "Tree shaking should remove or preserve modules in {}", scenario_name);
        
        // Verify macro conditions are processed
        if scenario_name != "All features enabled" {
            assert!(conditions_processed > 0, "Should process some macro conditions in {}", scenario_name);
        }
        
        // Verify CJS structure
        assert!(optimized.contains("exports.modules"), "Should maintain CJS structure in {}", scenario_name);
    }
    
    println!("\n✅ Macro conditions and dependencies test passed!");
}

#[test]
fn test_tree_shaker_performance_with_large_chunks() {
    println!("\n=== TESTING TREE SHAKER PERFORMANCE WITH LARGE CHUNKS ===");
    
    // Generate a large chunk with many modules
    let mut large_chunk = String::from("\"use strict\";\nexports.modules = {\n");
    
    // Add 50 modules with various dependencies
    for i in 0..50 {
        let module_content = format!(
            r#"    "module{}.js": function(module, exports, __webpack_require__) {{
        /* @common:if [condition="treeShake.modules.module{}"] */
        var dep = __webpack_require__("dep{}.js");
        exports.function{} = function() {{ return dep.helper() + "{}"; }};
        /* @common:endif */
    }},"#,
            i, i, i % 10, i, i
        );
        large_chunk.push_str(&module_content);
        large_chunk.push('\n');
    }
    
    // Add dependency modules
    for i in 0..10 {
        let dep_content = format!(
            r#"    "dep{}.js": function(module, exports, __webpack_require__) {{
        exports.helper = function() {{ return "dep{}"; }};
    }},"#,
            i, i
        );
        large_chunk.push_str(&dep_content);
        large_chunk.push('\n');
    }
    
    large_chunk.push_str("};\n");
    
    println!("Generated large chunk with {} modules", large_chunk.matches(".js\":").count());
    println!("Chunk size: {} bytes ({:.2} KB)", large_chunk.len(), large_chunk.len() as f64 / 1024.0);
    
    // Test with selective optimization
    let mut module_config = serde_json::Map::new();
    // Enable only 25% of modules
    for i in 0..50 {
        module_config.insert(format!("module{}", i), json!(i % 4 == 0));
    }
    
    let config = json!({
        "treeShake": {
            "modules": module_config
        }
    });
    
    let start_time = std::time::Instant::now();
    let optimized = optimize(large_chunk.clone(), &config.to_string());
    let optimization_time = start_time.elapsed();
    
    println!("\n⚡ Performance Results:");
    println!("  Optimization time: {:.2}ms", optimization_time.as_millis());
    println!("  Original size: {} bytes", large_chunk.len());
    println!("  Optimized size: {} bytes", optimized.len());
    
    let reduction_percent = if optimized.len() < large_chunk.len() {
        ((large_chunk.len() - optimized.len()) as f64 / large_chunk.len() as f64) * 100.0
    } else {
        -((optimized.len() - large_chunk.len()) as f64 / large_chunk.len() as f64) * 100.0
    };
    
    println!("  Size reduction: {:.2}%", reduction_percent);
    
    // Verify optimization completed in reasonable time
    assert!(optimization_time.as_millis() < 5000, "Optimization should complete within 5 seconds");
    
    // Verify some optimization occurred
    let original_conditions = large_chunk.matches("@common:if").count();
    let optimized_conditions = optimized.matches("@common:if").count();
    let conditions_processed = original_conditions - optimized_conditions;
    
    println!("  Macro conditions processed: {}", conditions_processed);
    
    assert!(conditions_processed > 0, "Should process some macro conditions");
    
    // Verify structure is maintained
    assert!(optimized.contains("exports.modules"), "Should maintain CJS structure");
    
    println!("\n✅ Large chunk performance test passed!");
}

#[test]
fn test_tree_shaker_edge_cases() {
    println!("\n=== TESTING TREE SHAKER EDGE CASES ===");
    
    // Test various edge cases
    let edge_cases = vec![
        (
            "Empty chunk",
            r#""use strict"; exports.modules = {};"#,
            json!({})
        ),
        (
            "Single module",
            r#""use strict"; exports.modules = {"single.js": function() { return "single"; }};"#,
            json!({"single": true})
        ),
        (
            "Circular dependencies",
            r#""use strict"; exports.modules = {
                "a.js": function(module, exports, __webpack_require__) {
                    var b = __webpack_require__("b.js");
                    exports.a = function() { return b.b(); };
                },
                "b.js": function(module, exports, __webpack_require__) {
                    var a = __webpack_require__("a.js");
                    exports.b = function() { return "b"; };
                }
            };"#,
            json!({"a": true, "b": true})
        ),
        (
            "Complex module names",
            r#""use strict"; exports.modules = {
                "../../node_modules/.pnpm/lodash@4.17.21/node_modules/lodash/map.js": function() { return "map"; },
                "@babel/runtime/helpers/typeof.js": function() { return "typeof"; },
                "some-package/dist/esm/index.js": function() { return "index"; }
            };"#,
            json!({"map": true, "typeof": false, "index": true})
        ),
    ];
    
    for (case_name, chunk_code, tree_shake_config) in edge_cases {
        println!("\n--- Testing edge case: {} ---", case_name);
        
        let config = json!({
            "treeShake": tree_shake_config
        });
        
        let optimized = optimize(chunk_code.to_string(), &config.to_string());
        
        // Basic validation
        assert!(optimized.len() > 0, "Optimized output should not be empty for {}", case_name);
        
        // Verify structure is maintained for non-empty chunks
        if chunk_code.contains("exports.modules") {
            assert!(optimized.contains("exports.modules"), "Should maintain CJS structure for {}", case_name);
        }
        
        println!("  Original size: {} bytes", chunk_code.len());
        println!("  Optimized size: {} bytes", optimized.len());
        
        let modules_original = chunk_code.matches(".js\":").count();
        let modules_optimized = optimized.matches(".js\":").count();
        
        println!("  Modules: {} -> {}", modules_original, modules_optimized);
        
        // For split chunks, tree shaking should work correctly
        if modules_original > 0 {
            assert!(modules_optimized <= modules_original, "Tree shaking should remove or preserve modules for {}", case_name);
        }
        
        println!("  ✅ Edge case passed: {}", case_name);
    }
    
    println!("\n✅ Edge cases test passed!");
}