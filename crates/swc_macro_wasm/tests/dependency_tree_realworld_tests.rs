// Tests for nested dependency tree structures and real-world chunks using share-usage.json

use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use swc_macro_wasm::webpack_parser::DependencyNode;

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

fn max_depth(node: &DependencyNode) -> usize {
    if node.dependencies.is_empty() {
        1
    } else {
        1 + node
            .dependencies
            .iter()
            .map(|c| max_depth(c))
            .max()
            .unwrap_or(0)
    }
}

fn node_count(node: &DependencyNode) -> usize {
    1 + node.dependencies.iter().map(|c| node_count(c)).sum::<usize>()
}

#[test]
fn test_dependency_tree_nested_structure_synthetic_json_wrapper() {
    // Create a synthetic chunk with branching and a cycle: root -> A, B; A -> C, D; D -> F; B -> E; E -> root (cycle)
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["nested"], {
            "root": function(module, exports, __webpack_require__) {
                __webpack_require__("A");
                __webpack_require__("B");
            },
            "A": function(module, exports, __webpack_require__) {
                __webpack_require__("C");
                __webpack_require__("D");
            },
            "B": function(module, exports, __webpack_require__) {
                __webpack_require__("E");
            },
            "C": function(module, exports, __webpack_require__) {},
            "D": function(module, exports, __webpack_require__) {
                __webpack_require__("F");
            },
            "E": function(module, exports, __webpack_require__) {
                __webpack_require__("root");
            },
            "F": function(module, exports, __webpack_require__) {}
        }]);
    "#;

    // Use the wasm-exposed API to build dependency tree and return JSON
    let json = swc_macro_wasm::get_webpack_dependency_tree(content, "root");

    // Deserialize into the shared struct
    let tree: DependencyNode = serde_json::from_str(&json).expect("tree JSON should deserialize");

    // Validate structure
    assert_eq!(tree.id, "root");
    assert_eq!(tree.dependencies.len(), 2);

    let a = tree.dependencies.iter().find(|n| n.id == "A").expect("A child exists");
    let b = tree.dependencies.iter().find(|n| n.id == "B").expect("B child exists");

    assert_eq!(a.dependencies.len(), 2);
    assert!(a.dependencies.iter().any(|n| n.id == "C"));
    let d = a.dependencies.iter().find(|n| n.id == "D").expect("D child exists");
    assert_eq!(d.dependencies.len(), 1);
    assert_eq!(d.dependencies[0].id, "F");

    assert_eq!(b.dependencies.len(), 1);
    let e = &b.dependencies[0];
    assert_eq!(e.id, "E");
    // Cycle back to root should be marked with cycle=true and no further children
    assert_eq!(e.dependencies.len(), 1);
    let cycle = &e.dependencies[0];
    assert_eq!(cycle.id, "root");
    assert_eq!(cycle.cycle, Some(true));
    assert!(cycle.dependencies.is_empty());

    // Basic depth and node count sanity
    let depth = max_depth(&tree);
    let count = node_count(&tree);
    assert!(depth >= 4, "expected depth >= 4, got {}", depth);
    assert!(count >= 7, "expected at least 7 nodes including cycle node, got {}", count);
}

#[test]
fn test_dependency_tree_real_world_from_share_usage() {
    let usage = load_share_usage();
    let tree_shake = usage
        .get("treeShake")
        .expect("share-usage.json is expected to contain treeShake root object");

    // Select a few representative packages to keep test runtime reasonable
    let candidates = [
        "react-router-dom",
        "react-redux",
        "@ant-design/icons",
    ];

    for pkg_name in candidates.iter() {
        let Some(pkg_cfg) = tree_shake.get(*pkg_name) else { continue; };
        let Some(chars) = pkg_cfg.get("chunk_characteristics") else { continue; };
        let Some(entry_module_id) = chars.get("entry_module_id").and_then(|v| v.as_str()) else { continue; };
        let Some(files) = chars.get("chunk_files").and_then(|v| v.as_array()) else { continue; };
        if files.is_empty() { continue; }
        let first_chunk = match files[0].as_str() { Some(s) => s, None => continue };

        let content = load_chunk_file(first_chunk);
        let json = swc_macro_wasm::get_webpack_dependency_tree(&content, entry_module_id);

        // Ensure it is valid JSON and matches the entry id
        let tree: DependencyNode = match serde_json::from_str(&json) {
            Ok(t) => t,
            Err(e) => panic!("Failed to parse dependency tree for {}: {}\njson: {}", pkg_name, e, json),
        };

        assert_eq!(tree.id, entry_module_id, "entry id should match for {}", pkg_name);
        // Sanity: we should have some dependencies for non-trivial packages
        assert!(tree.dependencies.len() >= 1, "expected non-empty dependencies for {}", pkg_name);

        // Depth should be modest but > 1 for these packages
        let depth = max_depth(&tree);
        assert!(depth > 1, "expected depth > 1 for {}", pkg_name);

        // For visibility in CI logs: print some summary info without enforcing strict bounds
        let count = node_count(&tree);
        println!("Dependency tree for {}: depth={}, total_nodes={} (entry={})", pkg_name, depth, count, entry_module_id);
    }
}

#[test]
fn test_dependency_tree_missing_module_returns_error_json() {
    // Minimal valid webpack-like chunk
    let content = r#"
        (self[\"webpackChunk_test\"] = self[\"webpackChunk_test\"] || []).push([[\"x\"], {
            \"main.js\": function(module, exports, __webpack_require__) {
                __webpack_require__(\"a.js\");
            },
            \"a.js\": function(module, exports, __webpack_require__) {}
        }]);
    "#;

    let json = swc_macro_wasm::get_webpack_dependency_tree(content, "missing-entry.js");
    let v: serde_json::Value = serde_json::from_str(&json).expect("should be JSON");
    assert!(v.get("error").is_some(), "expected an error JSON when start module missing, got: {}", json);
}