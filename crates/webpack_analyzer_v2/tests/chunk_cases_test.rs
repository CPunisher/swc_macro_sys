use webpack_analyzer_v2::*;
use std::fs;
use std::path::{Path, PathBuf};

fn read_fixture_from_candidates(repo_root: &Path, candidates: &[&str]) -> Option<(String, String)> {
    for rel in candidates {
        let p: PathBuf = repo_root.join(rel);
        if let Ok(content) = fs::read_to_string(&p) {
            let filename = p.file_name().unwrap().to_string_lossy().to_string();
            return Some((content, filename));
        }
    }
    None
}

#[test]
fn test_host_vendor_chunk() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let (chunk_content, filename) = read_fixture_from_candidates(
        repo_root,
        &[
            "examples/module-federation-example/host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js",
            // Prefer CJS fixture over JSONP to match async-node characteristics
            "test-cases/rspack-cjs-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js",
        ],
    ).expect("Required host vendor fixture not found");
    
    let analyzer = WebpackAnalyzer::new();
    
    // Host chunk characteristics from share-usage.json
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec![filename],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let result = analyzer.analyze_chunk(&chunk_content, characteristics);
    
    match result {
        Ok(chunk) => {
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            // Update for new enum variant - could be CommonJSAsync or CommonJSSync
            assert!(matches!(chunk.chunk_type, ChunkType::CommonJSAsync | ChunkType::CommonJSSync));
            assert!(chunk.module_count() > 0, "Should have extracted modules");
            // Dependency count is always non-negative by type definition
        }
        Err(e) => panic!("Failed to analyze host vendor chunk: {}", e),
    }
}

#[test]
fn test_remote_vendor_chunk() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let (chunk_content, filename) = read_fixture_from_candidates(
        repo_root,
        &[
            "examples/module-federation-example/remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js",
            // Prefer CJS fixture over JSONP to match async-node characteristics
            "test-cases/rspack-cjs-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js",
        ],
    ).expect("Required remote vendor fixture not found");
    
    let analyzer = WebpackAnalyzer::new();
    
    // Remote chunk characteristics from share-usage.json
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["remote".to_string(), "main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec![filename],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let result = analyzer.analyze_chunk(&chunk_content, characteristics);
    
    match result {
        Ok(chunk) => {
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            // Update for new enum variant - could be CommonJSAsync or CommonJSSync
            assert!(matches!(chunk.chunk_type, ChunkType::CommonJSAsync | ChunkType::CommonJSSync));
            assert!(chunk.module_count() > 0, "Should have extracted modules");
            // Dependency count is always non-negative by type definition
        }
        Err(e) => panic!("Failed to analyze remote vendor chunk: {}", e),
    }
}

#[test]
fn test_source_utils_chunk() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let maybe = read_fixture_from_candidates(
        repo_root,
        &[
            "examples/module-federation-example/remote/dist/src_utils_js.js",
            // Fallback to annotated output naming
            "test-cases/rspack-annotated-output/shared_utils_js.js",
        ],
    );
    let (chunk_content, filename) = match maybe { Some(v) => v, None => { eprintln!("skipping test_source_utils_chunk: no fixtures found"); return; } };
    
    let analyzer = WebpackAnalyzer::new();
    
    // Utils chunk characteristics (jsonp format based on actual file)
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "jsonp".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["remote".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec![filename],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let result = analyzer.analyze_chunk(&chunk_content, characteristics);
    
    match result {
        Ok(chunk) => {
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            // Check for JSONP format based on the actual test fixtures
            assert!(matches!(chunk.chunk_type, ChunkType::JSONP), "Expected JSONP chunk type, got {:?}", chunk.chunk_type);
            assert!(chunk.module_count() > 0, "Should have extracted modules from JSONP chunk");
            // Dependency count is always non-negative by type definition
        }
        Err(e) => panic!("Failed to analyze source utils chunk: {}", e),
    }
}

#[test]
fn test_source_button_chunk() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let maybe = read_fixture_from_candidates(
        repo_root,
        &[
            "examples/module-federation-example/remote/dist/src_Button_js.js",
            // Fallback to annotated output naming
            "test-cases/rspack-annotated-output/shared_components_js.js",
        ],
    );
    let (chunk_content, filename) = match maybe { Some(v) => v, None => { eprintln!("skipping test_source_button_chunk: no fixtures found"); return; } };
    
    let analyzer = WebpackAnalyzer::new();
    
    // Button chunk characteristics (jsonp format based on actual file)
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "jsonp".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["remote".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec![filename],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let result = analyzer.analyze_chunk(&chunk_content, characteristics);
    
    match result {
        Ok(chunk) => {
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            // Check for JSONP format based on the actual test fixtures
            assert!(matches!(chunk.chunk_type, ChunkType::JSONP), "Expected JSONP chunk type, got {:?}", chunk.chunk_type);
            assert!(chunk.module_count() > 0, "Should have extracted modules from JSONP chunk");
            // Dependency count is always non-negative by type definition
        }
        Err(e) => panic!("Failed to analyze source button chunk: {}", e),
    }
}

#[test]
fn test_chunk_characteristics_detection() {
    let analyzer = WebpackAnalyzer::new();
    
    // Test with async-node chunk format characteristics
    let async_node_characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["remote".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["vendors-lodash.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let commonjs_sample = r#"
    exports.ids = ["test"];
    exports.modules = {
        "module1": function() { return "test"; }
    };
    "#;
    
    let result = analyzer.analyze_chunk(commonjs_sample, async_node_characteristics);
    match result {
        Ok(chunk) => {
            assert_eq!(chunk.chunk_type, ChunkType::CommonJSAsync);
            assert!(chunk.characteristics.is_some());
            if let Some(chars) = &chunk.characteristics {
                assert_eq!(chars.chunk_format, "async-node");
                assert!(!chars.is_vendor_chunk()); // Should be false since can_be_initial is false
            }
        }
        Err(e) => panic!("Chunk characteristics detection failed: {}", e),
    }
    
    // Test with jsonp chunk format characteristics
    let jsonp_characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "jsonp".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let jsonp_sample = r#"
    (self["webpackChunktest"] = self["webpackChunktest"] || []).push([["test"], {
        "module1": function() { return "test"; }
    }]);
    "#;
    
    let result = analyzer.analyze_chunk(jsonp_sample, jsonp_characteristics);
    match result {
        Ok(chunk) => {
            assert_eq!(chunk.chunk_type, ChunkType::JSONP);
            assert!(chunk.characteristics.is_some());
            if let Some(chars) = &chunk.characteristics {
                assert_eq!(chars.chunk_format, "jsonp");
                assert!(chars.is_vendor_chunk()); // Should be true since can_be_initial is true
            }
        }
        Err(e) => panic!("Chunk characteristics detection failed: {}", e),
    }
}