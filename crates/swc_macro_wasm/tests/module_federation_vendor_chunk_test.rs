use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_module_federation_vendor_chunk_preserves_exports() {
    println!("\n=== MODULE FEDERATION VENDOR CHUNK TEST ===");
    
    // Create a minimal vendor chunk that mimics the lodash structure
    let vendor_chunk = r#"
"use strict";
exports.ids = ["vendors-node_modules_lodash-es_lodash_js"];
exports.modules = {
    "node_modules/lodash-es/sortBy.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => sortBy
        });
        function sortBy(collection, iteratee) {
            // sortBy implementation
            return collection;
        }
    },
    "node_modules/lodash-es/uniq.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => uniq
        });
        function uniq(array) {
            // uniq implementation
            return array;
        }
    },
    "node_modules/lodash-es/filter.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => filter
        });
        function filter(collection, predicate) {
            // filter implementation
            return collection;
        }
    },
    "node_modules/lodash-es/lodash.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            sortBy: () => (/* @common:if [condition="treeShake.lodash-es.sortBy"] */ _sortBy["default"] /* @common:endif */),
            uniq: () => (/* @common:if [condition="treeShake.lodash-es.uniq"] */ _uniq["default"] /* @common:endif */),
            filter: () => (/* @common:if [condition="treeShake.lodash-es.filter"] */ _filter["default"] /* @common:endif */)
        });
        var _sortBy = __webpack_require__("node_modules/lodash-es/sortBy.js");
        var _uniq = __webpack_require__("node_modules/lodash-es/uniq.js");
        var _filter = __webpack_require__("node_modules/lodash-es/filter.js");
    }
};
"#;
    
    // Config: keep sortBy and uniq, remove filter
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": true,
                "uniq": true,
                "filter": false,
                "chunk_characteristics": {
                    "entry_module_id": "node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }
        }
    });
    
    println!("Testing vendor chunk optimization with tree shake config");
    println!("Config: keep sortBy and uniq, remove filter");
    
    let optimized = optimize(vendor_chunk.to_string(), &config.to_string());
    
    // Debug: print just the lodash.js module to see exports
    if let Some(lodash_start) = optimized.find("lodash.js") {
        let from_lodash = &optimized[lodash_start..];
        if let Some(end) = from_lodash.find("\n    },\n    \"") {
            println!("\nOptimized lodash.js module:\n{}", &from_lodash[..end]);
        } else if let Some(end) = from_lodash.find("\n};\n") {
            println!("\nOptimized lodash.js module:\n{}", &from_lodash[..end]);
        }
    }
    
    println!("\nOptimization results:");
    println!("  Original size: {} bytes", vendor_chunk.len());
    println!("  Optimized size: {} bytes", optimized.len());
    println!("  Reduction: {:.2}%", 
        (vendor_chunk.len() - optimized.len()) as f64 / vendor_chunk.len() as f64 * 100.0);
    
    // Verify what remains
    println!("\nVerifying remaining modules:");
    
    // The main lodash.js module should always be preserved
    let has_main_lodash = optimized.contains("lodash-es/lodash.js");
    println!("  Main lodash.js module: {}", if has_main_lodash { "✅ Present" } else { "❌ Missing" });
    
    // Check for individual function modules
    let has_sortby = optimized.contains("sortBy.js");
    let has_uniq = optimized.contains("uniq.js");
    let has_filter = optimized.contains("filter.js");
    
    println!("  sortBy.js module: {}", if has_sortby { "✅ Present" } else { "❌ Missing" });
    println!("  uniq.js module: {}", if has_uniq { "✅ Present" } else { "❌ Missing" });
    println!("  filter.js module: {}", if has_filter { "Present (should be removed)" } else { "✅ Removed" });
    
    // Check exports
    println!("\nVerifying exports:");
    let exports_sortby = optimized.contains("sortBy:") && !optimized.contains("sortBy: ()=>null");
    let exports_uniq = optimized.contains("uniq:") && !optimized.contains("uniq: ()=>null");
    let exports_filter = optimized.contains("filter:") && !optimized.contains("filter: ()=>null");
    
    println!("  sortBy export: {}", if exports_sortby { "✅ Active" } else { "❌ Removed/Nullified" });
    println!("  uniq export: {}", if exports_uniq { "✅ Active" } else { "❌ Removed/Nullified" });
    println!("  filter export: {}", if exports_filter { "Active (should be nullified)" } else { "✅ Removed/Nullified" });
    
    // Critical assertions
    assert!(has_main_lodash, "Main lodash.js module must be preserved!");
    assert!(exports_sortby, "sortBy export should be active");
    assert!(exports_uniq, "uniq export should be active");
    assert!(!has_filter || !exports_filter, "filter should be removed or nullified");
    
    // The optimization should not be too aggressive
    if optimized.len() < 500 {
        println!("\n⚠️  WARNING: Optimization seems too aggressive!");
        println!("Optimized chunk:\n{}", optimized);
    }
    
    println!("\n✅ Module Federation vendor chunk test completed");
}

#[test]
fn test_vendor_chunk_with_usage_data() {
    println!("\n=== VENDOR CHUNK WITH REAL USAGE DATA TEST ===");
    
    // Load a real vendor chunk structure
    let vendor_chunk = include_str!("fixtures/module_federation_lodash_chunk.js");
    
    // Simulate usage: only sortBy and uniq are used
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": true,
                "uniq": true,
                "default": true,
                "capitalize": false,
                "debounce": false,
                "filter": false,
                "groupBy": false,
                "map": false,
                "omit": false,
                "pick": false,
                "throttle": false,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }
        }
    });
    
    println!("Testing with real vendor chunk");
    println!("Used exports: sortBy, uniq, default");
    println!("Unused exports: capitalize, debounce, filter, groupBy, map, omit, pick, throttle");
    
    let original_size = vendor_chunk.len();
    let optimized = optimize(vendor_chunk.to_string(), &config.to_string());
    let optimized_size = optimized.len();
    
    println!("\nOptimization results:");
    println!("  Original size: {} bytes ({:.1} KB)", original_size, original_size as f64 / 1024.0);
    println!("  Optimized size: {} bytes ({:.1} KB)", optimized_size, optimized_size as f64 / 1024.0);
    println!("  Reduction: {:.2}%", (original_size - optimized_size) as f64 / original_size as f64 * 100.0);
    
    // Count remaining modules
    let module_count = optimized.matches(".js\":").count();
    println!("\nModules remaining: {}", module_count);
    
    // The main lodash.js export module must be preserved
    let has_lodash_exports = optimized.contains("lodash-es/lodash.js");
    println!("Main lodash.js module: {}", if has_lodash_exports { "✅ Present" } else { "❌ Missing" });
    
    // Verify critical structure
    assert!(optimized.contains("exports.modules"), "Webpack module structure must be preserved");
    assert!(has_lodash_exports, "Main lodash export module must be preserved");
    
    // Reasonable size check - it shouldn't be reduced to just 1KB
    if optimized_size < 10000 {  // Less than 10KB seems too small
        println!("\n⚠️  WARNING: Optimization might be too aggressive!");
        println!("Expected to preserve at least sortBy, uniq, and their dependencies");
        
        // Check what modules remain
        let lines: Vec<&str> = optimized.lines().collect();
        let module_lines: Vec<&str> = lines.iter()
            .filter(|line| line.contains(".js\":"))
            .copied()
            .collect();
        
        println!("\nRemaining modules:");
        for line in module_lines.iter().take(10) {
            println!("  {}", line);
        }
    }
    
    println!("\n✅ Vendor chunk with usage data test completed");
}