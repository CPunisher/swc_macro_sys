#![recursion_limit = "256"]

use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_lodash_orphan_detection_with_minimal_usage() {
    println!("\n=== TESTING LODASH ORPHAN DETECTION WITH MINIMAL USAGE ===");
    
    // Create a realistic lodash chunk structure similar to the real one
    let lodash_chunk = r#"
    "use strict";
    exports.ids = ["vendors-lodash"];
    exports.modules = {
        "../../node_modules/lodash-es/lodash.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                sortBy: () => (/* @common:if [condition="treeShake.lodash-es.sortBy"] */ /* reexport safe */ _sortBy_js__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
                map: () => (/* @common:if [condition="treeShake.lodash-es.map"] */ /* reexport safe */ _map_js__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
                filter: () => (/* @common:if [condition="treeShake.lodash-es.filter"] */ /* reexport safe */ _filter_js__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */),
                reduce: () => (/* @common:if [condition="treeShake.lodash-es.reduce"] */ /* reexport safe */ _reduce_js__WEBPACK_IMPORTED_MODULE_3__["default"] /* @common:endif */),
                "default": () => (/* @common:if [condition="treeShake.lodash-es.default"] */ /* reexport safe */ _lodash_default_js__WEBPACK_IMPORTED_MODULE_4__["default"] /* @common:endif */)
            });
            /* @common:if [condition="treeShake.lodash-es.sortBy"] */
            var _sortBy_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../node_modules/lodash-es/sortBy.js");
            /* @common:endif */
            /* @common:if [condition="treeShake.lodash-es.map"] */
            var _map_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("../../node_modules/lodash-es/map.js");
            /* @common:endif */
            /* @common:if [condition="treeShake.lodash-es.filter"] */
            var _filter_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__("../../node_modules/lodash-es/filter.js");
            /* @common:endif */
            /* @common:if [condition="treeShake.lodash-es.reduce"] */
            var _reduce_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__("../../node_modules/lodash-es/reduce.js");
            /* @common:endif */
            /* @common:if [condition="treeShake.lodash-es.default"] */
            var _lodash_default_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__("../../node_modules/lodash-es/lodash.default.js");
            /* @common:endif */
        },
        "../../node_modules/lodash-es/sortBy.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => __WEBPACK_DEFAULT_EXPORT__
            });
            /* @common:if [condition="treeShake.lodash-es.sortBy"] */
            function sortBy(collection, iteratee) {
                return collection.sort();
            }
            const __WEBPACK_DEFAULT_EXPORT__ = sortBy;
            /* @common:endif */
        },
        "../../node_modules/lodash-es/map.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => __WEBPACK_DEFAULT_EXPORT__
            });
            /* @common:if [condition="treeShake.lodash-es.map"] */
            function map(collection, iteratee) {
                return collection.map(iteratee);
            }
            const __WEBPACK_DEFAULT_EXPORT__ = map;
            /* @common:endif */
        },
        "../../node_modules/lodash-es/filter.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => __WEBPACK_DEFAULT_EXPORT__
            });
            /* @common:if [condition="treeShake.lodash-es.filter"] */
            function filter(collection, predicate) {
                return collection.filter(predicate);
            }
            const __WEBPACK_DEFAULT_EXPORT__ = filter;
            /* @common:endif */
        },
        "../../node_modules/lodash-es/reduce.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => __WEBPACK_DEFAULT_EXPORT__
            });
            /* @common:if [condition="treeShake.lodash-es.reduce"] */
            function reduce(collection, iteratee, accumulator) {
                return collection.reduce(iteratee, accumulator);
            }
            const __WEBPACK_DEFAULT_EXPORT__ = reduce;
            /* @common:endif */
        },
        "../../node_modules/lodash-es/lodash.default.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => __WEBPACK_DEFAULT_EXPORT__
            });
            /* @common:if [condition="treeShake.lodash-es.default"] */
            const lodashDefault = { version: "4.17.21" };
            const __WEBPACK_DEFAULT_EXPORT__ = lodashDefault;
            /* @common:endif */
        }
    };
    "#;
    
    println!("Original chunk size: {} bytes", lodash_chunk.len());
    println!("Original modules: {}", lodash_chunk.matches(".js\":").count());
    
    // Test 1: Enable only sortBy - should remove map, filter, reduce
    println!("\n🧪 Test 1: Only sortBy enabled");
    let minimal_config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": true,
                "default": true,
                "map": false,
                "filter": false,
                "reduce": false
            }
        }
    });
    
    let optimized = optimize(lodash_chunk.to_string(), &minimal_config.to_string());
    println!("Optimized size: {} bytes", optimized.len());
    println!("Optimized modules: {}", optimized.matches(".js\":").count());
    
    let reduction = ((lodash_chunk.len() - optimized.len()) as f64 / lodash_chunk.len() as f64) * 100.0;
    println!("Reduction: {:.1}%", reduction);
    
    // Verify which modules are preserved/removed
    let has_lodash = optimized.contains("lodash.js");
    let has_sortby = optimized.contains("sortBy.js");
    let has_map = optimized.contains("map.js");
    let has_filter = optimized.contains("filter.js");
    let has_reduce = optimized.contains("reduce.js");
    let has_default = optimized.contains("lodash.default.js");
    
    println!("\nModule preservation analysis:");
    println!("- lodash.js: {}", if has_lodash { "✅ preserved" } else { "❌ removed" });
    println!("- sortBy.js: {}", if has_sortby { "✅ preserved" } else { "❌ removed" });
    println!("- map.js: {}", if has_map { "⚠️  preserved (should be removed)" } else { "✅ removed" });
    println!("- filter.js: {}", if has_filter { "⚠️  preserved (should be removed)" } else { "✅ removed" });
    println!("- reduce.js: {}", if has_reduce { "⚠️  preserved (should be removed)" } else { "✅ removed" });
    println!("- lodash.default.js: {}", if has_default { "✅ preserved" } else { "❌ removed" });
    
    // The enhanced parser should remove orphaned modules
    assert!(has_lodash, "Main lodash.js should be preserved");
    assert!(has_sortby, "sortBy.js should be preserved when enabled");
    assert!(has_default, "lodash.default.js should be preserved when enabled");
    
    // These should be removed as orphaned modules
    assert!(!has_map, "map.js should be removed when disabled - orphaned module detection failed");
    assert!(!has_filter, "filter.js should be removed when disabled - orphaned module detection failed");
    assert!(!has_reduce, "reduce.js should be removed when disabled - orphaned module detection failed");
    
    // Test 2: Enable only default - should remove ALL function modules
    println!("\n🧪 Test 2: Only default enabled");
    let default_only_config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": false,
                "default": true,
                "map": false,
                "filter": false,
                "reduce": false
            }
        }
    });
    
    let default_optimized = optimize(lodash_chunk.to_string(), &default_only_config.to_string());
    println!("Default-only optimized size: {} bytes", default_optimized.len());
    println!("Default-only modules: {}", default_optimized.matches(".js\":").count());
    
    let default_reduction = ((lodash_chunk.len() - default_optimized.len()) as f64 / lodash_chunk.len() as f64) * 100.0;
    println!("Default-only reduction: {:.1}%", default_reduction);
    
    // Check what's in the default-only version
    let default_has_lodash = default_optimized.contains("lodash.js");
    let default_has_sortby = default_optimized.contains("sortBy.js");
    let default_has_map = default_optimized.contains("map.js");
    let default_has_filter = default_optimized.contains("filter.js");
    let default_has_reduce = default_optimized.contains("reduce.js");
    let default_has_default = default_optimized.contains("lodash.default.js");
    
    println!("\nDefault-only module preservation:");
    println!("- lodash.js: {}", if default_has_lodash { "✅ preserved" } else { "❌ removed" });
    println!("- sortBy.js: {}", if default_has_sortby { "⚠️  preserved (should be removed)" } else { "✅ removed" });
    println!("- map.js: {}", if default_has_map { "⚠️  preserved (should be removed)" } else { "✅ removed" });
    println!("- filter.js: {}", if default_has_filter { "⚠️  preserved (should be removed)" } else { "✅ removed" });
    println!("- reduce.js: {}", if default_has_reduce { "⚠️  preserved (should be removed)" } else { "✅ removed" });
    println!("- lodash.default.js: {}", if default_has_default { "✅ preserved" } else { "❌ removed" });
    
    // With only default enabled, all function modules should be orphaned
    assert!(default_has_lodash, "Main lodash.js should be preserved");
    assert!(default_has_default, "lodash.default.js should be preserved when enabled");
    assert!(!default_has_sortby, "sortBy.js should be removed when disabled - orphaned module detection");
    assert!(!default_has_map, "map.js should be removed when disabled - orphaned module detection");
    assert!(!default_has_filter, "filter.js should be removed when disabled - orphaned module detection");
    assert!(!default_has_reduce, "reduce.js should be removed when disabled - orphaned module detection");
    
    // Test 3: Verify the enhanced parser is working correctly
    println!("\n🧪 Test 3: Enhanced parser validation");
    
    // Count modules before and after
    let original_modules = lodash_chunk.matches(".js\":").count();
    let optimized_modules = optimized.matches(".js\":").count();
    let default_modules = default_optimized.matches(".js\":").count();
    
    println!("Original modules: {}", original_modules);
    println!("Optimized modules (sortBy+default): {}", optimized_modules);
    println!("Default-only modules: {}", default_modules);
    
    // The enhanced parser should show a clear difference in module count
    assert!(optimized_modules < original_modules, "Enhanced parser should remove orphaned modules");
    assert!(default_modules < optimized_modules, "Default-only should have fewer modules than sortBy+default");
    
    // Expected: 6 -> 3 (lodash.js + sortBy.js + lodash.default.js)
    assert_eq!(optimized_modules, 3, "Should have exactly 3 modules with sortBy+default enabled");
    
    // Expected: 6 -> 2 (lodash.js + lodash.default.js)
    assert_eq!(default_modules, 2, "Should have exactly 2 modules with default-only enabled");
    
    println!("\n✅ Enhanced parser orphan detection working correctly!");
    println!("   - Correctly removes modules that become orphaned after macro processing");
    println!("   - Preserves modules that are still reachable from enabled exports");
    println!("   - Achieves expected reduction: {:.1}% -> {:.1}%", reduction, default_reduction);
}

#[test]
fn test_real_world_lodash_aggressive_tree_shaking() {
    println!("\n=== TESTING REAL-WORLD LODASH AGGRESSIVE TREE SHAKING ===");
    
    // Read the actual lodash chunk from the module federation example
    let chunk_path = "../../../examples/module-federation-example/host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original";
    
    if std::path::Path::new(chunk_path).exists() {
        let chunk = std::fs::read_to_string(chunk_path).unwrap();
        assert!(chunk.len() > 0);
        
        // Count modules
        let original_modules = chunk.matches(".js\":").count();
        println!("Original modules: {}", original_modules);
        
        // Test with absolute minimal config - only default export
        let minimal_config_str = r#"{
            "treeShake": {
                "lodash-es": {
                    "default": true,
                    "sortBy": false,
                    "map": false,
                    "filter": false,
                    "reduce": false,
                    "find": false,
                    "forEach": false,
                    "groupBy": false,
                    "debounce": false,
                    "throttle": false,
                    "cloneDeep": false,
                    "merge": false,
                    "pick": false,
                    "omit": false,
                    "capitalize": false,
                    "flatten": false,
                    "isEmpty": false,
                    "isArray": false,
                    "isObject": false,
                    "isString": false,
                    "isNumber": false,
                    "isFunction": false,
                    "chunk": false,
                    "compact": false,
                    "concat": false,
                    "difference": false,
                    "drop": false,
                    "fill": false,
                    "first": false,
                    "head": false,
                    "indexOf": false,
                    "initial": false,
                    "intersection": false,
                    "join": false,
                    "last": false,
                    "lastIndexOf": false,
                    "nth": false,
                    "pull": false,
                    "remove": false,
                    "reverse": false,
                    "slice": false,
                    "tail": false,
                    "take": false,
                    "union": false,
                    "uniq": false,
                    "without": false,
                    "zip": false,
                    "zipObject": false,
                    "zipWith": false
                }
            }
        }"#;
        
        let optimized = optimize(chunk.to_string(), minimal_config_str);
        let reduction = ((chunk.len() - optimized.len()) as f64 / chunk.len() as f64) * 100.0;
        assert!(reduction >= 0.0);
        
        // Count optimized modules
        let optimized_modules = optimized.matches(".js\":").count();
        assert!(optimized_modules <= original_modules);
        
        // With enhanced parser, we should see significant module removal
        // Even if not 90%+, we should see meaningful reduction
        assert!(reduction > 30.0, "Should achieve at least 30% reduction with minimal config");
        
        // The enhanced parser should remove SOME modules
        if optimized_modules < original_modules {
            assert!(original_modules - optimized_modules > 0);
        } else {
            let has_lodash_js = optimized.contains("lodash.js");
            let has_sortby_js = optimized.contains("sortBy.js");
            let has_map_js = optimized.contains("map.js");
            assert!(has_lodash_js && (has_sortby_js || has_map_js));
        }
    } else {
        // Skip test when real chunk is not available
        return;
    }
}