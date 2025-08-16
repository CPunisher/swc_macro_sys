use webpack_analyzer_v2::{analyzer::WebpackAnalyzer, chunk::ChunkCharacteristics, chunk::ShareUsageConfig, tree_shaker::TreeShaker};

fn make_jsonp_chars(entry: &str) -> ChunkCharacteristics {
    ChunkCharacteristics {
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
        chunk_files: vec![],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: Some(entry.to_string()),
    }
}

fn make_cjs_chars(entry: &str) -> ChunkCharacteristics {
    ChunkCharacteristics {
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
        chunk_files: vec![],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: Some(entry.to_string()),
    }
}

#[test]
fn jsonp_with_runtime_and_missing_deps() {
    let src = r#"(self["webpackChunkapp"] = self["webpackChunkapp"] || []).push([
  ["id"],
  {
    "./entry.js": function(module, exports, __webpack_require__) {
      __webpack_require__("./present.js");
      __webpack_require__("./external.js"); // not defined in this chunk
      exports.ok = 1;
    },
    "./present.js": function(module, exports) { exports.p = 1; },
    42: function(module, exports, __webpack_require__) { __webpack_require__("./external2.js"); }
  },
  function(runtime) { /* runtime */ }
]);"#;

    let analyzer = WebpackAnalyzer::new();
    let chars = make_jsonp_chars("./entry.js");
    let chunk = analyzer.analyze_chunk(src, chars.clone()).unwrap();
    assert!(chunk.modules.contains_key(&"./entry.js".into()));
    assert!(chunk.modules.contains_key(&"./present.js".into()));
    assert!(chunk.modules.contains_key(&"42".into()));

    let shaker = TreeShaker::new();
    let plan = shaker.plan_prune(&chunk, &ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    });
    assert!(plan.skip_reason.is_none());
    // Only entry and present are reachable; 42 is not referenced by entry
    assert_eq!(plan.pruned_count, 2);
}

#[test]
fn commonjs_sparse_keys_and_missing_deps() {
    let src = r#"exports.modules = {
  "./entry.js": function(module, exports, __webpack_require__) {
    module.exports = __webpack_require__("./inner.js");
  },
  7: function(module, exports, __webpack_require__) {
    __webpack_require__("./extern.js"); // not present
  },
  "./inner.js": function(module, exports) { exports.x = 1; }
};"#;

    let analyzer = WebpackAnalyzer::new();
    let chars = make_cjs_chars("./entry.js");
    let chunk = analyzer.analyze_chunk(src, chars.clone()).unwrap();
    assert!(chunk.modules.contains_key(&"./entry.js".into()));
    assert!(chunk.modules.contains_key(&"./inner.js".into()));
    assert!(chunk.modules.contains_key(&"7".into()));

    let shaker = TreeShaker::new();
    let plan = shaker.plan_prune(&chunk, &ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    });
    assert!(plan.skip_reason.is_none());
    // entry -> inner; numeric 7 is unreachable
    assert_eq!(plan.pruned_count, 2);
}

#[test]
fn reexport_wrapper_entry_keeps_transitive() {
    let src = r#"exports.modules = {
  "./entry.js": function(module, exports, __webpack_require__) {
    module.exports = __webpack_require__("./actual.js");
  },
  "./actual.js": function(module, exports, __webpack_require__) {
    const d = __webpack_require__(`./dep.js`);
    exports.value = d.v;
  },
  "./dep.js": function(module, exports) { exports.v = 1; }
};"#;

    let analyzer = WebpackAnalyzer::new();
    let chars = make_cjs_chars("./entry.js");
    let chunk = analyzer.analyze_chunk(src, chars.clone()).unwrap();
    let shaker = TreeShaker::new();
    let plan = shaker.plan_prune(&chunk, &ShareUsageConfig { 
        entry_module_ids: vec![],
        tree_shake: std::collections::HashMap::new(),
    });
    assert!(plan.skip_reason.is_none());
    // All three are reachable via re-export chain and template literal require
    assert_eq!(plan.pruned_count, 3);
}


