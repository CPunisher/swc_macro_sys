use wasm_bindgen::prelude::*;

mod dce;
pub mod optimize;
pub mod webpack_parser;

pub use optimize::{optimize_with_prune_result, PruneResult};
use webpack_parser::WebpackChunkParser;

#[wasm_bindgen]
pub fn optimize(source: String, config: &str) -> String {
    let config: serde_json::Value =
        serde_json::from_str(config).expect("invalid config: must be a json object");
    optimize::optimize(source, config)
}

#[wasm_bindgen]
pub fn parse_webpack_chunk(content: &str) -> String {
    let parser = match WebpackChunkParser::new() {
        Ok(parser) => parser,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };
    
    match parser.parse_chunk_file(content) {
        Ok(chunk) => {
            let module_keys = parser.get_module_keys(&chunk);
            match serde_json::to_string(&serde_json::json!({
                "chunk_name": chunk.name,
                "module_keys": module_keys,
                "module_count": chunk.modules.len()
            })) {
                Ok(json) => json,
                Err(e) => format!("{{\"error\": \"{}\"}}", e),
            }
        }
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

#[wasm_bindgen]
pub fn get_webpack_module_info(content: &str, module_key: &str) -> String {
    let parser = match WebpackChunkParser::new() {
        Ok(parser) => parser,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };
    
    match parser.parse_chunk_file(content) {
        Ok(chunk) => {
            if let Some(module) = parser.get_module(&chunk, module_key) {
                match serde_json::to_string(module) {
                    Ok(json) => json,
                    Err(e) => format!("{{\"error\": \"{}\"}}", e),
                }
            } else {
                format!("{{\"error\": \"Module '{}' not found\"}}", module_key)
            }
        }
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

#[wasm_bindgen]
pub fn get_webpack_dependency_graph(content: &str) -> String {
    let parser = match WebpackChunkParser::new() {
        Ok(parser) => parser,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };
    
    match parser.parse_chunk_file(content) {
        Ok(chunk) => {
            let graph = parser.build_dependency_graph(&chunk);
            match serde_json::to_string(&graph) {
                Ok(json) => json,
                Err(e) => format!("{{\"error\": \"{}\"}}", e),
            }
        }
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

#[wasm_bindgen]
pub fn get_webpack_dependency_tree(content: &str, start_module_id: &str) -> String {
    let parser = match WebpackChunkParser::new() {
        Ok(parser) => parser,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };
    
    match parser.parse_chunk_file(content) {
        Ok(chunk) => {
            match parser.build_dependency_tree(&chunk, start_module_id) {
                Some(tree) => {
                    match serde_json::to_string(&tree) {
                        Ok(json) => json,
                        Err(e) => format!("{{\"error\": \"{}\"}}", e),
                    }
                }
                None => format!("{{\"error\": \"Failed to build dependency tree for module '{}'\"}}", start_module_id),
            }
        }
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

// New: WASM wrapper that returns both optimized source and prune result as JSON
#[wasm_bindgen]
pub fn optimize_with_prune_result_json(source: &str, config: &str) -> String {
    let config_value: serde_json::Value = match serde_json::from_str(config) {
        Ok(v) => v,
        Err(e) => return format!("{{\"error\": \"Invalid config: {}\"}}", e),
    };

    let (optimized_source, prune) = optimize::optimize_with_prune_result(source.to_string(), config_value);

    // Build a JSON object manually to avoid requiring Serialize on PruneResult
    let prune_json = serde_json::json!({
        "kept_modules": prune.kept_modules,
        "removed_modules": prune.removed_modules,
        "original_count": prune.original_count,
        "pruned_count": prune.pruned_count,
        "skip_reason": prune.skip_reason,
    });

    match serde_json::to_string(&serde_json::json!({
        "optimized_source": optimized_source,
        "prune_result": prune_json,
    })) {
        Ok(s) => s,
        Err(e) => format!("{{\"error\": \"Failed to serialize result: {}\"}}", e),
    }
}


