use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use serde_json::Value;
use swc_core::atoms::Atom;

use webpack_analyzer_v2::{
    ChunkCharacteristics, ChunkType, ShareUsageConfig, TreeShaker, WebpackAnalyzer,
    tree_shaker::SplitChunkOptimizer
};

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

/// Extract chunk characteristics for a specific library from share-usage.json
fn extract_chunk_characteristics(config_path: &PathBuf, lib_name: &str) -> Option<ChunkCharacteristics> {
    let content = fs::read_to_string(config_path).ok()?;
    let json: Value = serde_json::from_str(&content).ok()?;
    
    let lib_config = json
        .get("treeShake")?
        .get(lib_name)?
        .get("chunk_characteristics")?;
    
    serde_json::from_value(lib_config.clone()).ok()
}

/// Get export usage flags for a specific library
fn get_export_usage_flags(config_path: &PathBuf, lib_name: &str) -> HashMap<String, bool> {
    let mut usage_flags = HashMap::new();
    
    if let Ok(content) = fs::read_to_string(config_path) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            if let Some(lib_config) = json.get("treeShake").and_then(|v| v.get(lib_name)) {
                if let Some(obj) = lib_config.as_object() {
                    for (key, value) in obj {
                        if key != "chunk_characteristics" {
                            if let Some(bool_val) = value.as_bool() {
                                usage_flags.insert(key.clone(), bool_val);
                            }
                        }
                    }
                }
            }
        }
    }
    
    usage_flags
}

#[test]
fn test_configuration_schema_validation() {
    let config_path = repo_path(&["test-cases", "rspack-annotated-output", "share-usage.json"]);
    
    // Test that we can successfully load and parse the configuration
    let config = load_share_usage_config(&config_path)
        .expect("Should be able to load share-usage.json configuration");
    
    // Verify that entry module IDs were extracted
    assert!(!config.entry_module_ids.is_empty(), "Configuration should contain entry module IDs");
    
    // Verify specific libraries are present
    let entry_ids: Vec<String> = config.entry_module_ids.iter().map(|id| id.to_string()).collect();
    assert!(entry_ids.iter().any(|id| id.contains("lodash-es")), "Should contain lodash-es entry");
    assert!(entry_ids.iter().any(|id| id.contains("react")), "Should contain react entry");
}

#[test]
fn test_chunk_characteristics_extraction() {
    let config_path = repo_path(&["test-cases", "rspack-annotated-output", "share-usage.json"]);
    
    // Test extracting chunk characteristics for lodash-es
    let lodash_chars = extract_chunk_characteristics(&config_path, "lodash-es")
        .expect("Should extract lodash-es chunk characteristics");
    
    assert!(!lodash_chars.is_runtime_chunk, "Lodash should not be a runtime chunk");
    assert!(!lodash_chars.chunk_files.is_empty(), "Should have chunk files specified");
    assert!(lodash_chars.entry_module_id.is_some(), "Should have entry module ID");
    
    // Test extracting chunk characteristics for react-dom
    let react_dom_chars = extract_chunk_characteristics(&config_path, "react-dom")
        .expect("Should extract react-dom chunk characteristics");
    
    assert!(!react_dom_chars.is_runtime_chunk, "React-dom should not be a runtime chunk");
    assert_eq!(react_dom_chars.chunk_format, "async-node", "Should have correct chunk format");
}

#[test]
fn test_export_usage_flags_parsing() {
    let config_path = repo_path(&["test-cases", "rspack-annotated-output", "share-usage.json"]);
    
    // Test lodash-es export usage flags
    let lodash_flags = get_export_usage_flags(&config_path, "lodash-es");
    assert!(lodash_flags.get("map").copied().unwrap_or(false), "map should be marked as used");
    assert!(lodash_flags.get("filter").copied().unwrap_or(false), "filter should be marked as used");
    assert!(!lodash_flags.get("add").copied().unwrap_or(true), "add should be marked as unused");
    assert!(!lodash_flags.get("after").copied().unwrap_or(true), "after should be marked as unused");
    
    // Test utility-lib export usage flags
    let utility_flags = get_export_usage_flags(&config_path, "utility-lib");
    assert!(utility_flags.get("capitalize").copied().unwrap_or(false), "capitalize should be marked as used");
    assert!(utility_flags.get("formatDate").copied().unwrap_or(false), "formatDate should be marked as used");
    assert!(!utility_flags.get("debounce").copied().unwrap_or(true), "debounce should be marked as unused");
}

#[test]
fn test_configuration_driven_chunk_identification() {
    let config_path = repo_path(&["test-cases", "rspack-annotated-output", "share-usage.json"]);
    let config = load_share_usage_config(&config_path)
        .expect("Should load configuration");
    
    // Test lodash chunk
    let lodash_chunk_path = repo_path(&[
        "test-cases",
        "rspack-annotated-output",
        "vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js",
    ]);
    
    if lodash_chunk_path.exists() {
        let source = fs::read_to_string(&lodash_chunk_path)
            .expect("Should read lodash chunk");
        
        let characteristics = extract_chunk_characteristics(&config_path, "lodash-es")
            .expect("Should extract lodash characteristics");
        
        let analyzer = WebpackAnalyzer::new();
        let chunk = analyzer.analyze_chunk(&source, characteristics.clone())
            .expect("Should analyze lodash chunk");
        
        // Test split chunk optimizer with configuration
        let split_optimizer = SplitChunkOptimizer::new();
        let should_process = split_optimizer.should_process_chunk(&chunk);
        
        // Lodash vendor chunk should be processed
        assert!(should_process, "Lodash vendor chunk should be identified for processing");
        
        // Test tree shaker with configuration
        let shaker = TreeShaker::new();
        let plan = shaker.plan_prune(&chunk, &config);
        
        // Should not skip if entry module is present
        if let Some(entry_id) = &characteristics.entry_module_id {
            if chunk.modules.contains_key(&Atom::from(entry_id.as_str())) {
                assert!(plan.skip_reason.is_none(), "Should not skip when entry module is present");
            }
        }
    }
}

#[test]
fn test_runtime_chunk_preservation() {
    let config_path = repo_path(&["test-cases", "rspack-annotated-output", "share-usage.json"]);
    
    // Create a mock runtime chunk characteristics
    let runtime_characteristics = ChunkCharacteristics {
        is_runtime_chunk: true,
        has_runtime: true,
        is_entrypoint: true,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "webpack".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["runtime".to_string()],
        entry_name: Some("runtime".to_string()),
        has_async_chunks: false,
        chunk_files: vec!["runtime.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: Some("./src/runtime.js".to_string()),
    };
    
    // Create a mock chunk with runtime characteristics
    let mock_source = r#"
        var __webpack_modules__ = ({
            "./src/runtime.js": function(module, exports, __webpack_require__) {
                // Runtime code
            }
        });
    "#;
    
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(mock_source, runtime_characteristics)
        .expect("Should analyze runtime chunk");
    
    // Test that split chunk optimizer skips runtime chunks
    let split_optimizer = SplitChunkOptimizer::new();
    let should_process = split_optimizer.should_process_chunk(&chunk);
    assert!(!should_process, "Runtime chunks should not be processed by split chunk optimizer");
    
    // Test that tree shaker skips runtime chunks
    let config = load_share_usage_config(&config_path)
        .expect("Should load configuration");
    let shaker = TreeShaker::new();
    let plan = shaker.plan_prune(&chunk, &config);
    assert!(plan.skip_reason.is_some(), "Tree shaker should skip runtime chunks");
    assert!(plan.skip_reason.unwrap().contains("runtime"), "Skip reason should mention runtime");
}

#[test]
fn test_configuration_driven_optimization_integration() {
    let config_path = repo_path(&["test-cases", "rspack-cjs-annotated-output", "share-usage.json"]);
    
    if !config_path.exists() {
        return; // Skip if test case doesn't exist
    }
    
    let config = load_share_usage_config(&config_path)
        .expect("Should load CJS configuration");
    
    let lodash_chunk_path = repo_path(&[
        "test-cases",
        "rspack-cjs-annotated-output",
        "vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js",
    ]);
    
    if lodash_chunk_path.exists() {
        let source = fs::read_to_string(&lodash_chunk_path)
            .expect("Should read CJS lodash chunk");
        
        let characteristics = extract_chunk_characteristics(&config_path, "lodash-es")
            .expect("Should extract CJS lodash characteristics");
        
        let analyzer = WebpackAnalyzer::new();
        let _chunk = analyzer.analyze_chunk(&source, characteristics.clone())
            .expect("Should analyze CJS lodash chunk");
        
        // Test the full optimization pipeline
        let shaker = TreeShaker::new();
        
        // Test source-level optimization
        let (optimized_source, plan) = shaker.prune_source(&source, &characteristics)
            .expect("Should optimize source");
        
        if plan.skip_reason.is_none() {
            // Verify optimization occurred
            assert!(optimized_source.len() <= source.len(), "Optimized source should not be larger");
            
            if !plan.removed_modules.is_empty() {
                assert!(optimized_source.len() < source.len(), "Should have reduced size when modules removed");
                println!("Optimization removed {} modules ({}% reduction)", 
                    plan.removed_modules.len(),
                    (plan.removed_modules.len() * 100) / plan.original_count.max(1)
                );
            }
        }
    }
}