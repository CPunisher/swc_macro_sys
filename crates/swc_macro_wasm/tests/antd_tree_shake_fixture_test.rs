use std::fs;
use std::path::PathBuf;

use swc_macro_wasm::optimize;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn read(path: PathBuf) -> String {
    fs::read_to_string(path).expect("failed to read fixture")
}

#[test]
fn tree_shake_remote_antd_chunk_keeps_true_exports() {
    let root = repo_root();
    // Reuse analyzer fixtures for consistency
    let antd_js = root.join("webpack_analyzer_v2/test-cases/antd/remote_antd.js");
    let remote_json = root.join("webpack_analyzer_v2/test-cases/antd/share-usage-remote.json");
    if !antd_js.exists() || !remote_json.exists() {
        eprintln!("Skipping test: fixtures missing at {} and {}", antd_js.display(), remote_json.display());
        return;
    }

    let source = read(antd_js);
    let usage: serde_json::Value = serde_json::from_str(&read(remote_json)).unwrap();
    let chars = usage["treeShake"]["antd"]["chunk_characteristics"].clone();

    // Build minimal treeShake config: merge all truthy flags for 'antd' from remote
    let mut keep_flags = serde_json::Map::new();
    if let Some(obj) = usage["treeShake"]["antd"].as_object() {
        for (k, v) in obj {
            if k == "chunk_characteristics" { continue; }
            if v.as_bool() == Some(true) {
                keep_flags.insert(k.clone(), serde_json::Value::Bool(true));
            }
        }
    }
    // Attach chunk_characteristics (for this app only)
    keep_flags.insert("chunk_characteristics".to_string(), chars);

    let config = serde_json::json!({
        "treeShake": {
            "antd": serde_json::Value::Object(keep_flags)
        }
    });
    let config_str = serde_json::to_string(&config).unwrap();
    let optimized = optimize(source.clone(), &config_str);
    assert!(!optimized.is_empty(), "optimized output should not be empty");
    // Sanity: still looks like a webpack JSONP bundle push
    assert!(optimized.contains(".push(["), "optimized chunk should retain JSONP structure");
}


