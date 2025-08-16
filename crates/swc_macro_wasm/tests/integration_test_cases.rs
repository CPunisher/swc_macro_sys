use std::fs;
use std::path::Path;
use swc_macro_wasm::optimize;
use serde_json::json;

/// Integration test using real test-cases with share-usage.json configurations
#[test]
fn test_rspack_annotated_output_integration() {
    println!("\n=== RSPACK ANNOTATED OUTPUT INTEGRATION TEST ===");
    
    let test_dir = Path::new("../../test-cases/rspack-annotated-output");
    let share_usage_path = test_dir.join("share-usage.json");
    let lodash_chunk_path = test_dir.join("vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    
    assert!(share_usage_path.exists(), "share-usage.json not found in rspack-annotated-output");
    assert!(lodash_chunk_path.exists(), "lodash chunk not found in rspack-annotated-output");
    
    // Load configuration
    let share_usage_content = fs::read_to_string(&share_usage_path)
        .expect("Failed to read share-usage.json");
    let share_usage: serde_json::Value = serde_json::from_str(&share_usage_content)
        .expect("Failed to parse share-usage.json");
    
    // Extract lodash configuration
    let lodash_config = &share_usage["treeShake"]["lodash-es"];
    assert!(lodash_config.is_object(), "lodash-es configuration should be an object");
    
    // Create test configuration
    let config = json!({
        "treeShake": {
            "lodash-es": lodash_config
        }
    });
    
    // Load and optimize lodash chunk
    let original_chunk = fs::read_to_string(&lodash_chunk_path)
        .expect("Failed to read lodash chunk");
    
    println!("Original chunk size: {} bytes", original_chunk.len());
    println!("Configuration loaded with {} exports", 
        lodash_config.as_object().unwrap().len() - 1); // -1 for chunk_characteristics
    
    let optimized_chunk = optimize(original_chunk.clone(), &config.to_string());
    
    let reduction = ((original_chunk.len() - optimized_chunk.len()) as f64 / original_chunk.len() as f64) * 100.0;
    
    println!("Optimized chunk size: {} bytes", optimized_chunk.len());
    println!("Size reduction: {:.2}%", reduction);
    
    // Validate optimization results
    assert!(optimized_chunk.len() > 0, "Optimized chunk should not be empty");
    assert!(optimized_chunk.len() < original_chunk.len(), "Optimization should reduce size");
    assert!(reduction > 10.0, "Should achieve at least 10% reduction");
    
    // Check that module structure is preserved (JSONP format uses different structure)
    let has_jsonp_structure = optimized_chunk.contains(".push(") && optimized_chunk.contains("webpackChunk");
    let has_cjs_structure = optimized_chunk.contains("exports.modules");
    assert!(has_jsonp_structure || has_cjs_structure, "Module structure should be preserved (JSONP or CJS format)");
    
    println!("✅ RSPACK annotated output integration test passed");
}

#[test]
fn test_rspack_cjs_annotated_output_integration() {
    println!("\n=== RSPACK CJS ANNOTATED OUTPUT INTEGRATION TEST ===");
    
    let test_dir = Path::new("../../test-cases/rspack-cjs-annotated-output");
    let share_usage_path = test_dir.join("share-usage.json");
    let lodash_chunk_path = test_dir.join("vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    
    assert!(share_usage_path.exists(), "share-usage.json not found in rspack-cjs-annotated-output");
    assert!(lodash_chunk_path.exists(), "lodash chunk not found in rspack-cjs-annotated-output");
    
    // Load configuration
    let share_usage_content = fs::read_to_string(&share_usage_path)
        .expect("Failed to read share-usage.json");
    let share_usage: serde_json::Value = serde_json::from_str(&share_usage_content)
        .expect("Failed to parse share-usage.json");
    
    // Extract lodash configuration
    let lodash_config = &share_usage["treeShake"]["lodash-es"];
    assert!(lodash_config.is_object(), "lodash-es configuration should be an object");
    
    // Create test configuration
    let config = json!({
        "treeShake": {
            "lodash-es": lodash_config
        }
    });
    
    // Load and optimize lodash chunk
    let original_chunk = fs::read_to_string(&lodash_chunk_path)
        .expect("Failed to read lodash chunk");
    
    println!("Original chunk size: {} bytes", original_chunk.len());
    
    let optimized_chunk = optimize(original_chunk.clone(), &config.to_string());
    
    let reduction = ((original_chunk.len() - optimized_chunk.len()) as f64 / original_chunk.len() as f64) * 100.0;
    
    println!("Optimized chunk size: {} bytes", optimized_chunk.len());
    println!("Size reduction: {:.2}%", reduction);
    
    // Validate optimization results
    assert!(optimized_chunk.len() > 0, "Optimized chunk should not be empty");
    assert!(optimized_chunk.len() < original_chunk.len(), "Optimization should reduce size");
    assert!(reduction > 10.0, "Should achieve at least 10% reduction");
    
    // Check that module structure is preserved
    assert!(optimized_chunk.contains("exports.modules"), "Module structure should be preserved");
    
    println!("✅ RSPACK CJS annotated output integration test passed");
}

#[test]
fn test_webpack_esm_integration() {
    println!("\n=== WEBPACK ESM INTEGRATION TEST ===");
    
    let test_dir = Path::new("../../test-cases/webpack-esm");
    let share_usage_path = test_dir.join("share-usage.json");
    let react_chunk_path = test_dir.join("vendors-node_modules_pnpm_react_18_3_1_node_modules_react_index_js.mjs");
    
    assert!(share_usage_path.exists(), "share-usage.json not found in webpack-esm");
    assert!(react_chunk_path.exists(), "react chunk not found in webpack-esm");
    
    // Load configuration
    let share_usage_content = fs::read_to_string(&share_usage_path)
        .expect("Failed to read share-usage.json");
    let share_usage: serde_json::Value = serde_json::from_str(&share_usage_content)
        .expect("Failed to parse share-usage.json");
    
    // Extract react configuration
    let react_config = &share_usage["treeShake"]["react"];
    assert!(react_config.is_object(), "react configuration should be an object");
    
    // Create test configuration
    let config = json!({
        "treeShake": {
            "react": react_config
        }
    });
    
    // Load and optimize react chunk
    let original_chunk = fs::read_to_string(&react_chunk_path)
        .expect("Failed to read react chunk");
    
    println!("Original chunk size: {} bytes", original_chunk.len());
    
    let optimized_chunk = optimize(original_chunk.clone(), &config.to_string());
    
    let reduction = ((original_chunk.len() - optimized_chunk.len()) as f64 / original_chunk.len() as f64) * 100.0;
    
    println!("Optimized chunk size: {} bytes", optimized_chunk.len());
    println!("Size reduction: {:.2}%", reduction);
    
    // Validate optimization results
    assert!(optimized_chunk.len() > 0, "Optimized chunk should not be empty");
    assert!(reduction >= 0.0, "Should not increase size");
    
    println!("✅ Webpack ESM integration test passed");
}

#[test]
fn test_configuration_parsing_validation() {
    println!("\n=== CONFIGURATION PARSING VALIDATION TEST ===");
    
    let test_cases = [
        "../../test-cases/rspack-annotated-output/share-usage.json",
        "../../test-cases/rspack-cjs-annotated-output/share-usage.json",
        "../../test-cases/webpack-esm/share-usage.json",
    ];
    
    for config_path in &test_cases {
        let path = Path::new(config_path);
        assert!(path.exists(), "Configuration file should exist: {}", config_path);
        
        let content = fs::read_to_string(path)
            .expect(&format!("Failed to read {}", config_path));
        
        let config: serde_json::Value = serde_json::from_str(&content)
            .expect(&format!("Failed to parse JSON in {}", config_path));
        
        // Validate structure
        assert!(config.get("treeShake").is_some(), "Should have treeShake section in {}", config_path);
        
        let tree_shake = config["treeShake"].as_object()
            .expect(&format!("treeShake should be object in {}", config_path));
        
        // Check that each library has chunk_characteristics
        for (lib_name, lib_config) in tree_shake {
            let lib_obj = lib_config.as_object()
                .expect(&format!("Library {} should be object in {}", lib_name, config_path));
            
            assert!(lib_obj.contains_key("chunk_characteristics"), 
                "Library {} should have chunk_characteristics in {}", lib_name, config_path);
            
            let characteristics = &lib_obj["chunk_characteristics"];
            assert!(characteristics.get("entry_module_id").is_some(), 
                "Library {} should have entry_module_id in {}", lib_name, config_path);
            assert!(characteristics.get("is_runtime_chunk").is_some(), 
                "Library {} should have is_runtime_chunk in {}", lib_name, config_path);
            assert!(characteristics.get("chunk_files").is_some(), 
                "Library {} should have chunk_files in {}", lib_name, config_path);
        }
        
        println!("✅ Configuration {} is valid", config_path);
    }
    
    println!("✅ All configuration parsing validation tests passed");
}

#[test]
fn test_optimization_effectiveness_metrics() {
    println!("\n=== OPTIMIZATION EFFECTIVENESS METRICS TEST ===");
    
    let test_cases = [
        ("../../test-cases/rspack-annotated-output", "lodash-es", "vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js"),
        ("../../test-cases/rspack-cjs-annotated-output", "lodash-es", "vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js"),
    ];
    
    for (test_dir, lib_name, chunk_file) in &test_cases {
        println!("\nTesting {}/{}", test_dir, chunk_file);
        
        let test_path = Path::new(test_dir);
        let share_usage_path = test_path.join("share-usage.json");
        let chunk_path = test_path.join(chunk_file);
        
        if !share_usage_path.exists() || !chunk_path.exists() {
            println!("⚠️  Skipping {} - files not found", test_dir);
            continue;
        }
        
        // Load configuration
        let share_usage_content = fs::read_to_string(&share_usage_path).unwrap();
        let share_usage: serde_json::Value = serde_json::from_str(&share_usage_content).unwrap();
        
        let lib_config = &share_usage["treeShake"][lib_name];
        let config = json!({
            "treeShake": {
                (*lib_name): lib_config
            }
        });
        
        // Load and optimize chunk
        let original_chunk = fs::read_to_string(&chunk_path).unwrap();
        let optimized_chunk = optimize(original_chunk.clone(), &config.to_string());
        
        // Calculate metrics
        let original_size = original_chunk.len();
        let optimized_size = optimized_chunk.len();
        let size_reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;
        
        // Count object keys (rough estimate)
        let original_keys = original_chunk.matches(".js\":").count();
        let optimized_keys = optimized_chunk.matches(".js\":").count();
        let key_reduction = if original_keys > 0 {
            ((original_keys - optimized_keys) as f64 / original_keys as f64) * 100.0
        } else {
            0.0
        };
        
        println!("  Original size: {} bytes", original_size);
        println!("  Optimized size: {} bytes", optimized_size);
        println!("  Size reduction: {:.2}%", size_reduction);
        println!("  Original keys: {}", original_keys);
        println!("  Optimized keys: {}", optimized_keys);
        println!("  Key reduction: {:.2}%", key_reduction);
        
        // Validate effectiveness
        assert!(optimized_size <= original_size, "Optimization should not increase size");
        
        if size_reduction > 0.0 {
            println!("  ✅ Achieved {:.2}% size reduction", size_reduction);
        } else {
            println!("  ⚠️  No size reduction achieved");
        }
        
        if key_reduction > 0.0 {
            println!("  ✅ Achieved {:.2}% key reduction", key_reduction);
        } else {
            println!("  ⚠️  No key reduction achieved");
        }
    }
    
    println!("\n✅ Optimization effectiveness metrics test completed");
}