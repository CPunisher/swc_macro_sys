use std::fs;
use std::path::PathBuf;

use webpack_analyzer_v2::{analyzer::WebpackAnalyzer, chunk::ChunkCharacteristics, chunk::ShareUsageConfig, tree_shaker::TreeShaker};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

#[test]
fn host_react_dom_vendor_preserves_scheduler_dependency() {
    // Load real host react-dom vendor source (original)
    let root = repo_root();
    let src_path = root.join("webpack_analyzer_v2/test-cases/react_dom/host_react_dom.original.js");
    let usage_path = root.join("webpack_analyzer_v2/test-cases/react_dom/share-usage-host.json");
    if !src_path.exists() || !usage_path.exists() {
        eprintln!("Skipping test: fixtures missing at {} and {}", src_path.display(), usage_path.display());
        return;
    }

    let source = fs::read_to_string(&src_path).expect("read host react-dom vendor");
    let usage: serde_json::Value = serde_json::from_str(&fs::read_to_string(&usage_path).unwrap()).unwrap();

    // Pull chunk characteristics for react-dom from host share-usage
    let chars = usage
        .get("treeShake").and_then(|ts| ts.get("react-dom")).and_then(|rd| rd.get("chunk_characteristics")).cloned()
        .expect("react-dom.chunk_characteristics in host share-usage");
    let entry = chars.get("entry_module_id").and_then(|v| v.as_str()).expect("entry_module_id");
    let chunk_characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: chars.get("chunk_format").and_then(|v| v.as_str()).unwrap_or("jsonp").to_string(),
        chunk_loading_type: None,
        runtime_names: vec![],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec![],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: Some(entry.to_string()),
    };

    // Analyze and ensure the scheduler require is detected from react-dom impl module
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(&source, chunk_characteristics.clone()).expect("analysis failed");

    // Find react-dom implementation module id
    let react_dom_impl = chunk
        .modules
        .keys()
        .find(|mid| mid.contains("react-dom/cjs/react-dom.development.js") || mid.contains("react-dom/cjs/react-dom.production.min.js"))
        .cloned()
        .expect("react-dom implementation module present");
    let react_dom_impl_mod = chunk.modules.get(&react_dom_impl).expect("impl module");
    assert!(react_dom_impl_mod.dependencies.iter().any(|d| d.contains("scheduler/index.js")), "scheduler should be a dependency of react-dom impl");

    // Plan prune with minimal keep flags and assert both impl and scheduler stay kept
    let mut flags = serde_json::Map::new();
    flags.insert("createPortal".into(), serde_json::Value::Bool(true));
    let mut _cfg = serde_json::Map::new();
    _cfg.insert("react-dom".into(), serde_json::Value::Object(flags));
    let share_cfg = ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    };

    let shaker = TreeShaker::new();
    let plan = shaker.plan_prune(&chunk, &share_cfg);
    assert!(plan.kept_modules.contains(&react_dom_impl), "react-dom impl should be kept");
    assert!(plan.kept_modules.iter().any(|m| m.contains("scheduler/index.js")), "scheduler module should be kept if required by kept impl");
}


