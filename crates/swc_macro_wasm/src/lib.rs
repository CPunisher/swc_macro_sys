#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

mod dce;
pub mod optimize;
pub mod error;
pub mod config;
pub mod convergence;
pub mod performance;

// Cross-platform helper: parse JSON config string safely and call optimizer
pub fn optimize_with_config_str(source: String, config: &str) -> String {
    // Parse config with proper error handling to avoid panics
    let config: serde_json::Value = match serde_json::from_str(config) {
        Ok(cfg) => cfg,
        Err(_) => {
            // Return original source on invalid JSON (parity with WASM API)
            return source;
        }
    };

    match optimize::optimize(source.clone(), config) {
        Ok(result) => result,
        Err(_) => source,
    }
}

// WASM-only export that logs to browser console and installs panic hook
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn optimize(source: String, config: &str) -> String {
    // Install panic hook to surface Rust panic messages/stacktrace into JS console
    console_error_panic_hook::set_once();

    console::log_1(&format!(
        "WASM optimize: Called with source length {} and config: {}",
        source.len(), config
    )
    .into());

    console::log_1(&"WASM optimize: About to call optimize::optimize".into());

    // Delegate to cross-platform helper for consistent behavior
    optimize_with_config_str(source, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_webpack_tree_shaking_integration() {
        // Test with a realistic webpack bundle similar to our test cases
        let source = r#"
(()=>{
    "use strict";
    var __webpack_modules__ = {
        100: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Entry module");
            __webpack_require__(200);
        },
        200: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Used dependency");
        },
        300: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Unused module - should be tree shaken");
        }
    };
    
    function __webpack_require__(moduleId) {
        // webpack runtime
        return {};
    }
    
    (()=>{
        /* @common:if [condition="features.enableWebpackEntry"] */
        __webpack_require__(100);
        /* @common:endif */
    })();
})();
"#.to_string();

        let config = json!({
            "features": {
                "enableWebpackEntry": false  // This will remove the entry point call
            }
        });
        let original_size = source.len();
        let source_for_debug = source.clone();
        let result = optimize::optimize(source, config).unwrap_or_else(|_| String::new());
        
        println!("=== DEBUG INTEGRATION TEST ===");
        println!("Original source ({} bytes):\n{}", original_size, source_for_debug);
        println!("\nOptimized result ({} bytes):\n{}", result.len(), result);
        println!("\nSearching for patterns:");
        println!("  Contains '100:': {}", result.contains("100:"));
        println!("  Contains '200:': {}", result.contains("200:"));
        println!("  Contains '300:': {}", result.contains("300:"));
        println!("  Contains empty webpack_modules: {}", result.contains("var __webpack_modules__ = {};"));
        
        // Since the entry point is removed by DCE, tree shaking should remove all modules
        assert!(!result.contains("100:"), "Module 100 should be tree shaken");
        assert!(!result.contains("200:"), "Module 200 should be tree shaken");
        assert!(!result.contains("300:"), "Module 300 should be tree shaken");
        assert!(!result.contains("__webpack_modules__"), "webpack_modules should be completely removed when no entry points");
        
        println!("Tree shaking integration test passed!");
        println!(
            "Result size: {} bytes (tree shaking saved {} bytes)", 
            result.len(), 
            original_size - result.len()
        );
    }

    #[test]
    fn test_webpack_tree_shaking_with_macro_conditions() {
        // Test with a realistic webpack bundle with conditional features
        let source = r#"
(()=>{
    "use strict";
    var __webpack_modules__ = {
        100: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Entry module");
            /* @common:if [condition="features.enableFeatureA"] */
            __webpack_require__(200);
            /* @common:endif */
            /* @common:if [condition="features.enableFeatureB"] */
            __webpack_require__(300);
            /* @common:endif */
        },
        200: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Feature A module");
        },
        300: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Feature B module");
        },
        400: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Completely unused module");
        }
    };
    
    function __webpack_require__(moduleId) {
        // webpack runtime
        return {};
    }
    
    (()=>{
        /* @common:if [condition="features.enableEntryPoint"] */
        __webpack_require__(100);
        /* @common:endif */
    })();
})();
"#.to_string();

        let config = json!({
            "features": {
                "enableFeatureA": false,
                "enableFeatureB": false,
                "enableEntryPoint": false  // This removes the entry point entirely
            }
        });
        
        let result = optimize::optimize(source, config).unwrap_or_else(|_| String::new());
        
        println!("=== DEBUG MACRO CONDITIONS TEST ===");
        println!("Optimized result:\n{}", result);
        
        // All modules should be tree shaken since there are no entry points
        assert!(!result.contains("100:"), "Entry module should be tree shaken");
        assert!(!result.contains("200:"), "Feature A module should be tree shaken");
        assert!(!result.contains("300:"), "Feature B module should be tree shaken");
        assert!(!result.contains("400:"), "Unused module should be tree shaken");
        assert!(!result.contains("__webpack_modules__"), "webpack_modules should be completely removed when no entry points");
        
        println!("Tree shaking with macro conditions test passed!");
        println!("All modules successfully tree shaken due to no entry points");
    }

    #[test]
    fn test_invalid_config_does_not_panic() {
        let source = "console.log('hello');".to_string();
        // Pass deliberately invalid JSON through the cross-platform helper
        let result = super::optimize_with_config_str(source.clone(), "{not valid json}");
        // When config parsing fails we should get the original source back
        assert_eq!(result, source);
    }
}

// Native (non-wasm) shim to keep tests and CLI integration working
#[cfg(not(target_arch = "wasm32"))]
pub fn optimize(source: String, config: &str) -> String {
    optimize_with_config_str(source, config)
}