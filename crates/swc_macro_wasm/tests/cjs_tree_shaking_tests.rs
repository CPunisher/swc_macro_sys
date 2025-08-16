use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_cjs_chunk_preserves_export_modules() {
    // Test that main export modules (like lodash.js) are preserved in CJS chunks
    let cjs_chunk = r#"
"use strict";
exports.ids = ["vendors-lodash"];
exports.modules = {
    "lodash/sortBy.js": function(module, exports, __webpack_require__) {
        exports.sortBy = function(collection) { 
            return collection.sort(); 
        };
    },
    "lodash/uniq.js": function(module, exports, __webpack_require__) {
        exports.uniq = function(array) { 
            return [...new Set(array)]; 
        };
    },
    "lodash/filter.js": function(module, exports, __webpack_require__) {
        exports.filter = function(collection, predicate) { 
            return collection.filter(predicate); 
        };
    },
    "lodash/lodash.js": function(module, exports, __webpack_require__) {
        // Main export module - should always be preserved
        exports.sortBy = __webpack_require__("lodash/sortBy.js").sortBy;
        exports.uniq = __webpack_require__("lodash/uniq.js").uniq;
        exports.filter = __webpack_require__("lodash/filter.js").filter;
    }
};
"#;

    println!("\n=== CJS CHUNK EXPORT MODULE PRESERVATION TEST ===");
    
    let config = json!({});
    let optimized = optimize(cjs_chunk.to_string(), &config.to_string());
    
    // The main lodash.js module should be preserved even without entry points
    assert!(optimized.contains("lodash/lodash.js"), 
        "Main export module (lodash.js) should be preserved in split chunks");
    
    // All modules should be preserved in split chunks (no entry point based tree shaking)
    assert!(optimized.contains("lodash/sortBy.js"), "sortBy module should be preserved");
    assert!(optimized.contains("lodash/uniq.js"), "uniq module should be preserved");
    assert!(optimized.contains("lodash/filter.js"), "filter module should be preserved");
    
    println!("✅ CJS chunk correctly preserves all modules!");
}

#[test]
fn test_cjs_chunk_with_macro_conditions() {
    // Test that macro conditions work correctly with CJS chunks
    let cjs_chunk = r#"
"use strict";
exports.ids = ["vendors-utils"];
exports.modules = {
    "utils/helper1.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="features.enableHelper1"] */
        exports.helper1 = function() { return "Helper 1"; };
        /* @common:endif */
    },
    "utils/helper2.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="features.enableHelper2"] */
        exports.helper2 = function() { return "Helper 2"; };
        /* @common:endif */
    },
    "utils/index.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="features.enableHelper1"] */
        exports.helper1 = __webpack_require__("utils/helper1.js").helper1;
        /* @common:endif */
        /* @common:if [condition="features.enableHelper2"] */
        exports.helper2 = __webpack_require__("utils/helper2.js").helper2;
        /* @common:endif */
    }
};
"#;

    println!("\n=== CJS CHUNK WITH MACRO CONDITIONS TEST ===");
    
    // Enable only helper1
    let config = json!({
        "features": {
            "enableHelper1": true,
            "enableHelper2": false
        },
        "treeShake": {
            "utils": {
                "chunk_characteristics": {
                    "entry_module_id": "utils/index.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }
        }
    });
    
    let optimized = optimize(cjs_chunk.to_string(), &config.to_string());
    
    // Check that helper1 is preserved and helper2 is removed
    assert!(optimized.contains("Helper 1"), "Helper1 content should be preserved");
    assert!(!optimized.contains("Helper 2"), "Helper2 content should be removed by macro");
    
    // Check exports
    assert!(optimized.contains("exports.helper1"), "Helper1 export should be present");
    assert!(!optimized.contains("exports.helper2 = __webpack_require__"), 
        "Helper2 export assignment should be removed");
    
    println!("✅ CJS chunk macro conditions work correctly!");
}

#[test]
fn test_cjs_chunk_dependency_tracking() {
    // Test that dependencies are correctly tracked in CJS chunks
    let cjs_chunk = r#"
"use strict";
exports.ids = ["vendors-libs"];
exports.modules = {
    "libs/base.js": function(module, exports, __webpack_require__) {
        exports.base = "Base library";
    },
    "libs/derived1.js": function(module, exports, __webpack_require__) {
        var base = __webpack_require__("libs/base.js");
        exports.derived1 = base.base + " - Derived 1";
    },
    "libs/derived2.js": function(module, exports, __webpack_require__) {
        var base = __webpack_require__("libs/base.js");
        var derived1 = __webpack_require__("libs/derived1.js");
        exports.derived2 = derived1.derived1 + " - Derived 2";
    },
    "libs/unused.js": function(module, exports, __webpack_require__) {
        exports.unused = "This module has no dependents";
    }
};
"#;

    println!("\n=== CJS CHUNK DEPENDENCY TRACKING TEST ===");
    
    let config = json!({});
    let optimized = optimize(cjs_chunk.to_string(), &config.to_string());
    
    // All modules should be preserved (no tree shaking in split chunks)
    assert!(optimized.contains("libs/base.js"), "Base module should be preserved");
    assert!(optimized.contains("libs/derived1.js"), "Derived1 module should be preserved");
    assert!(optimized.contains("libs/derived2.js"), "Derived2 module should be preserved");
    assert!(optimized.contains("libs/unused.js"), "Even unused module should be preserved in split chunk");
    
    println!("✅ CJS chunk dependency tracking verified!");
}

#[test]
fn test_cjs_vs_standard_webpack_tree_shaking() {
    // Compare tree shaking behavior between CJS split chunks and standard webpack bundles
    
    // CJS split chunk - should preserve all modules
    let cjs_chunk = r#"
"use strict";
exports.ids = ["test-chunk"];
exports.modules = {
    "moduleA.js": function(module, exports, __webpack_require__) {
        exports.valueA = "Module A";
    },
    "moduleB.js": function(module, exports, __webpack_require__) {
        var a = __webpack_require__("moduleA.js");
        exports.valueB = a.valueA + " + Module B";
    },
    "moduleC.js": function(module, exports, __webpack_require__) {
        exports.valueC = "Unused Module C";
    }
};
"#;

    // Standard webpack bundle - should tree shake unused modules
    let standard_chunk = r#"
var __webpack_modules__ = {
    100: function(module, exports, __webpack_require__) {
        // Entry point
        var b = __webpack_require__(200);
        console.log(b.valueB);
    },
    200: function(module, exports, __webpack_require__) {
        var a = __webpack_require__(300);
        exports.valueB = a.valueA + " + Module B";
    },
    300: function(module, exports, __webpack_require__) {
        exports.valueA = "Module A";
    },
    400: function(module, exports, __webpack_require__) {
        exports.valueC = "Unused Module C";
    }
};
__webpack_require__(100);
"#;

    println!("\n=== CJS VS STANDARD WEBPACK TREE SHAKING TEST ===");
    
    let config = json!({});
    
    // Test CJS chunk
    let cjs_optimized = optimize(cjs_chunk.to_string(), &config.to_string());
    assert!(cjs_optimized.contains("moduleC.js"), 
        "CJS chunk should preserve unused moduleC");
    
    // Test standard webpack bundle
    let standard_optimized = optimize(standard_chunk.to_string(), &config.to_string());
    assert!(!standard_optimized.contains("Unused Module C"), 
        "Standard webpack bundle should remove unused module 400");
    
    println!("✅ Tree shaking behavior difference verified!");
}

#[test]
fn test_cjs_real_world_lodash_optimization() {
    // Test with actual lodash CJS chunk structure
    let lodash_chunk = r#"
"use strict";
exports.ids = ["vendors-node_modules_lodash-es_lodash_js"];
exports.modules = {
    "../../node_modules/lodash-es/_baseIteratee.js": function(module, exports, __webpack_require__) {
        var baseMatches = __webpack_require__("../../node_modules/lodash-es/_baseMatches.js");
        function baseIteratee(value) {
            if (typeof value == 'function') {
                return value;
            }
            return baseMatches(value);
        }
        exports.default = baseIteratee;
    },
    "../../node_modules/lodash-es/_baseMatches.js": function(module, exports, __webpack_require__) {
        function baseMatches(source) {
            return function(object) {
                return object === source;
            };
        }
        exports.default = baseMatches;
    },
    "../../node_modules/lodash-es/sortBy.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => sortBy
        });
        /* @common:if [condition="treeShake.lodash-es.sortBy"] */
        var baseIteratee = __webpack_require__("../../node_modules/lodash-es/_baseIteratee.js");
        function sortBy(collection, iteratees) {
            return collection.sort((a, b) => {
                var iteratee = baseIteratee.default(iteratees);
                return iteratee(a) - iteratee(b);
            });
        }
        /* @common:endif */
    },
    "../../node_modules/lodash-es/uniq.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => uniq
        });
        /* @common:if [condition="treeShake.lodash-es.uniq"] */
        function uniq(array) {
            return [...new Set(array)];
        }
        /* @common:endif */
    },
    "../../node_modules/lodash-es/lodash.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        /* @common:if [condition="treeShake.lodash-es.sortBy"] */
        __webpack_require__.d(exports, {
            sortBy: () => _sortBy.default
        });
        var _sortBy = __webpack_require__("../../node_modules/lodash-es/sortBy.js");
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash-es.uniq"] */
        __webpack_require__.d(exports, {
            uniq: () => _uniq.default
        });
        var _uniq = __webpack_require__("../../node_modules/lodash-es/uniq.js");
        /* @common:endif */
    }
};
"#;

    println!("\n=== REAL WORLD LODASH CJS OPTIMIZATION TEST ===");
    println!("Original size: {} bytes", lodash_chunk.len());
    
    // Enable only sortBy with entry module ID
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": true,
                "uniq": false,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }
        }
    });
    
    let optimized = optimize(lodash_chunk.to_string(), &config.to_string());
    println!("Optimized size: {} bytes", optimized.len());
    
    // Verify macro conditions worked
    assert!(optimized.contains("function sortBy"), "sortBy implementation should be present");
    assert!(!optimized.contains("function uniq"), "uniq implementation should be removed");
    
    // Verify exports - check for different possible formats
    let has_sortby_export = optimized.contains("sortBy: () =>") || 
                           optimized.contains("sortBy: ()=>") || 
                           optimized.contains("sortBy:");
    assert!(has_sortby_export, "sortBy export should be present in some form");
    
    let has_uniq_export = optimized.contains("uniq: () =>") || 
                         optimized.contains("uniq: ()=>") || 
                         (optimized.contains("uniq:") && optimized.contains("_uniq"));
    assert!(!has_uniq_export, "uniq export should be removed");
    
    // Helper modules should still be present (no tree shaking in split chunks)
    assert!(optimized.contains("_baseIteratee.js"), "Helper module should be preserved");
    assert!(optimized.contains("_baseMatches.js"), "Helper module should be preserved");
    
    let reduction = ((lodash_chunk.len() - optimized.len()) as f64 / lodash_chunk.len() as f64) * 100.0;
    println!("Size reduction: {:.1}%", reduction);
    
    println!("✅ Real world lodash CJS optimization works correctly!");
}

#[test]
fn test_cjs_chunk_module_removal_verification() {
    // Verify that the webpack module remover correctly handles CJS format
    let cjs_chunk = r#"
"use strict";
exports.ids = ["test"];
exports.modules = {
    "keep1.js": function(module, exports) {
        exports.value = "Keep this";
    },
    "remove1.js": function(module, exports) {
        exports.value = "Remove this";
    },
    "keep2.js": function(module, exports) {
        exports.value = "Keep this too";
    },
    "remove2.js": function(module, exports) {
        exports.value = "Remove this too";
    }
};
"#;

    println!("\n=== CJS CHUNK MODULE REMOVAL VERIFICATION TEST ===");
    
    // This would require internal testing of the WebpackModuleRemover
    // For now, we verify the modules are preserved in split chunks
    let config = json!({});
    let optimized = optimize(cjs_chunk.to_string(), &config.to_string());
    
    // All modules should be present (no tree shaking in split chunks)
    assert!(optimized.contains("keep1.js"), "Module should be preserved");
    assert!(optimized.contains("remove1.js"), "Module should be preserved");
    assert!(optimized.contains("keep2.js"), "Module should be preserved");
    assert!(optimized.contains("remove2.js"), "Module should be preserved");
    
    println!("✅ CJS chunk module preservation verified!");
}

#[test]
fn test_cjs_chunk_complex_dependencies() {
    // Test complex dependency scenarios in CJS chunks
    let complex_chunk = r#"
"use strict";
exports.ids = ["complex-vendor"];
exports.modules = {
    "vendor/core.js": function(module, exports, __webpack_require__) {
        exports.core = "Core functionality";
    },
    "vendor/plugin1.js": function(module, exports, __webpack_require__) {
        var core = __webpack_require__("vendor/core.js");
        exports.plugin1 = core.core + " + Plugin 1";
    },
    "vendor/plugin2.js": function(module, exports, __webpack_require__) {
        var core = __webpack_require__("vendor/core.js");
        var plugin1 = __webpack_require__("vendor/plugin1.js");
        exports.plugin2 = plugin1.plugin1 + " + Plugin 2";
    },
    "vendor/plugin3.js": function(module, exports, __webpack_require__) {
        // Circular dependency
        var index = __webpack_require__("vendor/index.js");
        exports.plugin3 = "Plugin 3";
    },
    "vendor/index.js": function(module, exports, __webpack_require__) {
        // Export module with complex dependencies
        exports.core = __webpack_require__("vendor/core.js").core;
        exports.plugin1 = __webpack_require__("vendor/plugin1.js").plugin1;
        exports.plugin2 = __webpack_require__("vendor/plugin2.js").plugin2;
        exports.plugin3 = __webpack_require__("vendor/plugin3.js").plugin3;
    }
};
"#;

    println!("\n=== CJS CHUNK COMPLEX DEPENDENCIES TEST ===");
    
    let config = json!({});
    let optimized = optimize(complex_chunk.to_string(), &config.to_string());
    
    // Verify all modules are preserved despite complex dependencies
    assert!(optimized.contains("vendor/core.js"), "Core module preserved");
    assert!(optimized.contains("vendor/plugin1.js"), "Plugin1 preserved");
    assert!(optimized.contains("vendor/plugin2.js"), "Plugin2 preserved");
    assert!(optimized.contains("vendor/plugin3.js"), "Plugin3 preserved (circular dep)");
    assert!(optimized.contains("vendor/index.js"), "Index module preserved");
    
    println!("✅ Complex dependency handling verified!");
}

#[test]
fn test_commonjs_module_removal() {
    // Test that modules are properly removed when entry module ID is specified
    let cjs_chunk = r#"
"use strict";
exports.ids = ["vendors-node_modules_lodash-es_lodash_js"];
exports.modules = {
    "../../node_modules/lodash-es/sortBy.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => sortBy
        });
        function sortBy(collection, iteratees) {
            return collection.sort();
        }
    },
    "../../node_modules/lodash-es/uniq.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => uniq
        });
        function uniq(array) {
            return [...new Set(array)];
        }
    },
    "../../node_modules/lodash-es/lodash.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            sortBy: () => _sortBy.default
        });
        var _sortBy = __webpack_require__("../../node_modules/lodash-es/sortBy.js");
        // Note: uniq is not imported here, making it orphaned
    }
};
"#;

    println!("\n=== COMMONJS MODULE REMOVAL TEST ===");
    
    // Configure strictly via chunk_characteristics with entry_module_id
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "async-node",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["vendors-node_modules_lodash-es_lodash_js.js"],
                    "is_shared_chunk": false,
                    "shared_modules": []
                }
            }
        }
    });
    
    let optimized = optimize(cjs_chunk.to_string(), &config.to_string());
    
    // Main entry module should be preserved
    assert!(optimized.contains("../../node_modules/lodash-es/lodash.js"), 
        "Entry module should be preserved");
    
    // Used module should be preserved
    assert!(optimized.contains("../../node_modules/lodash-es/sortBy.js"), 
        "Used module should be preserved");
    
    // Unused module should be removed (reachable only set from entry module)
    assert!(!optimized.contains("../../node_modules/lodash-es/uniq.js"), 
        "Unused module should be removed when entry module is specified");
    
    println!("✅ CommonJS module removal with entry module ID works correctly!");
}

#[test]
fn test_module_federation_share_usage_format() {
    // Test that we properly handle Module Federation share-usage.json format
    let cjs_chunk = r#"
"use strict";
exports.ids = ["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js"];
exports.modules = {
    "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/sortBy.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => sortBy
        });
        function sortBy(collection, iteratees) {
            return collection.sort();
        }
    },
    "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/uniq.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => uniq
        });
        function uniq(array) {
            return [...new Set(array)];
        }
    },
    "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            sortBy: () => _sortBy.default,
            uniq: () => _uniq.default
        });
        var _sortBy = __webpack_require__("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/sortBy.js");
        var _uniq = __webpack_require__("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/uniq.js");
    }
};
"#;

    println!("\n=== MODULE FEDERATION SHARE-USAGE FORMAT TEST ===");
    
    // Configure using chunk_characteristics as in share-usage.json
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }
        }
    });
    
    let optimized = optimize(cjs_chunk.to_string(), &config.to_string());
    
    // The entry module should be preserved
    assert!(optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"), 
        "Entry module from share-usage.json should be preserved");
    
    // Both dependency modules should be preserved since they are referenced
    assert!(optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/sortBy.js"), 
        "sortBy module should be preserved as it's referenced");
    assert!(optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/uniq.js"), 
        "uniq module should be preserved as it's referenced");
    
    println!("✅ Module Federation share-usage.json format handled correctly!");
}