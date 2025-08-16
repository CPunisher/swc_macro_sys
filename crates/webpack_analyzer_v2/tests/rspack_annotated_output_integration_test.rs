use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use swc_core::atoms::Atom;

use webpack_analyzer_v2::{ChunkCharacteristics, ChunkType, ShareUsageConfig, TreeShaker, WebpackAnalyzer};

fn repo_path(segments: &[&str]) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for s in ["..", ".."].into_iter() { p.push(s); }
    for seg in segments { p.push(seg); }
    p
}

/// Load ShareUsageConfig from share-usage.json file
fn load_share_usage_config(config_path: &PathBuf) -> Result<ShareUsageConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(config_path)?;
    ShareUsageConfig::from_json(&content)
}

#[test]
fn test_rspack_annotated_output_jsonp_analysis() {
    // Pick a JSONP-like annotated output chunk with modules object inside push
    let chunk_path = repo_path(&[
        "test-cases",
        "rspack-annotated-output",
        "vendors-node_modules_pnpm_react_19_1_0_node_modules_react_index_js.js",
    ]);

    if !chunk_path.exists() {
        // Some trees may not include this exact file; skip gracefully
        return;
    }

    let source = fs::read_to_string(&chunk_path).expect("failed to read rspack annotated output react chunk");

    // Load share-usage (if present) to get characteristics; otherwise construct minimal JSONP characteristics
    let usage_path = repo_path(&["test-cases", "rspack-annotated-output", "share-usage.json"]);
    let mut entry_module_id: Option<String> = None;
    if usage_path.exists() {
        if let Ok(val) = serde_json::from_str::<Value>(&fs::read_to_string(&usage_path).unwrap()) {
            if let Some(entry) = val
                .pointer("/treeShake/react-dom/chunk_characteristics/entry_module_id")
                .and_then(|v| v.as_str())
            {
                entry_module_id = Some(entry.to_string());
            }
        }
    }

    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "jsonp".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: true,
        chunk_files: vec![chunk_path.file_name().unwrap().to_string_lossy().to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id,
    };

    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer
        .analyze_chunk(&source, characteristics)
        .expect("analyze rspack annotated jsonp chunk");

    assert_eq!(chunk.chunk_type, ChunkType::JSONP);
    assert!(chunk.module_count() > 0);

    // Load ShareUsageConfig from share-usage.json
    let usage_path = repo_path(&["test-cases", "rspack-annotated-output", "share-usage.json"]);
    let config = if usage_path.exists() {
        load_share_usage_config(&usage_path).unwrap_or_else(|_| ShareUsageConfig { 
            entry_module_ids: vec![],
            tree_shake: std::collections::HashMap::new(),
        })
    } else {
        ShareUsageConfig { 
            entry_module_ids: vec![],
            tree_shake: std::collections::HashMap::new(),
        }
    };
    
    // Plan prune using configuration-driven approach
    let shaker = TreeShaker::new();
    let plan = shaker.plan_prune(&chunk, &config);
    
    // Should skip if no entry module IDs in config or entry not present in chunk
    if config.entry_module_ids.is_empty() {
        assert!(plan.skip_reason.is_some(), "Should skip when no entry module IDs configured");
    }
}


