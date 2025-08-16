use serde_json::json;

// We use the public re-exports from lib.rs
use swc_macro_wasm::{optimize_with_prune_result, PruneResult};

fn make_jsonp_chunk() -> String {
    r#"
(self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["test"], {
  "main.js": function(module, exports, __webpack_require__) {
    __webpack_require__("a.js");
    exports.run = function() {};
  },
  "a.js": function(module, exports, __webpack_require__) {
    __webpack_require__("c.js");
  },
  "b.js": function(module, exports, __webpack_require__) {
    // unused
  },
  "c.js": function(module, exports, __webpack_require__) {}
}]);
"#
    .to_string()
}

#[test]
fn test_optimize_with_prune_result_basic_pruning() {
    let source = make_jsonp_chunk();

    // Configure explicit entry module id for pruning
    let config = json!({
        "treeShake": {
            "test": {
                "chunk_characteristics": {
                    "entry_module_id": "main.js"
                }
            }
        }
    });

    let (optimized, result): (String, PruneResult) = optimize_with_prune_result(source.clone(), config);

    // Ensure pruning happened with no skip reason
    assert!(result.skip_reason.is_none(), "Expected pruning to occur, skip_reason: {:?}", result.skip_reason);
    assert_eq!(result.original_count, 4, "Original module count should be 4");

    // main.js reaches a.js and c.js; b.js is unused
    // Note: order is not guaranteed; assert by set membership
    assert!(result.kept_modules.contains(&"main.js".to_string()));
    assert!(result.kept_modules.contains(&"a.js".to_string()));
    assert!(result.kept_modules.contains(&"c.js".to_string()));
    assert!(result.removed_modules.contains(&"b.js".to_string()));

    // pruned_count should equal removed_modules length
    assert_eq!(result.pruned_count, result.removed_modules.len());

    // The optimized output should not include the removed module literal, and should include kept ones
    assert!(!optimized.contains("\"b.js\""), "Removed module b.js should not appear in optimized output: {}", optimized);
    assert!(optimized.contains("\"main.js\""));
    assert!(optimized.contains("\"a.js\""));
    assert!(optimized.contains("\"c.js\""));
}

#[test]
fn test_optimize_with_prune_result_entry_missing_in_chunk() {
    let source = make_jsonp_chunk();

    // Entry module id that does not exist in the chunk
    let config = json!({
        "treeShake": {
            "test": {
                "chunk_characteristics": {
                    "entry_module_id": "does-not-exist.js"
                }
            }
        }
    });

    let (optimized, result) = optimize_with_prune_result(source.clone(), config);

    // Should skip with a descriptive reason
    assert!(result.skip_reason.is_some(), "Expected skip_reason when entry not found");
    let reason = result.skip_reason.unwrap();
    assert!(reason.contains("Entry module not found in chunk"), "Unexpected skip reason: {}", reason);

    // No pruning should have occurred
    assert_eq!(result.pruned_count, 0);
    assert_eq!(result.original_count, 4);

    // Optimized output should still contain all modules including b.js
    assert!(optimized.contains("\"main.js\""));
    assert!(optimized.contains("\"a.js\""));
    assert!(optimized.contains("\"b.js\""));
    assert!(optimized.contains("\"c.js\""));
}

#[test]
fn test_optimize_with_prune_result_no_entry_configured() {
    let source = make_jsonp_chunk();

    // Missing entry_module_id under chunk_characteristics
    let config = json!({
        "treeShake": {
            "test": {
                "chunk_characteristics": {
                    // intentionally empty
                }
            }
        }
    });

    let (_optimized, result) = optimize_with_prune_result(source, config);

    // Should skip because no entry_module_id is configured
    assert!(result.skip_reason.is_some(), "Expected skip_reason when no entry configured");
    let reason = result.skip_reason.unwrap();
    assert!(reason.contains("No entry module ID configured"), "Unexpected skip reason: {}", reason);

    // No pruning should have occurred
    assert_eq!(result.pruned_count, 0);
    assert_eq!(result.original_count, 4);
}

#[test]
fn test_optimize_with_prune_result_numeric_module_ids() {
    // JSONP-like chunk with numeric module IDs
    let source = r#"
(self["webpackChunk_num"] = self["webpackChunk_num"] || []).push([["num"], {
  1: function(module, exports, __webpack_require__) {
    __webpack_require__("2");
  },
  2: function(module, exports, __webpack_require__) {
    __webpack_require__("3");
  },
  3: function(module, exports, __webpack_require__) {},
  4: function(module, exports, __webpack_require__) {}
}]);
"#
    .to_string();

    let config = json!({
        "treeShake": {
            "pkg": {
                "chunk_characteristics": {
                    "entry_module_id": "1"
                }
            }
        }
    });

    let (_optimized, result): (String, PruneResult) = optimize_with_prune_result(source, config);

    assert!(result.skip_reason.is_none(), "Expected pruning to occur, skip_reason: {:?}", result.skip_reason);
    assert_eq!(result.original_count, 4);

    // Kept modules: 1,2,3; Removed: 4
    assert!(result.kept_modules.iter().any(|m| m == "1"));
    assert!(result.kept_modules.iter().any(|m| m == "2"));
    assert!(result.kept_modules.iter().any(|m| m == "3"));
    assert!(result.removed_modules.iter().any(|m| m == "4"));

    assert_eq!(result.pruned_count, 1);
}

#[test]
fn test_optimize_with_prune_result_non_webpack_chunk_skips() {
    // Valid JS but not a webpack chunk shape
    let source = "var a = 1; function foo() { return a + 1; }".to_string();

    let config = json!({
        "treeShake": {
            "test": { "chunk_characteristics": { "entry_module_id": "main.js" } }
        }
    });

    let (_optimized, result) = optimize_with_prune_result(source, config);

    assert!(result.skip_reason.is_some(), "Expected skip when not a webpack chunk");
    let reason = result.skip_reason.unwrap();
    assert!(reason.contains("Failed to parse webpack chunk"), "Unexpected skip reason: {}", reason);
    assert_eq!(result.pruned_count, 0);
    assert_eq!(result.original_count, 0);
}