use crate::*;
use swc_core::atoms::Atom;

#[test]
fn test_chunk_type_detection() {
    // Test CommonJS Sync detection
    let commonjs_sync_chars = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    assert_eq!(commonjs_sync_chars.determine_chunk_type(), ChunkType::CommonJSSync);
    
    // Test JSONP detection
    let jsonp_chars = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
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
    
    assert_eq!(jsonp_chars.determine_chunk_type(), ChunkType::JSONP);
}

#[test]
fn test_commonjs_module_extraction() {
    let analyzer = WebpackAnalyzer::new();
    
    let source = r#"
        "use strict";
        exports.modules = {
            "module1.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                var dep = __webpack_require__("module2.js");
                const __WEBPACK_DEFAULT_EXPORT__ = "module1";
            },
            "module2.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                const __WEBPACK_DEFAULT_EXPORT__ = "module2";
            }
        };
    "#;
    
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let chunk = analyzer.analyze_chunk(source, characteristics).unwrap();
    
    assert_eq!(chunk.chunk_type, ChunkType::CommonJSSync);
    assert_eq!(chunk.module_count(), 2);
    assert!(chunk.get_module(&Atom::from("module1.js")).is_some());
    assert!(chunk.get_module(&Atom::from("module2.js")).is_some());
}

#[test]
fn test_jsonp_module_extraction() {
    let analyzer = WebpackAnalyzer::new();
    
    let source = r#"
        (self["webpackChunkapp"] = self["webpackChunkapp"] || []).push([["chunk1"], {
            "module1.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                var dep = __webpack_require__("module2.js");
            },
            "module2.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
            }
        }]);
    "#;
    
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
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
    
    let chunk = analyzer.analyze_chunk(source, characteristics).unwrap();
    
    assert_eq!(chunk.chunk_type, ChunkType::JSONP);
    assert_eq!(chunk.module_count(), 2);
    assert!(chunk.get_module(&Atom::from("module1.js")).is_some());
    assert!(chunk.get_module(&Atom::from("module2.js")).is_some());
}

#[test]
fn test_dependency_graph_building() {
    let analyzer = WebpackAnalyzer::new();
    
    let source = r#"
        "use strict";
        exports.modules = {
            "main.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                var util = __webpack_require__("util.js");
                var helper = __webpack_require__("helper.js");
            },
            "util.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                var helper = __webpack_require__("helper.js");
            },
            "helper.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                // No dependencies
            },
            "unused.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                // No dependencies and no dependents
            }
        };
    "#;
    
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let chunk = analyzer.analyze_chunk(source, characteristics).unwrap();
    
    // Test module dependencies
    let main_module = chunk.get_module(&Atom::from("main.js")).unwrap();
    assert!(main_module.depends_on(&Atom::from("util.js")));
    assert!(main_module.depends_on(&Atom::from("helper.js")));
    
    let util_module = chunk.get_module(&Atom::from("util.js")).unwrap();
    assert!(util_module.depends_on(&Atom::from("helper.js")));
    
    let helper_module = chunk.get_module(&Atom::from("helper.js")).unwrap();
    assert!(helper_module.is_depended_on_by(&Atom::from("main.js")));
    assert!(helper_module.is_depended_on_by(&Atom::from("util.js")));
    
    let unused_module = chunk.get_module(&Atom::from("unused.js")).unwrap();
    assert!(!unused_module.has_dependencies());
    assert!(!unused_module.has_dependents());
}

#[test]
fn test_real_world_lodash_chunk() {
    let analyzer = WebpackAnalyzer::new();
    
    // Simplified lodash chunk structure
    let source = r#"
        "use strict";
        exports.modules = {
            "../../node_modules/lodash-es/lodash.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__,
                    "map": () => reexport_map
                });
                var _map_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../node_modules/lodash-es/map.js");
                var reexport_map = _map_js__WEBPACK_IMPORTED_MODULE_0__["default"];
                const __WEBPACK_DEFAULT_EXPORT__ = lodash;
            },
            "../../node_modules/lodash-es/map.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                const __WEBPACK_DEFAULT_EXPORT__ = map;
            }
        };
    "#;
    
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let chunk = analyzer.analyze_chunk(source, characteristics).unwrap();
    
    assert_eq!(chunk.chunk_type, ChunkType::CommonJSSync);
    assert_eq!(chunk.module_count(), 2);
    
    // Test dependency relationships
    let lodash_module = chunk.get_module(&Atom::from("../../node_modules/lodash-es/lodash.js")).unwrap();
    assert!(lodash_module.depends_on(&Atom::from("../../node_modules/lodash-es/map.js")));
    
    let map_module = chunk.get_module(&Atom::from("../../node_modules/lodash-es/map.js")).unwrap();
    assert!(map_module.is_depended_on_by(&Atom::from("../../node_modules/lodash-es/lodash.js")));
}

#[test]
fn test_dependency_graph_orphan_detection() {
    let mut graph = DependencyGraph::new();
    
    // Add modules
    graph.add_module(WebpackModule::new(Atom::from("main.js"), "main source".to_string()));
    graph.add_module(WebpackModule::new(Atom::from("util.js"), "util source".to_string()));
    graph.add_module(WebpackModule::new(Atom::from("helper.js"), "helper source".to_string()));
    graph.add_module(WebpackModule::new(Atom::from("orphan.js"), "orphan source".to_string()));
    
    // Add dependencies
    graph.add_dependency(&Atom::from("main.js"), &Atom::from("util.js"));
    graph.add_dependency(&Atom::from("util.js"), &Atom::from("helper.js"));
    // orphan.js has no dependencies or dependents
    
    let entry_points = vec![Atom::from("main.js")];
    let orphaned = graph.find_orphaned_modules(&entry_points);
    
    assert_eq!(orphaned.len(), 1);
    assert!(orphaned.contains(&Atom::from("orphan.js")));
}

#[test]
fn test_reachability_analysis() {
    let mut graph = DependencyGraph::new();
    
    // Create a chain: main -> util -> helper
    graph.add_module(WebpackModule::new(Atom::from("main.js"), "main source".to_string()));
    graph.add_module(WebpackModule::new(Atom::from("util.js"), "util source".to_string()));
    graph.add_module(WebpackModule::new(Atom::from("helper.js"), "helper source".to_string()));
    
    graph.add_dependency(&Atom::from("main.js"), &Atom::from("util.js"));
    graph.add_dependency(&Atom::from("util.js"), &Atom::from("helper.js"));
    
    let reachable = graph.get_reachable_modules(&Atom::from("main.js"));
    
    assert_eq!(reachable.len(), 3);
    assert!(reachable.contains(&Atom::from("main.js")));
    assert!(reachable.contains(&Atom::from("util.js")));
    assert!(reachable.contains(&Atom::from("helper.js")));
}

#[test]
fn test_extract_explicit_entry_points() {
    use crate::chunk::ShareUsageConfig;
    use swc_core::atoms::Atom;
    
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    
    // Add some test modules
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "main entry code".to_string()));
    chunk.add_module(Atom::from("./src/utils.js"), WebpackModule::new(Atom::from("./src/utils.js"), "util code".to_string()));
    chunk.add_module(Atom::from("./src/components.js"), WebpackModule::new(Atom::from("./src/components.js"), "component code".to_string()));
    
    // Test with explicit entry points configuration
    let config = ShareUsageConfig {
        entry_module_ids: vec![
            Atom::from("./src/index.js"),
            Atom::from("./src/utils.js"),
        ],
        tree_shake: std::collections::HashMap::new(),
    };
    
    let entry_points = chunk.extract_explicit_entry_points(&config);
    
    // Should return exactly the configured entry points that exist in the chunk
    assert_eq!(entry_points.len(), 2);
    assert!(entry_points.contains(&Atom::from("./src/index.js")));
    assert!(entry_points.contains(&Atom::from("./src/utils.js")));
    assert!(!entry_points.contains(&Atom::from("./src/components.js")));
}

#[test]
fn test_extract_explicit_entry_points_missing_modules() {
    use crate::chunk::ShareUsageConfig;
    use swc_core::atoms::Atom;
    
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    
    // Add only one module
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "main entry code".to_string()));
    
    // Configure entry points including a missing module
    let config = ShareUsageConfig {
        entry_module_ids: vec![
            Atom::from("./src/index.js"),
            Atom::from("./src/missing.js"), // This doesn't exist in chunk
        ],
        tree_shake: std::collections::HashMap::new(),
    };
    
    let entry_points = chunk.extract_explicit_entry_points(&config);
    
    // Should only return the entry points that actually exist in the chunk
    assert_eq!(entry_points.len(), 1);
    assert!(entry_points.contains(&Atom::from("./src/index.js")));
    assert!(!entry_points.contains(&Atom::from("./src/missing.js")));
}

#[test]
fn test_extract_explicit_entry_points_with_characteristics() {
    use crate::chunk::ShareUsageConfig;
    use swc_core::atoms::Atom;
    
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    
    // Add test modules
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "main entry code".to_string()));
    chunk.add_module(Atom::from("./src/entry.js"), WebpackModule::new(Atom::from("./src/entry.js"), "entry code".to_string()));
    
    // Set chunk characteristics with entry_module_id
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: true,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "webpack".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: Some("main".to_string()),
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: Some("./src/entry.js".to_string()),
    };
    
    chunk.set_characteristics(characteristics);
    
    // Configure with one explicit entry point
    let config = ShareUsageConfig {
        entry_module_ids: vec![Atom::from("./src/index.js")],
        tree_shake: std::collections::HashMap::new(),
    };
    
    let entry_points = chunk.extract_explicit_entry_points(&config);
    
    // Should return both the configured entry and the one from characteristics
    assert_eq!(entry_points.len(), 2);
    assert!(entry_points.contains(&Atom::from("./src/index.js")));
    assert!(entry_points.contains(&Atom::from("./src/entry.js")));
}

#[test]
fn test_extract_explicit_entry_points_strict_success() {
    use crate::chunk::ShareUsageConfig;
    use swc_core::atoms::Atom;
    
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    
    // Add test modules
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "main entry code".to_string()));
    chunk.add_module(Atom::from("./src/utils.js"), WebpackModule::new(Atom::from("./src/utils.js"), "util code".to_string()));
    
    let config = ShareUsageConfig {
        entry_module_ids: vec![
            Atom::from("./src/index.js"),
            Atom::from("./src/utils.js"),
        ],
        tree_shake: std::collections::HashMap::new(),
    };
    
    let result = chunk.extract_explicit_entry_points_strict(&config);
    
    // Should succeed when all configured entry points exist
    assert!(result.is_ok());
    let entry_points = result.unwrap();
    assert_eq!(entry_points.len(), 2);
    assert!(entry_points.contains(&Atom::from("./src/index.js")));
    assert!(entry_points.contains(&Atom::from("./src/utils.js")));
}

#[test]
fn test_extract_explicit_entry_points_strict_missing_entry() {
    use crate::chunk::ShareUsageConfig;
    use swc_core::atoms::Atom;
    
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    
    // Add only one module
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "main entry code".to_string()));
    
    let config = ShareUsageConfig {
        entry_module_ids: vec![
            Atom::from("./src/index.js"),
            Atom::from("./src/missing.js"), // This doesn't exist
        ],
        tree_shake: std::collections::HashMap::new(),
    };
    
    let result = chunk.extract_explicit_entry_points_strict(&config);
    
    // Should return error when configured entry points are missing
    assert!(result.is_err());
    let error_msg = result.err().unwrap().to_string();
    assert!(error_msg.contains("Missing entry points in chunk"));
    assert!(error_msg.contains("./src/missing.js"));
}

#[test]
fn test_extract_explicit_entry_points_strict_no_config() {
    use crate::chunk::ShareUsageConfig;
    use swc_core::atoms::Atom;
    
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    
    // Add test modules
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "main entry code".to_string()));
    
    // Empty configuration - no explicit entry points
    let config = ShareUsageConfig {
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    };
    
    let result = chunk.extract_explicit_entry_points_strict(&config);
    
    // Should return error when no entry points are configured
    assert!(result.is_err());
    let error_msg = result.err().unwrap().to_string();
    assert!(error_msg.contains("No explicit entry points found in configuration"));
}

#[test]
fn test_extract_explicit_entry_points_no_inference() {
    use crate::chunk::ShareUsageConfig;
    use swc_core::atoms::Atom;
    
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    
    // Add modules with names that might be inferred as entry points by heuristics
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "main entry code".to_string()));
    chunk.add_module(Atom::from("./src/main.js"), WebpackModule::new(Atom::from("./src/main.js"), "main code".to_string()));
    chunk.add_module(Atom::from("./src/app.js"), WebpackModule::new(Atom::from("./src/app.js"), "app code".to_string()));
    chunk.add_module(Atom::from("./src/entry.js"), WebpackModule::new(Atom::from("./src/entry.js"), "entry code".to_string()));
    
    // Empty configuration - should not infer any entry points
    let config = ShareUsageConfig {
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    };
    
    let entry_points = chunk.extract_explicit_entry_points(&config);
    
    // Should return empty Vec - NO filename-based inference
    assert_eq!(entry_points.len(), 0);
}

#[test]
fn test_explicit_entry_points_integration_with_dependency_graph() {
    use crate::chunk::ShareUsageConfig;
    use crate::dependency_graph::DependencyGraph;
    use swc_core::atoms::Atom;
    
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    
    // Add test modules
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "main entry code".to_string()));
    chunk.add_module(Atom::from("./src/utils.js"), WebpackModule::new(Atom::from("./src/utils.js"), "util code".to_string()));
    chunk.add_module(Atom::from("./src/helper.js"), WebpackModule::new(Atom::from("./src/helper.js"), "helper code".to_string()));
    
    // Configure explicit entry points
    let config = ShareUsageConfig {
        entry_module_ids: vec![Atom::from("./src/index.js")],
        tree_shake: std::collections::HashMap::new(),
    };
    
    let entry_points = chunk.extract_explicit_entry_points(&config);
    
    // Test that these entry points work with DependencyGraph::get_reachable_from_multiple
    let mut graph = DependencyGraph::new();
    for (_module_id, module) in &chunk.modules {
        graph.add_module(module.clone());
    }
    
    // Add some dependencies
    graph.add_dependency(&Atom::from("./src/index.js"), &Atom::from("./src/utils.js"));
    graph.add_dependency(&Atom::from("./src/utils.js"), &Atom::from("./src/helper.js"));
    
    let reachable = graph.get_reachable_from_multiple(&entry_points);
    
    // Should be able to reach all modules from the explicit entry point
    assert!(reachable.contains(&Atom::from("./src/index.js")));
    assert!(reachable.contains(&Atom::from("./src/utils.js")));
    assert!(reachable.contains(&Atom::from("./src/helper.js")));
}

#[test]
fn test_tree_shaker_prune_conservative() {
    use crate::chunk::ShareUsageConfig;
    use crate::tree_shaker::TreeShaker;

    // Build a simple chunk with an unreachable module
    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    chunk.add_module(Atom::from("./src/index.js"), WebpackModule::new(Atom::from("./src/index.js"), "".to_string()));
    chunk.add_module(Atom::from("./src/utils.js"), WebpackModule::new(Atom::from("./src/utils.js"), "".to_string()));
    chunk.add_module(Atom::from("./src/unused.js"), WebpackModule::new(Atom::from("./src/unused.js"), "".to_string()));

    // Provide required ChunkCharacteristics with explicit entry
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "webpack".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: Some("./src/index.js".to_string()),
    };
    chunk.set_characteristics(characteristics);

    // Add dependencies: index -> utils
    let mut graph = DependencyGraph::new();
    for (_, m) in &chunk.modules { graph.add_module(m.clone()); }
    graph.add_dependency(&Atom::from("./src/index.js"), &Atom::from("./src/utils.js"));
    // Reinject updated dependencies into chunk modules for consistency in analysis-level view
    // (Normally analyzer builds these; here we simulate minimal state)
    if let Some(idx) = chunk.modules.get_mut(&Atom::from("./src/index.js")) {
        idx.add_dependency(Atom::from("./src/utils.js"));
    }
    if let Some(utils) = chunk.modules.get_mut(&Atom::from("./src/utils.js")) {
        utils.add_dependent(Atom::from("./src/index.js"));
    }

    let shaker = TreeShaker::new();
    let cfg = ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    };

    let result = shaker.prune_chunk(&chunk, &cfg);
    assert!(result.skip_reason.is_none());
    assert!(result.removed_modules.contains(&Atom::from("./src/unused.js")));
    assert!(result.kept_modules.contains(&Atom::from("./src/index.js")));
    assert!(result.kept_modules.contains(&Atom::from("./src/utils.js")));

    let pruned = result.pruned_chunk.expect("pruned chunk should be present");
    assert!(pruned.get_module(&Atom::from("./src/index.js")).is_some());
    assert!(pruned.get_module(&Atom::from("./src/utils.js")).is_some());
    assert!(pruned.get_module(&Atom::from("./src/unused.js")).is_none());
}

#[test]
fn test_tree_shaker_skips_without_entries() {
    use crate::chunk::ShareUsageConfig;
    use crate::tree_shaker::TreeShaker;

    let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "test source".to_string());
    chunk.add_module(Atom::from("./src/a.js"), WebpackModule::new(Atom::from("./src/a.js"), "".to_string()));

    let shaker = TreeShaker::new();
    let cfg = ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    };
    let result = shaker.prune_chunk(&chunk, &cfg);
    assert!(result.skip_reason.is_some());
    assert_eq!(result.original_count, result.pruned_count);
}

#[test]
fn test_tree_shaker_skips_runtime_chunk() {
    use crate::chunk::ShareUsageConfig;
    use crate::tree_shaker::TreeShaker;

    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: true,
        has_runtime: true,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "jsonp".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["runtime.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };

    let mut chunk = WebpackChunk::new_with_characteristics(
        ChunkType::JSONP,
        "source".to_string(),
        characteristics,
    );
    chunk.add_module(Atom::from("./src/runtime.js"), WebpackModule::new(Atom::from("./src/runtime.js"), "".to_string()));

    let shaker = TreeShaker::new();
    let cfg = ShareUsageConfig { 
        entry_module_ids: vec![Atom::from("./src/runtime.js")],
        tree_shake: std::collections::HashMap::new(),
    };
    let result = shaker.prune_chunk(&chunk, &cfg);
    assert!(result.skip_reason.is_some());
    assert_eq!(result.original_count, result.pruned_count);
}