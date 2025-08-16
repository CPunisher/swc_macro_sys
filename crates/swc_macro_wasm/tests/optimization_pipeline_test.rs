use std::fs;
use std::path::Path;
use swc_macro_wasm::optimize;

#[test]
fn test_optimization_pipeline_on_real_chunk() {
    println!("\n=== TESTING OPTIMIZATION PIPELINE ON REAL MODULE FEDERATION CHUNK ===");
    
    // Load the same real chunk fixture
    let mf_chunk_path = Path::new("tests/fixtures/module_federation_lodash_chunk.js");
    let host_usage_path = Path::new("tests/fixtures/module_federation_usage.json");
    let remote_usage_path = Path::new("tests/fixtures/module_federation_remote_usage.json");
    
    if !mf_chunk_path.exists() {
        println!("⚠️  Real MF chunk fixture not found - skipping test");
        return;
    }
    
    let original_code = fs::read_to_string(mf_chunk_path).expect("Failed to read MF chunk fixture");
    let original_size = original_code.len();
    
    println!("Starting with real MF chunk: {:.2} KB ({} bytes)", 
        original_size as f64 / 1024.0, original_size);
    
    // Load usage data exactly like the scripts do
    let host_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(host_usage_path).expect("Failed to read host usage")
    ).expect("Failed to parse host usage JSON");
    
    let remote_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(remote_usage_path).expect("Failed to read remote usage")
    ).expect("Failed to parse remote usage JSON");
    
    // Extract usage patterns - handle both old and new formats
    let (host_used, remote_used, unused_exports) = if host_usage.get("treeShake").is_some() {
        // New format
        let host_lodash = &host_usage["treeShake"]["lodash-es"];
        let remote_lodash = &remote_usage["treeShake"]["lodash-es"];
        
        let host_lodash_obj = host_lodash.as_object().expect("host_lodash should be object");
        let remote_lodash_obj = remote_lodash.as_object().expect("remote_lodash should be object");
        
        // Extract used exports (where value is true)
        let host_used: Vec<String> = host_lodash_obj.iter()
            .filter(|(k, v)| k.as_str() != "chunk_characteristics" && v.as_bool() == Some(true))
            .map(|(k, _)| k.clone())
            .collect();
        let remote_used: Vec<String> = remote_lodash_obj.iter()
            .filter(|(k, v)| k.as_str() != "chunk_characteristics" && v.as_bool() == Some(true))
            .map(|(k, _)| k.clone())
            .collect();
        let unused_exports: Vec<String> = host_lodash_obj.iter()
            .filter(|(k, v)| k.as_str() != "chunk_characteristics" && v.as_bool() == Some(false))
            .map(|(k, _)| k.clone())
            .collect();
        
        (host_used, remote_used, unused_exports)
    } else {
        // Old format
        let host_used_arr = host_usage["consume_shared_modules"]["lodash-es"]["used_exports"].as_array().unwrap();
        let remote_used_arr = remote_usage["consume_shared_modules"]["lodash-es"]["used_exports"].as_array().unwrap();
        let unused_exports_arr = host_usage["consume_shared_modules"]["lodash-es"]["unused_exports"].as_array().unwrap();
        
        let host_used: Vec<String> = host_used_arr.iter().map(|v| v.as_str().unwrap().to_string()).collect();
        let remote_used: Vec<String> = remote_used_arr.iter().map(|v| v.as_str().unwrap().to_string()).collect();
        let unused_exports: Vec<String> = unused_exports_arr.iter().map(|v| v.as_str().unwrap().to_string()).collect();
        
        (host_used, remote_used, unused_exports)
    };
    
    println!("Host exports: {:?}", host_used);
    println!("Remote exports: {:?}", remote_used);
    
    // === TEST 1: Direct optimize() call (like our successful test) ===
    println!("\n--- TEST 1: Direct optimize() call ---");
    
    let mut all_used_exports = std::collections::HashSet::new();
    for export in &host_used {
        all_used_exports.insert(export.clone());
    }
    for export in &remote_used {
        all_used_exports.insert(export.clone());
    }
    
    let mut tree_shake_config = serde_json::Map::new();
    for export in &all_used_exports {
        tree_shake_config.insert(export.clone(), serde_json::Value::Bool(true));
    }
    for export in &unused_exports {
        if !all_used_exports.contains(export) {
            tree_shake_config.insert(export.clone(), serde_json::Value::Bool(false));
        }
    }
    
    let mut tree_shake_config_with_chars = tree_shake_config.clone();
    tree_shake_config_with_chars.insert("chunk_characteristics".to_string(), serde_json::json!({
        "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
        "is_runtime_chunk": false,
        "chunk_format": "require"
    }));
    let config1 = serde_json::json!({
        "treeShake": {
            "lodash-es": tree_shake_config_with_chars
        }
    });
    
    let config_str1 = serde_json::to_string(&config1).unwrap();
    let result1 = optimize(original_code.clone(), &config_str1);
    let reduction1 = ((original_size - result1.len()) as f64 / original_size as f64) * 100.0;
    
    println!("Direct optimize() result: {:.2}% reduction → {:.2}KB", 
        reduction1, result1.len() as f64 / 1024.0);
    
    // === TEST 2: Simulating script behavior with multiple iterations ===
    println!("\n--- TEST 2: Multi-pass optimization (like scripts) ---");
    
    let mut current_code = original_code.clone();
    let _total_reduction = 0.0;
    let max_passes = 10;
    
    for pass in 1..=max_passes {
        let before_size = current_code.len();
        let pass_result = optimize(current_code.clone(), &config_str1);
        let after_size = pass_result.len();
        
        let pass_reduction = ((before_size - after_size) as f64 / before_size as f64) * 100.0;
        let cumulative_reduction = ((original_size - after_size) as f64 / original_size as f64) * 100.0;
        
        println!("Pass {}: {:.1}% reduction → {:.2}KB (cumulative: {:.1}%)", 
            pass, pass_reduction, after_size as f64 / 1024.0, cumulative_reduction);
        
        if pass_reduction < 0.1 {
            println!("Converged after {} passes", pass);
            break;
        }
        
        current_code = pass_result;
    }
    
    // === TEST 3: Different config formats ===
    println!("\n--- TEST 3: Different configuration formats ---");
    
    // Test with minimal config (like some scripts might use)
    let minimal_config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }
        }
    });
    
    let minimal_result = optimize(original_code.clone(), &serde_json::to_string(&minimal_config).unwrap());
    let minimal_reduction = ((original_size - minimal_result.len()) as f64 / original_size as f64) * 100.0;
    
    println!("Minimal config result: {:.2}% reduction → {:.2}KB", 
        minimal_reduction, minimal_result.len() as f64 / 1024.0);
    
    // Test with no tree shaking (pure DCE)
    let no_treeshake_config = serde_json::json!({});
    let no_treeshake_result = optimize(original_code.clone(), &serde_json::to_string(&no_treeshake_config).unwrap());
    let no_treeshake_reduction = if original_size > no_treeshake_result.len() {
        ((original_size - no_treeshake_result.len()) as f64 / original_size as f64) * 100.0
    } else {
        0.0
    };
    
    println!("No tree-shaking (DCE only): {:.2}% reduction → {:.2}KB", 
        no_treeshake_reduction, no_treeshake_result.len() as f64 / 1024.0);
    
    // === TEST 4: Enhanced config with features ===
    println!("\n--- TEST 4: Enhanced config (like enhanced optimizer) ---");
    
    let mut enhanced_tree = tree_shake_config.clone();
    enhanced_tree.insert("chunk_characteristics".to_string(), serde_json::json!({
        "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
        "is_runtime_chunk": false,
        "chunk_format": "require"
    }));
    let enhanced_config = serde_json::json!({
        "treeShake": {
            "lodash-es": enhanced_tree
        },
        "optimization": {
            "aggressive": true,
            "convergence_mode": true,
            "max_iterations": 10
        },
        "features": {
            "enableDebugging": false,
            "enableLogging": false,
            "enableWebpackHMR": false
        },
        "build": {
            "target": "production",
            "mode": "aggressive"
        }
    });
    
    let enhanced_result = optimize(original_code.clone(), &serde_json::to_string(&enhanced_config).unwrap());
    let enhanced_reduction = ((original_size - enhanced_result.len()) as f64 / original_size as f64) * 100.0;
    
    println!("Enhanced config result: {:.2}% reduction → {:.2}KB", 
        enhanced_reduction, enhanced_result.len() as f64 / 1024.0);
    
    // === ANALYSIS ===
    println!("\n=== ANALYSIS ===");
    println!("Direct optimize():     {:.1}% reduction", reduction1);
    println!("Multi-pass:           {:.1}% reduction", ((original_size - current_code.len()) as f64 / original_size as f64) * 100.0);
    println!("Minimal config:       {:.1}% reduction", minimal_reduction);
    println!("No tree-shaking:      {:.1}% reduction", no_treeshake_reduction);
    println!("Enhanced config:      {:.1}% reduction", enhanced_reduction);
    
    // Check if all results are similar (indicating our tree shaker works consistently)
    let results = vec![reduction1, minimal_reduction, no_treeshake_reduction, enhanced_reduction];
    let max_reduction = results.iter().cloned().fold(0.0, f64::max);
    let min_reduction = results.iter().cloned().fold(100.0, f64::min);
    let variance = max_reduction - min_reduction;
    
    println!("\nVariance between approaches: {:.1}% points", variance);
    
    if variance < 5.0 {
        println!("✅ All optimization approaches yield similar results - tree shaker working consistently");
    } else if max_reduction > 90.0 {
        println!("✅ Maximum optimization achieved: {:.1}% - tree shaker working well", max_reduction);
    } else {
        println!("⚠️  Inconsistent results - may indicate integration issues");
    }
    
    println!("✅ Optimization pipeline test completed!");
}

#[test]
fn test_javascript_integration_simulation() {
    println!("\n=== SIMULATING JAVASCRIPT INTEGRATION EXACTLY ===");
    
    let mf_chunk_path = Path::new("tests/fixtures/module_federation_lodash_chunk.js");
    let host_usage_path = Path::new("tests/fixtures/module_federation_usage.json");
    let remote_usage_path = Path::new("tests/fixtures/module_federation_remote_usage.json");
    
    if !mf_chunk_path.exists() {
        println!("⚠️  Fixtures not available - skipping JS integration test");
        return;
    }
    
    let original_code = fs::read_to_string(mf_chunk_path).unwrap();
    let original_size = original_code.len();
    
    println!("Testing JavaScript integration simulation...");
    println!("Original chunk: {:.2}KB", original_size as f64 / 1024.0);
    
    // Simulate exactly what the JavaScript enhanced optimizer does
    let host_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(host_usage_path).unwrap()
    ).unwrap();
    
    let remote_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(remote_usage_path).unwrap()
    ).unwrap();
    
    // Extract usage supporting both legacy and new share-usage formats
    let (host_used_vec, remote_used_vec, unused_exports_vec): (Vec<String>, Vec<String>, Vec<String>) = if host_usage.get("treeShake").is_some() {
        let host_obj = host_usage["treeShake"]["lodash-es"].as_object().expect("host lodash-es should be object");
        let remote_obj = remote_usage["treeShake"]["lodash-es"].as_object().expect("remote lodash-es should be object");
        let mut host_used = Vec::new();
        let mut remote_used = Vec::new();
        let mut unused = Vec::new();
        for (k, v) in host_obj {
            if k == "chunk_characteristics" { continue; }
            match v.as_bool() {
                Some(true) => host_used.push(k.clone()),
                Some(false) => unused.push(k.clone()),
                _ => {}
            }
        }
        for (k, v) in remote_obj {
            if k == "chunk_characteristics" { continue; }
            if v.as_bool() == Some(true) { remote_used.push(k.clone()); }
        }
        (host_used, remote_used, unused)
    } else {
        let host_used = host_usage["consume_shared_modules"]["lodash-es"]["used_exports"].as_array().unwrap()
            .iter().map(|v| v.as_str().unwrap().to_string()).collect();
        let remote_used = remote_usage["consume_shared_modules"]["lodash-es"]["used_exports"].as_array().unwrap()
            .iter().map(|v| v.as_str().unwrap().to_string()).collect();
        let unused = host_usage["consume_shared_modules"]["lodash-es"]["unused_exports"].as_array().unwrap()
            .iter().map(|v| v.as_str().unwrap().to_string()).collect();
        (host_used, remote_used, unused)
    };
    
    // Create the same tree-shake config structure as JS
    let mut combined_used = std::collections::HashSet::new();
    for export in &host_used_vec {
        combined_used.insert(export.clone());
    }
    for export in &remote_used_vec {
        combined_used.insert(export.clone());
    }
    
    let mut tree_shake_config = serde_json::Map::new();
    for export in &combined_used {
        tree_shake_config.insert(export.clone(), serde_json::Value::Bool(true));
    }
    for export_name in unused_exports_vec {
        if !combined_used.contains(&export_name) {
            tree_shake_config.insert(export_name, serde_json::Value::Bool(false));
        }
    }
    
    // Simulate the enhanced optimizer's config structure
    let js_config = serde_json::json!({
        "treeShake": {
            "lodash-es": tree_shake_config
        },
        "optimization": {
            "pass": 1,
            "aggressive": true,
            "convergence_mode": true,
            "max_iterations": 10,
            "remove_unused_imports": true,
            "eliminate_dead_code": true
        },
        "features": {
            "enableDebugging": false,
            "enableLogging": false,
            "enableDevMode": false,
            "enableTestMode": false,
            "enableAnalytics": false,
            "enableMetrics": false,
            "enableTracing": false,
            "enableProfiling": false,
            "enableWebpackHMR": false,
            "enableSourceMaps": false,
            "enableComments": false,
            "enableLazyLoading": false,
            "enableCodeSplitting": false,
            "enableDynamicImports": false,
            "enablePolyfills": false,
            "enableBabelTransforms": false,
            "enableMinification": false
        },
        
        "build": {
            "target": "production",
            "mode": "aggressive",
            "minify": false,
            "removeDeadCode": true,
            "treeShake": true,
            "removeUnusedImports": true,
            "eliminateDeadBranches": true
        }
    });
    
    // Run multiple passes like the JS enhanced optimizer does
    let mut current_code = original_code.clone();
    let max_passes = 10;
    let convergence_threshold = 0.1;
    
    for pass in 1..=max_passes {
        let before_size = current_code.len();
        
        // Update pass number in config
        let mut pass_config = js_config.clone();
        pass_config["optimization"]["pass"] = serde_json::Value::Number(serde_json::Number::from(pass));
        
        let config_str = serde_json::to_string(&pass_config).unwrap();
        let optimized_code = optimize(current_code.clone(), &config_str);
        let after_size = optimized_code.len();
        
        let pass_reduction = ((before_size - after_size) as f64 / before_size as f64) * 100.0;
        let cumulative_reduction = ((original_size - after_size) as f64 / original_size as f64) * 100.0;
        
        println!("Pass {}: {:.1}% reduction → {:.2}KB (cumulative: {:.1}%)", 
            pass, pass_reduction, after_size as f64 / 1024.0, cumulative_reduction);
        
        if pass_reduction < convergence_threshold {
            println!("✅ Converged after {} passes (reduction < {}%)", pass, convergence_threshold);
            break;
        }
        
        if after_size == before_size {
            println!("✅ No further optimization possible after {} passes", pass);
            break;
        }
        
        current_code = optimized_code;
    }
    
    let final_reduction = ((original_size - current_code.len()) as f64 / original_size as f64) * 100.0;
    
    println!("\n📊 FINAL RESULTS:");
    println!("Original size: {:.2}KB", original_size as f64 / 1024.0);
    println!("Final size: {:.2}KB", current_code.len() as f64 / 1024.0);
    println!("Total reduction: {:.1}%", final_reduction);
    
    // Compare with expected results
    if final_reduction > 90.0 {
        println!("✅ Excellent optimization achieved - matches test expectations");
    } else if final_reduction > 70.0 {
        println!("📊 Good optimization achieved");
    } else if final_reduction > 30.0 {
        println!("⚠️  Moderate optimization - may not match test expectations");
    } else {
        println!("❌ Poor optimization - significant discrepancy with test expectations");
    }
    
    println!("✅ JavaScript integration simulation completed!");
}

#[test]
fn test_compare_with_scripts_behavior() {
    println!("\n=== COMPARING WITH ACTUAL SCRIPT BEHAVIOR ===");
    
    let mf_chunk_path = Path::new("tests/fixtures/module_federation_lodash_chunk.js");
    if !mf_chunk_path.exists() {
        println!("⚠️  Fixture not available - skipping comparison");
        return;
    }
    
    let original_code = fs::read_to_string(mf_chunk_path).unwrap();
    let original_size = original_code.len();
    
    println!("Simulating actual script behavior...");
    println!("Original: {:.2}KB", original_size as f64 / 1024.0);
    
    // This simulates what the scripts are actually doing and getting 39.8%
    let script_config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "uniq": true,
                "sortBy": true,
                "default": true,
                "omit": true,
                "capitalize": true,
                "pick": true,
                "groupBy": true,
                "throttle": true,
                "debounce": true
            }
        },
        
    });
    
    let script_result = optimize(original_code.clone(), &serde_json::to_string(&script_config).unwrap());
    let script_reduction = ((original_size - script_result.len()) as f64 / original_size as f64) * 100.0;
    
    println!("Script-like config result: {:.1}% reduction → {:.2}KB", 
        script_reduction, script_result.len() as f64 / 1024.0);
    
    // Expected based on our successful test
    let expected_reduction = 99.9;
    let actual_reduction = script_reduction;
    
    println!("\nComparison:");
    println!("Expected (from test): ~{:.1}%", expected_reduction);
    println!("Actual (from script): {:.1}%", actual_reduction);
    println!("Difference: {:.1}% points", (expected_reduction - actual_reduction).abs());
    
    if (expected_reduction - actual_reduction).abs() < 5.0 {
        println!("✅ Results match expectations - optimization working correctly");
    } else {
        println!("❌ Significant discrepancy - need to investigate further");
        
        // Additional debugging
        println!("\nDebugging info:");
        println!("Original contains 'exports.modules': {}", original_code.contains("exports.modules"));
        println!("Result contains 'exports.modules': {}", script_result.contains("exports.modules"));
        println!("Original lodash modules: {}", original_code.matches("lodash-es").count());
        println!("Result lodash modules: {}", script_result.matches("lodash-es").count());
        
        // Check if tree shaker is working by looking at module reduction
        let original_modules = count_webpack_modules(&original_code);
        let result_modules = count_webpack_modules(&script_result);
        println!("Modules: {} → {} ({} removed)", original_modules, result_modules, original_modules - result_modules);
        
        if original_modules - result_modules > 100 {
            println!("✅ Tree shaker is removing modules");
        } else {
            println!("⚠️  Tree shaker may not be working properly");
        }
    }
    
    println!("✅ Script behavior comparison completed!");
}

fn count_webpack_modules(code: &str) -> usize {
    // Count webpack module patterns
    let function_patterns = code.matches("function(__unused_webpack").count();
    let path_patterns = code.matches("node_modules/.pnpm/lodash-es").count();
    std::cmp::max(function_patterns, path_patterns)
}