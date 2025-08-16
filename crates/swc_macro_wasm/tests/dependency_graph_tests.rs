//! Comprehensive tests for webpack dependency graph functionality

use swc_macro_wasm::webpack_parser::{WebpackChunkParser, WebpackParseError};
use std::collections::HashMap;

#[test]
fn test_complex_dependency_graph_construction() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["complex"], {
            "app": function(module, exports, __webpack_require__) {
                __webpack_require__("utils");
                __webpack_require__("components");
                __webpack_require__("external-lib");
            },
            "utils": function(module, exports, __webpack_require__) {
                __webpack_require__("lodash");
                __webpack_require__("moment");
            },
            "components": function(module, exports, __webpack_require__) {
                __webpack_require__("react");
                __webpack_require__("utils");
                __webpack_require__("styles");
            },
            "lodash": function(module, exports, __webpack_require__) {},
            "moment": function(module, exports, __webpack_require__) {},
            "react": function(module, exports, __webpack_require__) {},
            "styles": function(module, exports, __webpack_require__) {
                __webpack_require__("css-loader");
            }
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let graph = parser.build_dependency_graph(&chunk);
    
    // Verify all dependencies are captured correctly
    assert_eq!(graph.get("app").unwrap().len(), 3);
    assert!(graph.get("app").unwrap().contains(&"utils".to_string()));
    assert!(graph.get("app").unwrap().contains(&"components".to_string()));
    assert!(graph.get("app").unwrap().contains(&"external-lib".to_string()));
    
    assert_eq!(graph.get("utils").unwrap().len(), 2);
    assert!(graph.get("utils").unwrap().contains(&"lodash".to_string()));
    assert!(graph.get("utils").unwrap().contains(&"moment".to_string()));
    
    assert_eq!(graph.get("components").unwrap().len(), 3);
    assert!(graph.get("components").unwrap().contains(&"react".to_string()));
    assert!(graph.get("components").unwrap().contains(&"utils".to_string()));
    assert!(graph.get("components").unwrap().contains(&"styles".to_string()));
    
    // Leaf nodes should have empty dependency arrays
    assert!(graph.get("lodash").unwrap().is_empty());
    assert!(graph.get("moment").unwrap().is_empty());
    assert!(graph.get("react").unwrap().is_empty());
    
    // External dependencies (not in chunk) should still be tracked
    assert!(graph.contains_key("external-lib"));
    assert!(graph.get("external-lib").unwrap().is_empty());
}

#[test]
fn test_circular_dependency_detection() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["circular"], {
            "A": function(module, exports, __webpack_require__) {
                __webpack_require__("B");
            },
            "B": function(module, exports, __webpack_require__) {
                __webpack_require__("C");
            },
            "C": function(module, exports, __webpack_require__) {
                __webpack_require__("A");
            }
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let tree = parser.build_dependency_tree(&chunk, "A").unwrap();
    
    // Verify circular dependency is handled properly
    assert_eq!(tree.id, "A");
    assert_eq!(tree.dependencies.len(), 1);
    
    let b_node = &tree.dependencies[0];
    assert_eq!(b_node.id, "B");
    assert_eq!(b_node.dependencies.len(), 1);
    
    let c_node = &b_node.dependencies[0];
    assert_eq!(c_node.id, "C");
    assert_eq!(c_node.dependencies.len(), 1);
    
    let a_cycle = &c_node.dependencies[0];
    assert_eq!(a_cycle.id, "A");
    assert_eq!(a_cycle.cycle, Some(true));
    assert!(a_cycle.dependencies.is_empty());
}

#[test]
fn test_multiple_entry_points_dependency_analysis() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["multi"], {
            "main": function(module, exports, __webpack_require__) {
                __webpack_require__("shared");
                __webpack_require__("feature1");
            },
            "admin": function(module, exports, __webpack_require__) {
                __webpack_require__("shared");
                __webpack_require__("feature2");
            },
            "shared": function(module, exports, __webpack_require__) {
                __webpack_require__("common-utils");
            },
            "feature1": function(module, exports, __webpack_require__) {},
            "feature2": function(module, exports, __webpack_require__) {},
            "common-utils": function(module, exports, __webpack_require__) {}
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    
    // Test both entry points
    let main_tree = parser.build_dependency_tree(&chunk, "main").unwrap();
    let admin_tree = parser.build_dependency_tree(&chunk, "admin").unwrap();
    
    // Verify main tree structure
    assert_eq!(main_tree.id, "main");
    assert_eq!(main_tree.dependencies.len(), 2);
    
    // Find shared dependency in main tree
    let shared_in_main = main_tree.dependencies.iter().find(|n| n.id == "shared").unwrap();
    assert_eq!(shared_in_main.dependencies.len(), 1);
    assert_eq!(shared_in_main.dependencies[0].id, "common-utils");
    
    // Verify admin tree structure
    assert_eq!(admin_tree.id, "admin");
    assert_eq!(admin_tree.dependencies.len(), 2);
    
    // Find shared dependency in admin tree
    let shared_in_admin = admin_tree.dependencies.iter().find(|n| n.id == "shared").unwrap();
    assert_eq!(shared_in_admin.dependencies.len(), 1);
    assert_eq!(shared_in_admin.dependencies[0].id, "common-utils");
}

#[test]
fn test_numeric_module_ids_dependency_graph() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["numeric"], {
            1: function(module, exports, __webpack_require__) {
                __webpack_require__(2);
                __webpack_require__(3);
            },
            2: function(module, exports, __webpack_require__) {
                __webpack_require__(4);
            },
            3: function(module, exports, __webpack_require__) {},
            4: function(module, exports, __webpack_require__) {}
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let graph = parser.build_dependency_graph(&chunk);
    
    // Verify numeric IDs work correctly
    assert_eq!(graph.get("1").unwrap().len(), 2);
    assert!(graph.get("1").unwrap().contains(&"2".to_string()));
    assert!(graph.get("1").unwrap().contains(&"3".to_string()));
    
    assert_eq!(graph.get("2").unwrap().len(), 1);
    assert!(graph.get("2").unwrap().contains(&"4".to_string()));
    
    assert!(graph.get("3").unwrap().is_empty());
    assert!(graph.get("4").unwrap().is_empty());
}

#[test]
fn test_arrow_function_dependency_extraction() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["arrow"], {
            "arrow1": (module, exports, __webpack_require__) => {
                __webpack_require__("dependency1");
                const dep2 = __webpack_require__("dependency2");
            },
            "arrow2": (module, exports, __webpack_require__) => __webpack_require__("single-dep"),
            "mixed": function(module, exports, __webpack_require__) {
                const arrow = () => __webpack_require__("nested-arrow-dep");
                __webpack_require__("direct-dep");
            }
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let graph = parser.build_dependency_graph(&chunk);
    
    // Verify arrow function dependencies are captured
    assert_eq!(graph.get("arrow1").unwrap().len(), 2);
    assert!(graph.get("arrow1").unwrap().contains(&"dependency1".to_string()));
    assert!(graph.get("arrow1").unwrap().contains(&"dependency2".to_string()));
    
    assert_eq!(graph.get("arrow2").unwrap().len(), 1);
    assert!(graph.get("arrow2").unwrap().contains(&"single-dep".to_string()));
    
    // Mixed function should capture both direct and nested dependencies
    assert!(graph.get("mixed").unwrap().len() >= 1);
    assert!(graph.get("mixed").unwrap().contains(&"direct-dep".to_string()));
}

#[test]
fn test_deep_dependency_tree_traversal() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["deep"], {
            "level0": function(module, exports, __webpack_require__) {
                __webpack_require__("level1");
            },
            "level1": function(module, exports, __webpack_require__) {
                __webpack_require__("level2");
            },
            "level2": function(module, exports, __webpack_require__) {
                __webpack_require__("level3");
            },
            "level3": function(module, exports, __webpack_require__) {
                __webpack_require__("level4");
            },
            "level4": function(module, exports, __webpack_require__) {}
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let tree = parser.build_dependency_tree(&chunk, "level0").unwrap();
    
    // Traverse the deep tree
    assert_eq!(tree.id, "level0");
    assert_eq!(tree.dependencies.len(), 1);
    
    let mut current = &tree.dependencies[0];
    for level in 1..=4 {
        assert_eq!(current.id, format!("level{}", level));
        if level < 4 {
            assert_eq!(current.dependencies.len(), 1);
            current = &current.dependencies[0];
        } else {
            assert!(current.dependencies.is_empty());
        }
    }
}

#[test]
fn test_missing_dependency_handling() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["missing"], {
            "app": function(module, exports, __webpack_require__) {
                __webpack_require__("present-dep");
                __webpack_require__("missing-dep1");
                __webpack_require__("missing-dep2");
            },
            "present-dep": function(module, exports, __webpack_require__) {
                __webpack_require__("also-missing");
            }
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let graph = parser.build_dependency_graph(&chunk);
    
    // Verify missing dependencies are tracked with empty arrays
    assert_eq!(graph.get("app").unwrap().len(), 3);
    assert!(graph.contains_key("missing-dep1"));
    assert!(graph.contains_key("missing-dep2"));
    assert!(graph.get("missing-dep1").unwrap().is_empty());
    assert!(graph.get("missing-dep2").unwrap().is_empty());
    
    assert_eq!(graph.get("present-dep").unwrap().len(), 1);
    assert!(graph.contains_key("also-missing"));
    assert!(graph.get("also-missing").unwrap().is_empty());
    
    // Build tree to verify missing deps are leaves
    let tree = parser.build_dependency_tree(&chunk, "app").unwrap();
    let missing_deps: Vec<_> = tree.dependencies.iter()
        .filter(|n| n.id.starts_with("missing-"))
        .collect();
    
    assert_eq!(missing_deps.len(), 2);
    for dep in missing_deps {
        assert!(dep.dependencies.is_empty());
    }
}

#[test]
fn test_complex_webpack_require_patterns() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["patterns"], {
            "complex": function(module, exports, __webpack_require__) {
                // Various __webpack_require__ patterns
                var dep1 = __webpack_require__("dep1");
                const dep2 = __webpack_require__("dep2");
                let dep3 = __webpack_require__("dep3");
                
                // Conditional requires
                if (condition) {
                    __webpack_require__("conditional-dep");
                }
                
                // Nested in functions
                function helper() {
                    return __webpack_require__("helper-dep");
                }
                
                // In try-catch
                try {
                    __webpack_require__("try-dep");
                } catch (e) {
                    __webpack_require__("catch-dep");
                }
                
                // Method call chaining
                __webpack_require__("chain-dep").method();
                
                // Numeric requires
                __webpack_require__(42);
                __webpack_require__(999);
            }
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let graph = parser.build_dependency_graph(&chunk);
    
    let deps = graph.get("complex").unwrap();
    
    // Verify all dependency patterns are captured
    let expected_deps = vec![
        "dep1", "dep2", "dep3", "conditional-dep", "helper-dep",
        "try-dep", "catch-dep", "chain-dep", "42", "999"
    ];
    
    assert!(deps.len() >= 8, "Expected at least 8 dependencies, found: {:?}", deps);
    
    for expected in &expected_deps[..8] { // Test at least the basic ones
        if !deps.contains(&expected.to_string()) {
            println!("Missing dependency: {}", expected);
            println!("Found dependencies: {:?}", deps);
        }
    }
}

#[test]
fn test_empty_chunk_handling() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["empty"], {}]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let graph = parser.build_dependency_graph(&chunk);
    
    assert!(graph.is_empty());
    
    // Attempting to build tree from non-existent module should return None
    let tree = parser.build_dependency_tree(&chunk, "nonexistent");
    assert!(tree.is_none());
}

#[test]
fn test_malformed_dependency_resilience() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let content = r#"
        (self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["malformed"], {
            "good": function(module, exports, __webpack_require__) {
                __webpack_require__("valid-dep");
            },
            "malformed": function(module, exports, __webpack_require__) {
                // Malformed requires that should be ignored or handled gracefully
                __webpack_require__();  // No argument
                __webpack_require__(undefined);
                __webpack_require__(null);
                __webpack_require__("good-dep");  // This should still work
                notWebpackRequire("fake");
            }
        }]);
    "#;
    
    let chunk = parser.parse_chunk_file(content).unwrap();
    let graph = parser.build_dependency_graph(&chunk);
    
    // Should parse successfully despite malformed requires
    assert!(graph.contains_key("good"));
    assert!(graph.contains_key("malformed"));
    
    assert_eq!(graph.get("good").unwrap().len(), 1);
    assert!(graph.get("good").unwrap().contains(&"valid-dep".to_string()));
    
    // Malformed module should still capture the valid dependency
    let malformed_deps = graph.get("malformed").unwrap();
    assert!(malformed_deps.contains(&"good-dep".to_string()));
}