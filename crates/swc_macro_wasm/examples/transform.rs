use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};

fn read_jsonp_dir() -> PathBuf {
    PathBuf::from("/Users/bytedance/dev/swc_macro_sys/tests/jsonp")
}

fn load_share_usage() -> Value {
    let path = read_jsonp_dir().join("share-usage.json");
    let data = fs::read_to_string(&path).expect("failed to read share-usage.json");
    serde_json::from_str(&data).expect("invalid JSON in share-usage.json")
}

fn load_chunk_file(file_name: &str) -> String {
    let path = read_jsonp_dir().join(file_name);
    fs::read_to_string(&path).expect("failed to read chunk file")
}

pub fn main() {
    // Read share usage data similar to the JS example
    let usage = load_share_usage();
    let tree_shake = usage
        .get("treeShake")
        .and_then(|v| v.as_object())
        .expect("share-usage.json must contain a treeShake object");

    // Iterate each package entry
    for (pkg_name, pkg_cfg) in tree_shake.iter() {
        if let Some(chars) = pkg_cfg.get("chunk_characteristics") {
            // Read chunk files for this package
            let chunk_files = chars
                .get("chunk_files")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            if chunk_files.is_empty() {
                continue;
            }

            let entry_module_id = chars
                .get("entry_module_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            // Use the first chunk for demonstration
            let first_chunk = chunk_files[0]
                .as_str()
                .unwrap_or("");
            if first_chunk.is_empty() {
                continue;
            }

            let source = load_chunk_file(first_chunk);

            // Build config that includes only this package and its chunk characteristics
            let mut module_cfg = pkg_cfg.clone();
            if let Some(obj) = module_cfg.as_object_mut() {
                obj.insert("chunk_characteristics".into(), chars.clone());
            }
            let mut tree_shake_obj = serde_json::Map::new();
            tree_shake_obj.insert(pkg_name.clone(), module_cfg);
            let config = Value::Object(serde_json::Map::from_iter([
                ("treeShake".into(), Value::Object(tree_shake_obj)),
            ]));

            let optimized = swc_macro_wasm::optimize::optimize(source, config);

            println!(
                "Optimized chunk for {} (entry_module_id: {}), length: {} bytes",
                pkg_name,
                entry_module_id,
                optimized.len()
            );
        }
    }

    // Also demonstrate a simple macro transform call as before
    let demo_source = "console.log('demo');".to_string();
    let demo_config = json!({
        "build": {"target": "production"},
        "featureFlags": {"someFlag": true}
    });

    let _ = swc_macro_wasm::optimize::optimize(demo_source, demo_config);
}
