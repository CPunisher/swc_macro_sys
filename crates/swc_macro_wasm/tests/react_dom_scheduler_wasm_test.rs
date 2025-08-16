use std::fs;
use std::path::PathBuf;

use swc_macro_wasm::optimize;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

#[test]
fn wasm_optimize_preserves_scheduler_in_react_dom_vendor() {
    // Use remote vendor original fixture
    let root = repo_root();
    let src_path = root.join("webpack_analyzer_v2/test-cases/react_dom/remote_react_dom.original.js");
    let usage_path = root.join("webpack_analyzer_v2/test-cases/react_dom/share-usage-remote.json");
    if !src_path.exists() || !usage_path.exists() {
        eprintln!("Skipping test: fixtures missing at {} and {}", src_path.display(), usage_path.display());
        return;
    }

    let source = fs::read_to_string(&src_path).expect("read react-dom vendor");
    let usage: serde_json::Value = serde_json::from_str(&fs::read_to_string(&usage_path).unwrap()).unwrap();

    // Build config: only boolean flags and this chunk's characteristics
    let rd = usage.get("treeShake").and_then(|t| t.get("react-dom")).expect("react-dom in share-usage");
    let mut keep_flags = serde_json::Map::new();
    if let Some(obj) = rd.as_object() {
        for (k, v) in obj {
            if k == "chunk_characteristics" { continue; }
            if v.as_bool() == Some(true) { keep_flags.insert(k.clone(), serde_json::Value::Bool(true)); }
        }
    }
    let characteristics = rd.get("chunk_characteristics").cloned().expect("chunk_characteristics");
    keep_flags.insert("chunk_characteristics".into(), characteristics);
    let config = serde_json::json!({ "treeShake": { "react-dom": serde_json::Value::Object(keep_flags) } });
    let config_str = serde_json::to_string(&config).unwrap();

    let optimized = optimize(source.clone(), &config_str);
    assert!(!optimized.is_empty(), "optimized output should not be empty");

    // Assert that both the require and the module definition for scheduler remain
    assert!(optimized.contains("__webpack_require__(\"../../../node_modules/.pnpm/scheduler@0.23.2/node_modules/scheduler/index.js\")")
        || optimized.contains("__webpack_require__(\"../../../node_modules/.pnpm/scheduler@0.23.2/node_modules/scheduler/index.js\")"),
        "optimized should still contain require to scheduler/index.js");
    assert!(optimized.contains("\"../../../node_modules/.pnpm/scheduler@0.23.2/node_modules/scheduler/index.js\":")
        || optimized.contains("'../../../node_modules/.pnpm/scheduler@0.23.2/node_modules/scheduler/index.js':"),
        "optimized should still define scheduler/index.js module key");
}


