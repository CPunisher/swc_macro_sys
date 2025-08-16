use std::fs;
use std::path::Path;
use swc_macro_wasm::optimize;

#[test]
fn test_real_module_federation_chunk_optimization() {
    println!("\n=== TESTING REAL MODULE FEDERATION CHUNK OPTIMIZATION ===");
    
    // Load real Module Federation chunk and usage data
    let mf_chunk_path = Path::new("tests/fixtures/module_federation_lodash_chunk.js");
    let host_usage_path = Path::new("tests/fixtures/module_federation_usage.json");
    let remote_usage_path = Path::new("tests/fixtures/module_federation_remote_usage.json");
    
    assert!(mf_chunk_path.exists(), "Real MF chunk fixture not found at {}. Build examples/module-federation-example and copy fixtures first", mf_chunk_path.display());
    
    let original_code = fs::read_to_string(mf_chunk_path).expect("Failed to read MF chunk fixture");
    let original_size = original_code.len();
    
    assert!(original_size > 0);
    
    // Load and merge usage data
    let host_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(host_usage_path).expect("Failed to read host usage")
    ).expect("Failed to parse host usage JSON");
    
    let remote_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(remote_usage_path).expect("Failed to read remote usage")
    ).expect("Failed to parse remote usage JSON");
    
    // Extract lodash usage patterns - handle both old and new formats
    let (host_used, remote_used, tree_shake_config) = if host_usage.get("treeShake").is_some() {
        // New format
        let host_lodash = &host_usage["treeShake"]["lodash-es"];
        let remote_lodash = &remote_usage["treeShake"]["lodash-es"];
        
        let host_lodash_obj = host_lodash.as_object().expect("host_lodash should be object");
        let remote_lodash_obj = remote_lodash.as_object().expect("remote_lodash should be object");
        
        // Extract used exports (where value is true)
        let host_used: Vec<&str> = host_lodash_obj.iter()
            .filter(|(k, v)| k.as_str() != "chunk_characteristics" && v.as_bool() == Some(true))
            .map(|(k, _)| k.as_str())
            .collect();
        let remote_used: Vec<&str> = remote_lodash_obj.iter()
            .filter(|(k, v)| k.as_str() != "chunk_characteristics" && v.as_bool() == Some(true))
            .map(|(k, _)| k.as_str())
            .collect();
        
        // Create merged config
        let mut tree_shake_config = serde_json::Map::new();
        for (key, value) in host_lodash_obj {
            if key != "chunk_characteristics" {
                tree_shake_config.insert(key.clone(), value.clone());
            }
        }
        for (key, value) in remote_lodash_obj {
            if key != "chunk_characteristics" && value.as_bool() == Some(true) {
                tree_shake_config.insert(key.clone(), serde_json::Value::Bool(true));
            }
        }
        
        (host_used, remote_used, tree_shake_config)
    } else {
        // Old format
        let host_used = host_usage["consume_shared_modules"]["lodash-es"]["used_exports"].as_array().unwrap();
        let remote_used = remote_usage["consume_shared_modules"]["lodash-es"]["used_exports"].as_array().unwrap();
        let unused_exports = host_usage["consume_shared_modules"]["lodash-es"]["unused_exports"].as_array().unwrap();
        
        let host_used_strs: Vec<&str> = host_used.iter().map(|v| v.as_str().unwrap()).collect();
        let remote_used_strs: Vec<&str> = remote_used.iter().map(|v| v.as_str().unwrap()).collect();
        
        // Merge used exports (union)
        let mut all_used_exports = std::collections::HashSet::new();
        for export in &host_used_strs {
            all_used_exports.insert(export.to_string());
        }
        for export in &remote_used_strs {
            all_used_exports.insert(export.to_string());
        }
        
        // Create tree shake config
        let mut tree_shake_config = serde_json::Map::new();
        
        // Mark used exports as true, unused as false
        for export in &all_used_exports {
            tree_shake_config.insert(export.clone(), serde_json::Value::Bool(true));
        }
        for export in unused_exports {
            let export_name = export.as_str().unwrap();
            if !all_used_exports.contains(export_name) {
                tree_shake_config.insert(export_name.to_string(), serde_json::Value::Bool(false));
            }
        }
        
        (host_used_strs, remote_used_strs, tree_shake_config)
    };
    
    println!("Host app uses {} lodash exports: {:?}", host_used.len(), host_used);
    println!("Remote app uses {} lodash exports: {:?}", remote_used.len(), remote_used);
    
    let all_used_count = tree_shake_config.iter()
        .filter(|(_, v)| v.as_bool() == Some(true))
        .count();
    println!("Combined used exports: {} total", all_used_count);
    
    let mut tree_shake_config = tree_shake_config;
    tree_shake_config.insert("chunk_characteristics".to_string(), serde_json::json!({
        "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
        "is_runtime_chunk": false,
        "chunk_format": "require"
    }));
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": tree_shake_config
        }
    });
    
    println!("Tree shake config includes {} exports ({} used, {} unused)",
        tree_shake_config.len(),
        all_used_count,
        tree_shake_config.len() - all_used_count
    );
    
    // Run optimization
    println!("\n🔧 Running optimization on real MF chunk...");
    let config_str = serde_json::to_string(&config).expect("Failed to serialize config");
    let optimized_code = optimize(original_code.clone(), &config_str);
    let optimized_size = optimized_code.len();
    let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;
    
    assert!(optimized_size > 0);
    
    // Analyze the chunk format
    analyze_real_chunk_structure(&original_code, "Original MF Chunk");
    analyze_real_chunk_structure(&optimized_code, "Optimized MF Chunk");
    
    // Check if our tree shaker is working
    if reduction > 50.0 {
        println!("✅ Excellent optimization achieved!");
    } else if reduction > 30.0 {
        println!("📊 Moderate optimization achieved");
    } else {
        println!("⚠️  Limited optimization - tree shaker may not be working properly");
    }
    
    // Verify the optimized code maintains structure
    assert!(optimized_code.contains("exports.modules") || optimized_code.contains("exports.ids"),
        "Optimized MF chunk should maintain CommonJS exports structure");
    
    println!("✅ Real Module Federation chunk test completed!");
}

#[test]
fn test_real_standard_webpack_chunk_optimization() {
    println!("\n=== TESTING REAL STANDARD WEBPACK CHUNK OPTIMIZATION ===");
    
    let std_chunk_path = Path::new("tests/fixtures/standard_webpack_lodash_chunk.js");
    let std_usage_path = Path::new("tests/fixtures/standard_webpack_usage.json");
    
    if !std_chunk_path.exists() {
        println!("⚠️  Real standard chunk fixture not found");
        return;
    }
    
    let original_code = fs::read_to_string(std_chunk_path).expect("Failed to read standard chunk fixture");
    let original_size = original_code.len();
    
    let usage_data: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(std_usage_path).expect("Failed to read standard usage")
    ).expect("Failed to parse standard usage JSON");
    
    // Support both legacy and new share-usage formats
    let (used_exports_vec, unused_exports_vec): (Vec<String>, Vec<String>) = if usage_data.get("treeShake").is_some() {
        let obj = usage_data["treeShake"]["lodash-es"].as_object().expect("lodash-es should be object");
        let mut used = Vec::new();
        let mut unused = Vec::new();
        for (k, v) in obj {
            if k == "chunk_characteristics" { continue; }
            if v.as_bool() == Some(true) { used.push(k.clone()); } else { unused.push(k.clone()); }
        }
        (used, unused)
    } else {
        let used = usage_data["consume_shared_modules"]["lodash-es"]["used_exports"]
            .as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
        let unused = usage_data["consume_shared_modules"]["lodash-es"]["unused_exports"]
            .as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
        (used, unused)
    };
    
    println!("Standard webpack chunk size: {} bytes ({:.2} KB)", 
        original_size, original_size as f64 / 1024.0);
    println!("Used exports: {:?}", used_exports_vec);
    
    // Create tree shake config
    let mut tree_shake_config = serde_json::Map::new();
    for export in used_exports_vec { tree_shake_config.insert(export, serde_json::Value::Bool(true)); }
    for export in unused_exports_vec { tree_shake_config.insert(export, serde_json::Value::Bool(false)); }
    
    let mut tree_shake_config = tree_shake_config;
    tree_shake_config.insert("chunk_characteristics".to_string(), serde_json::json!({
        "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
        "is_runtime_chunk": false,
        "chunk_format": "require"
    }));
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": tree_shake_config
        }
    });
    
    // Run optimization
    let config_str = serde_json::to_string(&config).unwrap();
    let optimized_code = optimize(original_code.clone(), &config_str);
    let optimized_size = optimized_code.len();
    let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;
    
    println!("Standard webpack optimization results:");
    println!("  Original size: {} bytes ({:.2} KB)", original_size, original_size as f64 / 1024.0);
    println!("  Optimized size: {} bytes ({:.2} KB)", optimized_size, optimized_size as f64 / 1024.0);
    println!("  Size reduction: {:.2}% ({} bytes saved)", reduction, original_size - optimized_size);
    
    analyze_real_chunk_structure(&original_code, "Original Standard Chunk");
    analyze_real_chunk_structure(&optimized_code, "Optimized Standard Chunk");
    
    println!("✅ Real standard webpack chunk test completed!");
}

#[test]
fn test_chunk_format_comparison_with_real_data() {
    println!("\n=== COMPARING REAL CHUNK FORMAT OPTIMIZATION ===");
    
    let mf_path = Path::new("tests/fixtures/module_federation_lodash_chunk.js");
    let std_path = Path::new("tests/fixtures/standard_webpack_lodash_chunk.js");
    
    if !mf_path.exists() || !std_path.exists() {
        println!("⚠️  Real chunk fixtures not available, skipping comparison");
        return;
    }
    
    let mf_code = fs::read_to_string(mf_path).unwrap();
    let std_code = fs::read_to_string(std_path).unwrap();
    
    println!("MF chunk size: {} bytes ({:.1} KB)", mf_code.len(), mf_code.len() as f64 / 1024.0);
    println!("Standard chunk size: {} bytes ({:.1} KB)", std_code.len(), std_code.len() as f64 / 1024.0);
    
    // Use same minimal config for both (only keep 'default' export)
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
    let config_str = serde_json::to_string(&minimal_config).unwrap();
    
    // Test both with same config
    let mf_optimized = optimize(mf_code.clone(), &config_str);
    let std_optimized = optimize(std_code.clone(), &config_str);
    
    let mf_reduction = ((mf_code.len() - mf_optimized.len()) as f64 / mf_code.len() as f64) * 100.0;
    let std_reduction = ((std_code.len() - std_optimized.len()) as f64 / std_code.len() as f64) * 100.0;
    
    println!("\nWith minimal configuration (default export only):");
    println!("  MF optimization: {:.1}% → {:.1}KB", mf_reduction, mf_optimized.len() as f64 / 1024.0);
    println!("  Standard optimization: {:.1}% → {:.1}KB", std_reduction, std_optimized.len() as f64 / 1024.0);
    
    let reduction_diff = (std_reduction - mf_reduction).abs();
    println!("  Difference: {:.1}% points", reduction_diff);
    
    if reduction_diff < 5.0 {
        println!("✅ Both formats optimize similarly - tree shaker working correctly");
    } else if std_reduction > mf_reduction + 10.0 {
        println!("⚠️  Standard format optimizes much better - MF tree shaker needs improvement");
    } else {
        println!("📊 Moderate difference detected");
    }
    
    // Debug: Check if tree shaking is working by looking for removed modules
    let mf_modules_before = count_modules_in_chunk(&mf_code);
    let mf_modules_after = count_modules_in_chunk(&mf_optimized);
    let std_modules_before = count_modules_in_chunk(&std_code);
    let std_modules_after = count_modules_in_chunk(&std_optimized);
    
    println!("\nModule removal analysis:");
    println!("  MF modules: {} → {} ({} removed)", mf_modules_before, mf_modules_after, mf_modules_before - mf_modules_after);
    println!("  Standard modules: {} → {} ({} removed)", std_modules_before, std_modules_after, std_modules_before - std_modules_after);
    
    if mf_modules_before - mf_modules_after > 100 {
        println!("✅ MF tree shaker successfully removing modules");
    } else {
        println!("⚠️  MF tree shaker not removing many modules - investigate further");
    }
    
    println!("✅ Real chunk format comparison completed!");
}

fn analyze_real_chunk_structure(code: &str, name: &str) {
    println!("\n📋 Analyzing {}:", name);
    
    // Module count estimation
    let module_count = count_modules_in_chunk(code);
    println!("  Estimated modules: {}", module_count);
    
    // Check formats
    let has_exports_modules = code.contains("exports.modules");
    let has_webpack_chunk = code.contains("webpackChunk");
    let has_webpack_require = code.contains("__webpack_require__");
    
    println!("  Format indicators:");
    println!("    exports.modules: {}", has_exports_modules);
    println!("    webpackChunk: {}", has_webpack_chunk);
    println!("    __webpack_require__: {}", has_webpack_require);
    
    // Check for specific lodash modules
    let lodash_modules = [
        "map.js", "filter.js", "VERSION.js", "default.js",
        "uniq.js", "sortBy.js", "omit.js", "capitalize.js",
        "pick.js", "groupBy.js", "throttle.js", "debounce.js"
    ];
    
    let mut present_modules = Vec::new();
    for module in &lodash_modules {
        if code.contains(module) {
            present_modules.push(*module);
        }
    }
    
    println!("  Key lodash modules present: {}/{}", present_modules.len(), lodash_modules.len());
    if !present_modules.is_empty() {
        println!("    {:?}", present_modules);
    }
    
    // Size breakdown
    let lines = code.lines().count();
    println!("  Lines of code: {}", lines);
    println!("  Average bytes per line: {:.1}", code.len() as f64 / lines as f64);
}

fn count_modules_in_chunk(code: &str) -> usize {
    // Count different module patterns
    let function_modules = code.matches("function(__unused_webpack").count();
    let arrow_matches = code.matches("(__unused_webpack").count();
    let arrow_modules = if arrow_matches > function_modules {
        arrow_matches - function_modules
    } else {
        0
    };
    let total_webpack_modules = function_modules + arrow_modules;
    
    // Also try counting by module path patterns
    let path_modules = code.matches("node_modules/.pnpm/lodash-es").count();
    
    // Return the most reasonable count
    std::cmp::max(total_webpack_modules, path_modules)
}

#[test]
fn test_debug_tree_shaker_on_real_chunk() {
    println!("\n=== DEBUGGING TREE SHAKER ON REAL CHUNK ===");
    
    let mf_path = Path::new("tests/fixtures/module_federation_lodash_chunk.js");
    if !mf_path.exists() {
        println!("⚠️  MF chunk fixture not available");
        return;
    }
    
    let original_code = fs::read_to_string(mf_path).unwrap();
    
    // Test with progressively aggressive configurations
    let configs = vec![
        ("No tree shaking", serde_json::json!({})),
        ("Minimal (default only)", serde_json::json!({
            "treeShake": { "lodash-es": { "default": true, "chunk_characteristics": {
                "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                "is_runtime_chunk": false,
                "chunk_format": "require"
            } } }
        })),
        ("Conservative (4 exports)", serde_json::json!({
            "treeShake": { "lodash-es": {
                "map": true, "filter": true, "VERSION": true, "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }}
        })),
        ("Aggressive (1 export)", serde_json::json!({
            "treeShake": { "lodash-es": { "map": true, "chunk_characteristics": {
                "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                "is_runtime_chunk": false,
                "chunk_format": "require"
            } } }
        }))
    ];
    
    println!("Original chunk size: {:.1}KB", original_code.len() as f64 / 1024.0);
    
    for (name, config) in configs {
        let config_str = serde_json::to_string(&config).unwrap();
        let optimized = optimize(original_code.clone(), &config_str);
        let before = original_code.len();
        let after = optimized.len();
        let reduction = if before > after {
            ((before - after) as f64 / before as f64) * 100.0
        } else { 0.0 };
        
        println!("{}: {:.1}% reduction → {:.1}KB", 
            name, reduction, optimized.len() as f64 / 1024.0);
    }
    
    println!("✅ Tree shaker debugging completed!");
}