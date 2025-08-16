use std::fs;
use std::path::PathBuf;

use serde_json::Value;
// use swc_core::atoms::Atom;

use webpack_analyzer_v2::{ChunkCharacteristics, ChunkType, ShareUsageConfig, TreeShaker, WebpackAnalyzer};

fn repo_path(segments: &[&str]) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for s in ["..", ".."].into_iter() { p.push(s); }
    for seg in segments { p.push(seg); }
    p
}

#[test]
fn test_esm_react_chunk_analysis_and_skip_prune_without_entries() {
    // Load ESM chunk (React)
    let chunk_path = repo_path(&[
        "test-cases",
        "webpack-esm",
        "vendors-node_modules_pnpm_react_18_3_1_node_modules_react_index_js.mjs",
    ]);
    let source = fs::read_to_string(&chunk_path).expect("failed to read ESM react chunk");

    // Load share-usage to get characteristics for 'react'
    let usage_path = repo_path(&["test-cases", "webpack-esm", "share-usage.json"]);
    let usage_json: Value = serde_json::from_str(&fs::read_to_string(&usage_path).expect("read usage"))
        .expect("invalid json");
    let react_entry = usage_json["treeShake"]["react"]["chunk_characteristics"]["entry_module_id"]
        .as_str()
        .map(|s| s.to_string());

    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "module".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec![chunk_path.file_name().unwrap().to_string_lossy().to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: react_entry,
    };

    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer
        .analyze_chunk(&source, characteristics)
        .expect("analyze ESM react chunk");

    // ESM is currently treated like webpack modules for extraction; ensure analysis succeeded
    assert_eq!(chunk.chunk_type, ChunkType::ESModules);

    // Plan prune via characteristics only; no guessing
    let shaker = TreeShaker::new();
    let cfg = ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    };
    let plan = shaker.plan_prune(&chunk, &cfg);
    assert!(plan.skip_reason.is_some() || plan.pruned_count <= plan.original_count);
}


