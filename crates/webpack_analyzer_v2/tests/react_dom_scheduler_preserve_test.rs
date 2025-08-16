use std::fs;
use std::path::PathBuf;

use webpack_analyzer_v2::{analyzer::WebpackAnalyzer, chunk::ChunkCharacteristics, chunk::ShareUsageConfig, tree_shaker::TreeShaker};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

#[test]
fn react_dom_vendor_preserves_scheduler_dependency() {
    // Load real remote react-dom vendor source (original)
    let root = repo_root();
    let src_path = root.join("webpack_analyzer_v2/test-cases/react_dom/remote_react_dom.original.js");
    let usage_path = root.join("webpack_analyzer_v2/test-cases/react_dom/share-usage-remote.json");
    if !src_path.exists() || !usage_path.exists() {
        eprintln!("Skipping test: fixtures missing at {} and {}", src_path.display(), usage_path.display());
        return;
    }

    let source = fs::read_to_string(&src_path).expect("read react-dom vendor");
    let usage: serde_json::Value = serde_json::from_str(&fs::read_to_string(&usage_path).unwrap()).unwrap();
    let chars = usage["treeShake"]["react-dom"]["chunk_characteristics"].clone();
    let characteristics: ChunkCharacteristics = serde_json::from_value(chars).unwrap();

    // Analyze and ensure the scheduler require is detected
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(&source, characteristics.clone()).expect("analysis failed");

    // Find react-dom implementation module and assert it depends on scheduler
    let react_dom_impl = chunk
        .modules
        .keys()
        .find(|m| m.as_ref().contains("react-dom/cjs/react-dom.development.js"))
        .cloned()
        .expect("react-dom impl not found in vendor chunk");
    let impl_mod = chunk.modules.get(&react_dom_impl).unwrap();
    assert!(impl_mod
        .dependencies
        .iter()
        .any(|d| d.as_ref().contains("scheduler/index.js") || d.as_ref().contains("/scheduler@") ),
        "react-dom impl should have a dependency on scheduler");

    // In the remote vendor, scheduler module id is NOT defined inside this chunk.
    // The tree shaker must NOT remove the react-dom module that requires it; it should be kept as entry reachable.
    let shaker = TreeShaker::new();
    let plan = shaker.plan_prune(&chunk, &ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    });
    assert!(plan.skip_reason.is_none(), "tree shaker should run with characteristics");

    // Entry must be kept
    let entry_id = characteristics.entry_module_id.clone().unwrap();
    assert!(plan.kept_modules.iter().any(|m| m.as_ref() == entry_id), "entry react-dom should be kept");

    // Ensure that react-dom implementation module that does __webpack_require__(scheduler) is also kept
    assert!(plan.kept_modules.contains(&react_dom_impl), "react-dom implementation must be kept despite scheduler dep");
}


