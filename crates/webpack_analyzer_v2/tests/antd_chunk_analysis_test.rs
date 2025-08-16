use std::fs;
use std::path::PathBuf;

use webpack_analyzer_v2::{chunk::ChunkCharacteristics, analyzer::WebpackAnalyzer, tree_shaker::TreeShaker};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn read(path: PathBuf) -> String {
    fs::read_to_string(path).expect("failed to read fixture")
}

#[test]
fn analyze_and_prune_remote_antd_chunk() {
    let root = repo_root();
    let antd_js = root.join("webpack_analyzer_v2/test-cases/antd/remote_antd.js");
    let remote_json = root.join("webpack_analyzer_v2/test-cases/antd/share-usage-remote.json");
    if !antd_js.exists() || !remote_json.exists() {
        eprintln!("Skipping test: fixtures missing at {} and {}", antd_js.display(), remote_json.display());
        return;
    }

    let source = read(antd_js);
    let usage: serde_json::Value = serde_json::from_str(&read(remote_json)).unwrap();
    let chars = usage["treeShake"]["antd"]["chunk_characteristics"].clone();
    let characteristics: ChunkCharacteristics = serde_json::from_value(chars).unwrap();

    // Analyze
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(&source, characteristics.clone()).expect("analysis failed");
    assert!(chunk.modules.len() > 0, "should extract modules from antd chunk");

    // Plan prune conservatively using characteristics.entry_module_id
    let shaker = TreeShaker::new();
    let plan = shaker.plan_prune(&chunk, &webpack_analyzer_v2::chunk::ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    });
    assert!(plan.skip_reason.is_none(), "tree shaker should not skip with valid characteristics");
    assert!(plan.original_count >= plan.pruned_count, "pruned count should be <= original");
}


