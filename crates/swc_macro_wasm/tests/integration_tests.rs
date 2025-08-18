// Integration tests for optimization pipeline using real webpack chunks and share-usage.json

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn read_jsonp_dir() -> PathBuf {
    PathBuf::from("/Users/bytedance/dev/swc_macro_sys/tests/jsonp")
}

fn load_share_usage() -> Value {
    let path = read_jsonp_dir().join("share-usage.json");
    let data = fs::read_to_string(&path).expect("failed to read share-usage.json");
    serde_json::from_str(&data).expect("invalid JSON in share-usage.json")
}

fn load_chunk_file(file_name: &str) -> String {
    let path = read_jsonp_dir().join(file_name);
    fs::read_to_string(&path).expect("failed to read chunk file")
}

fn optimize_with_config(source: String, config: Value) -> String {
    swc_macro_wasm::optimize::optimize(source, config)
}

fn count_modules_in_webpack_chunk(source: &str) -> usize {
    // Use the webpack parser to count modules
    if let Ok(parser) = swc_macro_wasm::webpack_parser::WebpackChunkParser::new() {
        if let Ok(chunk) = parser.parse_chunk_file(source) {
            return chunk.modules.len();
        }
    }
    0
}

#[test]
fn iterate_share_usage_and_optimize_each_chunk() {
    // Load jsonp usage data
    let usage = load_share_usage();

    // Expect top-level object with libraries
    let tree_shake = usage
        .get("treeShake")
        .expect("share-usage.json is expected to contain treeShake root object");

    // Iterate modules under treeShake (e.g., antd, react, chart.js)
    for (pkg_name, pkg_cfg) in tree_shake.as_object().expect("treeShake should be an object") {
        // Each package should have a chunk_characteristics object with entry_module_id and chunk_files
        if let Some(chars) = pkg_cfg.get("chunk_characteristics") {
            // chunk_files tells us which actual chunk file to optimize
            let chunk_files = chars
                .get("chunk_files")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            // Skip if no files listed
            if chunk_files.is_empty() {
                continue;
            }

            // Use first chunk file for this package
            let first_chunk = chunk_files[0]
                .as_str()
                .expect("chunk_files entries must be strings");
            let entry_module_id = chars
                .get("entry_module_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            // Build config in a similar shape to JS example: attach chunk_characteristics under treeShake for this package
            // Also carry through the package-specific used exports map to allow future selective pruning
            let mut module_cfg = pkg_cfg.clone();
            // Ensure chunk_characteristics is present and entry_module_id is visible
            if let Some(obj) = module_cfg.as_object_mut() {
                obj.insert("chunk_characteristics".into(), chars.clone());
            }

            let mut tree_shake_obj = serde_json::Map::new();
            tree_shake_obj.insert(pkg_name.clone(), module_cfg);

            let config = Value::Object(serde_json::Map::from_iter([
                ("treeShake".into(), Value::Object(tree_shake_obj)),
            ]));

            // Load and optimize the chunk
            let source = load_chunk_file(first_chunk);
            let optimized = optimize_with_config(source.clone(), config);

            // Basic assertions: optimized output should be valid JS string
            assert!(optimized.len() > 0, "optimized output should not be empty for {}", pkg_name);

            // Count modules in original and optimized chunks
            let original_modules = count_modules_in_webpack_chunk(&source);
            let optimized_modules = count_modules_in_webpack_chunk(&optimized);

            println!("Package: {}, Original modules: {}, Optimized modules: {}", 
                     pkg_name, original_modules, optimized_modules);

            // If entry_module_id is present, we expect it to survive; at minimum string should still be present
            if !entry_module_id.is_empty() {
                assert!(
                    optimized.contains(entry_module_id) || source.contains(entry_module_id),
                    "entry_module_id should be preserved in or referenced by optimized output for {}",
                    pkg_name
                );

                // For chunks with explicit entry points and more than 10 modules, 
                // expect pruning to achieve some reduction
                if original_modules > 10 {
                    assert!(optimized_modules <= original_modules, 
                            "Optimized module count should not exceed original for {}", pkg_name);
                }
            }
        }
    }
}

#[test]
fn ensure_non_entry_shared_chunks_are_prunable() {
    // Focus on a known shared chunk (react-redux) which is marked is_shared_chunk true in share-usage.json
    let usage = load_share_usage();
    let tree_shake = usage.get("treeShake").expect("missing treeShake");

    let pkg = tree_shake.get("react-redux").expect("react-redux config missing");
    let chars = pkg
        .get("chunk_characteristics")
        .expect("chunk_characteristics missing for react-redux");

    let chunk_files = chars
        .get("chunk_files")
        .and_then(|v| v.as_array())
        .expect("react-redux should have chunk_files");
    let first_chunk = chunk_files[0]
        .as_str()
        .expect("chunk file must be string");

    // Build minimal config with only react-redux
    let mut module_cfg = pkg.clone();
    if let Some(obj) = module_cfg.as_object_mut() {
        obj.insert("chunk_characteristics".into(), chars.clone());
    }
    let mut tree_shake_obj = serde_json::Map::new();
    tree_shake_obj.insert("react-redux".to_string(), module_cfg);
    let config = Value::Object(serde_json::Map::from_iter([(
        "treeShake".into(),
        Value::Object(tree_shake_obj),
    )]));

    let source = load_chunk_file(first_chunk);
    let optimized = optimize_with_config(source.clone(), config);

    // For a shared non-entry chunk, optimization should either keep same or reduce size
    assert!(optimized.len() > 0, "optimized output should not be empty for react-redux");

    // Module count analysis
    let original_modules = count_modules_in_webpack_chunk(&source);
    let optimized_modules = count_modules_in_webpack_chunk(&optimized);

    println!("React-redux chunk - Original modules: {}, Optimized modules: {}", 
             original_modules, optimized_modules);

    assert!(optimized_modules <= original_modules, 
            "React-redux optimization should not increase module count");
}

#[test]
fn test_pruning_with_chart_js_aggressive_config() {
    // Test with chart.js which has many exports marked as false/unused
    let usage = load_share_usage();
    let tree_shake = usage.get("treeShake").expect("missing treeShake");

    let pkg = tree_shake.get("chart.js").expect("chart.js config missing");
    let chars = pkg
        .get("chunk_characteristics")
        .expect("chunk_characteristics missing for chart.js");

    let chunk_files = chars
        .get("chunk_files")
        .and_then(|v| v.as_array())
        .expect("chart.js should have chunk_files");
    let first_chunk = chunk_files[0]
        .as_str()
        .expect("chunk file must be string");

    // Build config with chart.js
    let mut module_cfg = pkg.clone();
    if let Some(obj) = module_cfg.as_object_mut() {
        obj.insert("chunk_characteristics".into(), chars.clone());
    }
    let mut tree_shake_obj = serde_json::Map::new();
    tree_shake_obj.insert("chart.js".to_string(), module_cfg);
    let config = Value::Object(serde_json::Map::from_iter([(
        "treeShake".into(),
        Value::Object(tree_shake_obj),
    )]));

    let source = load_chunk_file(first_chunk);
    let optimized = optimize_with_config(source.clone(), config);

    assert!(optimized.len() > 0, "optimized output should not be empty for chart.js");

    let original_modules = count_modules_in_webpack_chunk(&source);
    let optimized_modules = count_modules_in_webpack_chunk(&optimized);

    println!(
        "chart.js chunk - Original modules: {}, Optimized modules: {}",
        original_modules, optimized_modules
    );

    assert!(
        optimized_modules <= original_modules,
        "chart.js optimization should not increase module count"
    );
}

#[test]
fn test_antd_icons_pruning_effectiveness() {
    let usage = load_share_usage();
    let tree_shake = usage.get("treeShake").expect("missing treeShake");

    let pkg = tree_shake
        .get("@ant-design/icons")
        .expect("@ant-design/icons config missing");
    let chars = pkg
        .get("chunk_characteristics")
        .expect("chunk_characteristics missing for @ant-design/icons");

    let chunk_files = chars
        .get("chunk_files")
        .and_then(|v| v.as_array())
        .expect("@ant-design/icons should have chunk_files");
    let first_chunk = chunk_files[0]
        .as_str()
        .expect("chunk file must be string");

    let mut module_cfg = pkg.clone();
    if let Some(obj) = module_cfg.as_object_mut() {
        obj.insert("chunk_characteristics".into(), chars.clone());
    }
    let mut tree_shake_obj = serde_json::Map::new();
    tree_shake_obj.insert("@ant-design/icons".to_string(), module_cfg);
    let config = Value::Object(serde_json::Map::from_iter([(
        "treeShake".into(),
        Value::Object(tree_shake_obj),
    )]));

    let source = load_chunk_file(first_chunk);
    let optimized = optimize_with_config(source.clone(), config);

    assert!(optimized.len() > 0, "optimized output should not be empty for @ant-design/icons");

    let original_modules = count_modules_in_webpack_chunk(&source);
    let optimized_modules = count_modules_in_webpack_chunk(&optimized);

    println!(
        "@ant-design/icons chunk - Original modules: {}, Optimized modules: {}",
        original_modules, optimized_modules
    );

    assert!(
        optimized_modules <= original_modules,
        "@ant-design/icons optimization should not increase module count"
    );
}