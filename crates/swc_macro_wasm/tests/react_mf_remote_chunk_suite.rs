use std::fs;
use std::path::{Path, PathBuf};

use swc_macro_wasm::optimize;

fn read_fixture(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed reading {}: {}", path.display(), e))
}

fn join_fixture(parts: &[&str]) -> PathBuf {
    let mut p = PathBuf::from("tests/fixtures/react_mf_remote");
    for part in parts { p.push(part); }
    p
}

#[test]
fn optimize_each_remote_chunk_independently_using_share_usage() {
    let share_usage_path = join_fixture(&["share-usage.json"]);
    assert!(share_usage_path.exists(), "Missing fixtures/share-usage.json at {}", share_usage_path.display());

    let data: serde_json::Value = serde_json::from_str(&read_fixture(&share_usage_path))
        .expect("Invalid share-usage.json");

    let Some(tree_shake) = data.get("treeShake").and_then(|v| v.as_object()) else {
        panic!("share-usage.json missing treeShake object");
    };

    for (library_name, flags_val) in tree_shake.iter() {
        let Some(flags_obj) = flags_val.as_object() else { continue };
        let Some(characteristics) = flags_obj.get("chunk_characteristics").cloned() else { continue };
        let Some(chunk_files) = characteristics
            .get("chunk_files")
            .and_then(|v| v.as_array()) else { continue };

        let used_true_count = flags_obj.iter().filter(|(k, v)| k.as_str() != "chunk_characteristics" && v.as_bool() == Some(true)).count();

        for file_val in chunk_files {
            let Some(file_name) = file_val.as_str() else { continue };
            let chunk_path = join_fixture(&[file_name]);
            if !chunk_path.exists() {
                eprintln!("Skipping missing remote chunk fixture: {}", chunk_path.display());
                continue;
            }

            let original = read_fixture(&chunk_path);
            assert!(!original.is_empty(), "Fixture chunk is empty: {}", chunk_path.display());

            let mut lib_config = flags_obj.clone();
            assert!(lib_config.get("chunk_characteristics").is_some(), "{} missing chunk_characteristics in share-usage.json", library_name);

            let config = serde_json::json!({
                "treeShake": {
                    library_name: serde_json::Value::Object(lib_config)
                }
            });

            let optimized = optimize(original.clone(), &serde_json::to_string(&config).unwrap());
            assert!(!optimized.is_empty(), "Optimized output is empty for {}", chunk_path.display());

            // Structure kept
            let has_webpack_chunk = optimized.contains("webpackChunk");
            let has_require = optimized.contains("__webpack_require__");
            assert!(has_webpack_chunk || has_require, "Optimized chunk lost webpack structure: {}", chunk_path.display());

            if used_true_count > 0 {
                for (export_name, is_used) in flags_obj.iter() {
                    if export_name == "chunk_characteristics" { continue; }
                    if is_used.as_bool() == Some(true) {
                        assert!(optimized.contains(export_name),
                            "Kept export '{}' for '{}' not found in optimized chunk {}",
                            export_name, library_name, chunk_path.display());
                    }
                }
            }
        }
    }
}


