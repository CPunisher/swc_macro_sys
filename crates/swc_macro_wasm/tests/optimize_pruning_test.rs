use swc_macro_wasm::optimize::{optimize_with_prune_result, PruneResult};
use serde_json::json;

fn create_test_webpack_chunk(modules: Vec<(&str, Vec<&str>)>) -> String {
    let mut module_defs = Vec::new();
    
    for (id, deps) in modules {
        let dep_requires = deps.iter()
            .map(|d| format!(r#"__webpack_require__("{}")"#, d))
            .collect::<Vec<_>>()
            .join("; ");
            
        module_defs.push(format!(
            r#""{id}": (module, exports, __webpack_require__) => {{
                {dep_requires}
                module.exports = {{ test: "module_{id}" }};
            }}"#,
            id = id,
            dep_requires = if dep_requires.is_empty() { 
                "// no dependencies".to_string() 
            } else { 
                dep_requires 
            }
        ));
    }
    
    format!(
        r#"(self["webpackChunktest"] = self["webpackChunktest"] || []).push([
            ["test-chunk"],
            {{
                {modules}
            }}
        ]);"#,
        modules = module_defs.join(",\n                ")
    )
}

#[test]
fn test_basic_module_pruning() {
    // Create a simple dependency graph: 
    // entry (100) -> dep1 (101) -> dep2 (102)
    //             -> unused1 (103)
    // unused2 (104) -> unused3 (105)
    
    let source = create_test_webpack_chunk(vec![
        ("100", vec!["101"]),  // entry module
        ("101", vec!["102"]),  // reachable dependency
        ("102", vec![]),       // reachable leaf
        ("103", vec![]),       // unreachable module
        ("104", vec!["105"]),  // unreachable module with deps
        ("105", vec![]),       // unreachable leaf
    ]);
    
    let config = json!({
        "treeShake": {
            "test-lib": {
                "export1": true,
                "chunk_characteristics": {
                    "entry_module_id": "100"
                }
            }
        }
    });
    
    let (optimized, prune_result) = optimize_with_prune_result(source, config);
    
    // Verify pruning results
    assert_eq!(prune_result.original_count, 6, "Should have 6 original modules");
    assert_eq!(prune_result.pruned_count, 3, "Should prune 3 modules");
    assert_eq!(prune_result.kept_modules.len(), 3, "Should keep 3 modules");
    
    // Check kept modules are the reachable ones
    assert!(prune_result.kept_modules.contains(&"100".to_string()));
    assert!(prune_result.kept_modules.contains(&"101".to_string()));
    assert!(prune_result.kept_modules.contains(&"102".to_string()));
    
    // Check removed modules are the unreachable ones
    assert!(prune_result.removed_modules.contains(&"103".to_string()));
    assert!(prune_result.removed_modules.contains(&"104".to_string()));
    assert!(prune_result.removed_modules.contains(&"105".to_string()));
    
    // Verify the optimized output doesn't contain pruned modules
    assert!(optimized.contains(r#""100""#), "Should contain module 100");
    assert!(optimized.contains(r#""101""#), "Should contain module 101");
    assert!(optimized.contains(r#""102""#), "Should contain module 102");
    assert!(!optimized.contains(r#""103""#), "Should not contain module 103");
    assert!(!optimized.contains(r#""104""#), "Should not contain module 104");
    assert!(!optimized.contains(r#""105""#), "Should not contain module 105");
}

#[test]
fn test_single_library_config() {
    // Test that optimizer handles single library config (as JS passes it)
    // entry (200) -> shared (202) -> dep (203)
    // unused1 (201)
    // unused2 (204)
    
    let source = create_test_webpack_chunk(vec![
        ("200", vec!["202"]),  // entry
        ("201", vec![]),       // unused module
        ("202", vec!["203"]),  // shared module
        ("203", vec![]),       // dependency of shared
        ("204", vec![]),       // unused module
    ]);
    
    // Config as passed by JavaScript - only one library at a time
    let config = json!({
        "treeShake": {
            "my-library": {
                "export1": true,
                "export2": false,
                "chunk_characteristics": {
                    "entry_module_id": "200"
                }
            }
        }
    });
    
    let (_optimized, prune_result) = optimize_with_prune_result(source, config);
    
    // Only reachable modules from entry 200 should be kept
    assert_eq!(prune_result.kept_modules.len(), 3, "Should keep 3 modules");
    assert_eq!(prune_result.pruned_count, 2, "Should prune 2 modules");
    
    assert!(prune_result.kept_modules.contains(&"200".to_string()));
    assert!(prune_result.kept_modules.contains(&"202".to_string()));
    assert!(prune_result.kept_modules.contains(&"203".to_string()));
    assert!(prune_result.removed_modules.contains(&"201".to_string()));
    assert!(prune_result.removed_modules.contains(&"204".to_string()));
}

#[test]
fn test_circular_dependencies() {
    // Test handling of circular dependencies
    // entry (300) -> a (301) -> b (302) -> a (301) [circular]
    // unused (303)
    
    let source = create_test_webpack_chunk(vec![
        ("300", vec!["301"]),  // entry
        ("301", vec!["302"]),  // part of cycle
        ("302", vec!["301"]),  // part of cycle (circular ref)
        ("303", vec![]),       // unused
    ]);
    
    let config = json!({
        "treeShake": {
            "test": {
                "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "300"
                }
            }
        }
    });
    
    let (_optimized, prune_result) = optimize_with_prune_result(source, config);
    
    // Circular dependencies should be handled correctly
    assert_eq!(prune_result.kept_modules.len(), 3, "Should keep 3 modules in cycle");
    assert_eq!(prune_result.pruned_count, 1, "Should prune 1 module");
    
    assert!(prune_result.kept_modules.contains(&"300".to_string()));
    assert!(prune_result.kept_modules.contains(&"301".to_string()));
    assert!(prune_result.kept_modules.contains(&"302".to_string()));
    assert!(prune_result.removed_modules.contains(&"303".to_string()));
}

#[test]
fn test_no_entry_module_in_chunk() {
    // Test when entry module is not in the chunk
    let source = create_test_webpack_chunk(vec![
        ("400", vec!["401"]),
        ("401", vec![]),
    ]);
    
    let config = json!({
        "treeShake": {
            "test": {
                "export": true,
                "chunk_characteristics": {
                    "entry_module_id": "999" // non-existent entry
                }
            }
        }
    });
    
    let (_optimized, prune_result) = optimize_with_prune_result(source, config);
    
    // Should skip pruning when entry is not found
    assert!(prune_result.skip_reason.is_some(), "Should have skip reason");
    assert!(prune_result.skip_reason.unwrap().contains("not found in chunk"));
    assert_eq!(prune_result.pruned_count, 0, "Should not prune anything");
}

#[test]
fn test_isolated_module_groups() {
    // Test multiple disconnected module groups
    // entry (500) -> dep1 (501)
    // isolated1 (502) -> isolated2 (503)
    // isolated3 (504) standalone
    
    let source = create_test_webpack_chunk(vec![
        ("500", vec!["501"]),  // entry group
        ("501", vec![]),       // entry group leaf
        ("502", vec!["503"]),  // isolated group
        ("503", vec![]),       // isolated group leaf
        ("504", vec![]),       // standalone isolated
    ]);
    
    let config = json!({
        "treeShake": {
            "lib": {
                "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "500"
                }
            }
        }
    });
    
    let (_optimized, prune_result) = optimize_with_prune_result(source, config);
    
    // Only entry group should be kept
    assert_eq!(prune_result.kept_modules.len(), 2, "Should keep only entry group");
    assert_eq!(prune_result.pruned_count, 3, "Should prune isolated modules");
    
    assert!(prune_result.kept_modules.contains(&"500".to_string()));
    assert!(prune_result.kept_modules.contains(&"501".to_string()));
    assert!(prune_result.removed_modules.contains(&"502".to_string()));
    assert!(prune_result.removed_modules.contains(&"503".to_string()));
    assert!(prune_result.removed_modules.contains(&"504".to_string()));
}

#[test]
fn test_complex_dependency_tree() {
    // Test a more complex tree structure
    //        entry (600)
    //        /    |    \
    //    a(601) b(602) c(603)
    //      |      |      |
    //    d(604) e(605) f(606)
    //             |
    //           g(607)
    // unused (608) -> unusedDep (609)
    
    let source = create_test_webpack_chunk(vec![
        ("600", vec!["601", "602", "603"]),  // entry with 3 deps
        ("601", vec!["604"]),                // branch a
        ("602", vec!["605"]),                // branch b
        ("603", vec!["606"]),                // branch c
        ("604", vec![]),                     // leaf d
        ("605", vec!["607"]),                // node e
        ("606", vec![]),                     // leaf f
        ("607", vec![]),                     // leaf g
        ("608", vec!["609"]),                // unused branch
        ("609", vec![]),                     // unused leaf
    ]);
    
    let config = json!({
        "treeShake": {
            "complex": {
                "main": true,
                "chunk_characteristics": {
                    "entry_module_id": "600"
                }
            }
        }
    });
    
    let (_optimized, prune_result) = optimize_with_prune_result(source, config);
    
    // All modules in the tree should be kept, unused branch pruned
    assert_eq!(prune_result.kept_modules.len(), 8, "Should keep entire tree");
    assert_eq!(prune_result.pruned_count, 2, "Should prune unused branch");
    
    // Check all tree modules are kept
    for id in ["600", "601", "602", "603", "604", "605", "606", "607"] {
        assert!(prune_result.kept_modules.contains(&id.to_string()), 
                "Module {} should be kept", id);
    }
    
    // Check unused modules are pruned
    assert!(prune_result.removed_modules.contains(&"608".to_string()));
    assert!(prune_result.removed_modules.contains(&"609".to_string()));
}

#[test]
fn test_empty_config() {
    // Test with no tree-shake config
    let source = create_test_webpack_chunk(vec![
        ("700", vec![]),
        ("701", vec![]),
    ]);
    
    let config = json!({});
    
    let (_optimized, prune_result) = optimize_with_prune_result(source, config);
    
    // Should skip with appropriate message
    assert!(prune_result.skip_reason.is_some());
    assert_eq!(prune_result.pruned_count, 0);
}

#[test]
fn test_malformed_webpack_chunk() {
    // Test with invalid JavaScript that can't be parsed
    let source = "this is not valid javascript {{{{";
    
    let config = json!({
        "treeShake": {
            "test": {
                "chunk_characteristics": {
                    "entry_module_id": "100"
                }
            }
        }
    });
    
    let (optimized, prune_result) = optimize_with_prune_result(source.to_string(), config);
    
    // Should return original source when parsing fails
    assert_eq!(optimized, source);
    assert!(prune_result.skip_reason.is_some());
    assert!(prune_result.skip_reason.unwrap().contains("Parsing failed"));
}

#[test]
fn test_all_modules_reachable() {
    // Test when all modules are reachable from entry
    let source = create_test_webpack_chunk(vec![
        ("800", vec!["801", "802"]),
        ("801", vec!["802"]),
        ("802", vec![]),
    ]);
    
    let config = json!({
        "treeShake": {
            "full": {
                "all": true,
                "chunk_characteristics": {
                    "entry_module_id": "800"
                }
            }
        }
    });
    
    let (_optimized, prune_result) = optimize_with_prune_result(source, config);
    
    // No modules should be pruned
    assert_eq!(prune_result.kept_modules.len(), 3);
    assert_eq!(prune_result.pruned_count, 0);
    assert!(prune_result.removed_modules.is_empty());
}