use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod optimize;

// Console logging macro for WASM
macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format!($($t)*).into()))
}

/// Main optimization function that processes JavaScript code with conditional macros
/// and webpack-specific tree shaking using the new modular architecture.
/// 
/// This is the main entry point that delegates to our modular optimization pipeline.
#[wasm_bindgen]
pub fn optimize(source: String, config: &str) -> String {
    let config: serde_json::Value = match serde_json::from_str(config) {
        Ok(config) => config,
        Err(e) => {
            console_log!("❌ Failed to parse config: {}", e);
            return source; // Return original on error
        }
    };
    
    // Delegate to our modular optimization pipeline
    optimize::optimize_with_modular_pipeline(source, config)
}

/// Helper function to get optimization statistics (for testing/debugging)
#[wasm_bindgen]
pub fn get_optimization_info(source: String, config: &str) -> String {
    console_log!("📊 Getting optimization info without applying changes");
    
    let config: serde_json::Value = match serde_json::from_str(config) {
        Ok(config) => config,
        Err(e) => {
            return format!("{{\"error\": \"Failed to parse: {}\"}}", e);
        }
    };
    
    optimize::get_optimization_info(source, config)
}

/// Export version and build info
#[wasm_bindgen]
pub fn get_version_info() -> String {
    serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "architecture": "modular",
        "features": [
            "webpack_tree_shaking",
            "module_graph_analysis", 
            "mutation_tracking",
            "feature_analysis",
            "optimization_pipeline",
            "conditional_macros"
        ]
    }).to_string()
}


