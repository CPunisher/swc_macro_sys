use swc_macro_wasm::{optimize, optimize_with_prune_result_json};

#[test]
fn test_antd_icons_real_chunk_comprehensive() {
    // Real chunk pattern from @ant-design/icons with macros
    let source = r#"
(self["webpackChunkmodule_federation_react_example_remote"] = self["webpackChunkmodule_federation_react_example_remote"] || []).push([["vendors-node_modules_pnpm_ant-design_icons"], {
  "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.r(__webpack_exports__);
    __webpack_require__.d(__webpack_exports__, {
      AccountBookFilled: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookFilled"] */ _AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
      AccountBookOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookOutlined"] */ _AccountBookOutlined__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
      UserOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */ _UserOutlined__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */),
      DeleteOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.DeleteOutlined"] */ _DeleteOutlined__WEBPACK_IMPORTED_MODULE_3__["default"] /* @common:endif */),
      PlusOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.PlusOutlined"] */ _PlusOutlined__WEBPACK_IMPORTED_MODULE_4__["default"] /* @common:endif */),
      MinusOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.MinusOutlined"] */ _MinusOutlined__WEBPACK_IMPORTED_MODULE_5__["default"] /* @common:endif */)
    });
    var _AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./AccountBookFilled.js");
    var _AccountBookOutlined__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("./AccountBookOutlined.js");
    var _UserOutlined__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__("./UserOutlined.js");
    var _DeleteOutlined__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__("./DeleteOutlined.js");
    var _PlusOutlined__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__("./PlusOutlined.js");
    var _MinusOutlined__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__("./MinusOutlined.js");
  },
  "./node_modules/.pnpm/another-module/index.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.r(__webpack_exports__);
    __webpack_require__.d(__webpack_exports__, {
      someFunction: () => __webpack_require__("./lib/someFunction.js")
    });
  }
}]);
"#;

    // Config where only UserOutlined and DeleteOutlined should be kept
    let config_json = r#"
{
  "treeShake": {
    "@ant-design/icons": {
      "AccountBookFilled": false,
      "AccountBookOutlined": false,
      "UserOutlined": true,
      "DeleteOutlined": true,
      "PlusOutlined": false,
      "MinusOutlined": false,
      "chunk_characteristics": {
        "entry_module_id": "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js"
      }
    }
  }
}
"#;

    // Test the optimization
    let result = optimize(source.to_string(), config_json);
    
    println!("Optimized result:\n{}", result);

    // Test macro evaluation: false conditions should become null
    assert!(result.contains("AccountBookFilled: ()=>(null)") || result.contains("AccountBookFilled: ()=>null"), 
            "AccountBookFilled should be null when condition is false");
    assert!(result.contains("AccountBookOutlined: ()=>(null)") || result.contains("AccountBookOutlined: ()=>null"), 
            "AccountBookOutlined should be null when condition is false");
    assert!(result.contains("PlusOutlined: ()=>(null)") || result.contains("PlusOutlined: ()=>null"), 
            "PlusOutlined should be null when condition is false");
    assert!(result.contains("MinusOutlined: ()=>(null)") || result.contains("MinusOutlined: ()=>null"), 
            "MinusOutlined should be null when condition is false");
    
    // Test macro evaluation: true conditions should keep the original content
    assert!(result.contains("UserOutlined: ()=>(_UserOutlined__WEBPACK_IMPORTED_MODULE_2__") || 
            result.contains("UserOutlined: ()=>_UserOutlined__WEBPACK_IMPORTED_MODULE_2__"), 
            "UserOutlined should keep module reference when condition is true");
    assert!(result.contains("DeleteOutlined: ()=>(_DeleteOutlined__WEBPACK_IMPORTED_MODULE_3__") || 
            result.contains("DeleteOutlined: ()=>_DeleteOutlined__WEBPACK_IMPORTED_MODULE_3__"), 
            "DeleteOutlined should keep module reference when condition is true");

    // Unused modules should be transformed to plain require calls without variable assignment
    // The optimization converts "var _ModuleName__WEBPACK_IMPORTED_MODULE_N__ = __webpack_require__(...)" 
    // to just "__webpack_require__(...)" for unused modules
    assert!(result.contains("__webpack_require__(\"./AccountBookFilled.js\")"), 
            "Unused modules should become plain require calls");
    assert!(result.contains("var _UserOutlined__WEBPACK_IMPORTED_MODULE_2__"), 
            "Used module imports should still have variable assignment");
    
    // The optimization may have pruned the second module since it's unreachable from the entry
    // This is actually correct behavior - unreachable modules should be removed
}

#[test]
fn test_antd_icons_with_prune_result() {
    let source = r#"
(self["webpackChunkmodule_federation_react_example_remote"] = self["webpackChunkmodule_federation_react_example_remote"] || []).push([["vendors-node_modules_pnpm_ant-design_icons"], {
  "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.r(__webpack_exports__);
    __webpack_require__.d(__webpack_exports__, {
      AccountBookFilled: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookFilled"] */ _AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
      UserOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */ _UserOutlined__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */)
    });
    var _AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./AccountBookFilled.js");
    var _UserOutlined__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__("./UserOutlined.js");
  },
  "./component1.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    var icons = __webpack_require__("../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js");
  },
  "./component2.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    var other = __webpack_require__("./other.js");
  },
  "./other.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    console.log("other module");
  }
}]);
"#;

    let config_json = r#"
{
  "treeShake": {
    "@ant-design/icons": {
      "AccountBookFilled": false,
      "UserOutlined": true,
      "chunk_characteristics": {
        "entry_module_id": "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js"
      }
    }
  }
}
"#;

    let result_json = optimize_with_prune_result_json(source, config_json);
    let result: serde_json::Value = serde_json::from_str(&result_json).expect("Should parse result JSON");
    
    println!("Prune result JSON:\n{}", serde_json::to_string_pretty(&result).unwrap());

    // Check that we have optimized source
    assert!(result["optimized_source"].is_string(), "Should have optimized_source");
    
    let optimized_source = result["optimized_source"].as_str().unwrap();
    
    // Verify macro transformations
    assert!(optimized_source.contains("AccountBookFilled: ()=>(null)") || 
            optimized_source.contains("AccountBookFilled: ()=>null"), 
            "AccountBookFilled should be null");
    assert!(optimized_source.contains("UserOutlined: ()=>(_UserOutlined__WEBPACK_IMPORTED_MODULE_2__") || 
            optimized_source.contains("UserOutlined: ()=>_UserOutlined__WEBPACK_IMPORTED_MODULE_2__"), 
            "UserOutlined should keep module reference");

    // Check prune result
    if let Some(prune_result) = result.get("prune_result") {
        if let Some(skip_reason) = prune_result.get("skip_reason") {
            println!("Pruning was skipped: {}", skip_reason);
        } else {
            // If pruning was not skipped, check the results
            if let Some(original_count) = prune_result.get("original_count") {
                assert!(original_count.as_u64().unwrap() > 0, "Should have original modules");
            }
            
            if let Some(kept_modules) = prune_result.get("kept_modules") {
                let kept = kept_modules.as_array().unwrap();
                println!("Kept modules: {}", kept.len());
                // Entry module should be kept
                assert!(kept.iter().any(|m| m.as_str().unwrap().contains("@ant-design/icons")), 
                        "Entry module should be kept");
            }
            
            if let Some(removed_modules) = prune_result.get("removed_modules") {
                let removed = removed_modules.as_array().unwrap();
                println!("Removed modules: {}", removed.len());
            }
        }
    }
}

#[test]
fn test_antd_icons_no_chunk_characteristics() {
    // Test what happens when no chunk_characteristics are provided
    let source = r#"
(self["webpackChunkmodule_federation_react_example_remote"] = self["webpackChunkmodule_federation_react_example_remote"] || []).push([["vendors-node_modules_pnpm_ant-design_icons"], {
  "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.d(__webpack_exports__, {
      AccountBookFilled: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookFilled"] */ _AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */)
    });
  }
}]);
"#;

    let config_json = r#"
{
  "treeShake": {
    "@ant-design/icons": {
      "AccountBookFilled": false
    }
  }
}
"#;

    let result_json = optimize_with_prune_result_json(source, config_json);
    let result: serde_json::Value = serde_json::from_str(&result_json).expect("Should parse result JSON");
    
    println!("Result without chunk characteristics:\n{}", serde_json::to_string_pretty(&result).unwrap());

    // Should still perform macro evaluation
    let optimized_source = result["optimized_source"].as_str().unwrap();
    assert!(optimized_source.contains("AccountBookFilled: ()=>(null)") || 
            optimized_source.contains("AccountBookFilled: ()=>null"), 
            "Macro evaluation should work without chunk characteristics");

    // Pruning MUST be skipped without chunk characteristics - this is non-negotiable
    if let Some(prune_result) = result.get("prune_result") {
        if let Some(skip_reason) = prune_result.get("skip_reason") {
            println!("Pruning correctly skipped: {}", skip_reason);
            // Verify that pruning was actually skipped
            assert!(skip_reason.as_str().unwrap().contains("No modules pruned"), 
                    "Pruning should be skipped without chunk characteristics");
        } else {
            panic!("Pruning should be skipped when chunk_characteristics are missing! This is a critical requirement.");
        }
        
        // Verify no modules were actually pruned
        if let Some(pruned_count) = prune_result.get("pruned_count") {
            assert_eq!(pruned_count.as_u64().unwrap(), 0, 
                      "No modules should be pruned without chunk characteristics");
        }
    } else {
        panic!("prune_result should be present even when pruning is skipped");
    }
}

#[test]
fn test_antd_icons_all_false_conditions() {
    // Test when all icons are marked as false
    let source = r#"
(self["webpackChunkmodule_federation_react_example_remote"] = self["webpackChunkmodule_federation_react_example_remote"] || []).push([["vendors-node_modules_pnpm_ant-design_icons"], {
  "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.d(__webpack_exports__, {
      AccountBookFilled: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookFilled"] */ _AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
      UserOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */ _UserOutlined__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
      DeleteOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.DeleteOutlined"] */ _DeleteOutlined__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */)
    });
  }
}]);
"#;

    let config_json = r#"
{
  "treeShake": {
    "@ant-design/icons": {
      "AccountBookFilled": false,
      "UserOutlined": false,
      "DeleteOutlined": false,
      "chunk_characteristics": {
        "entry_module_id": "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js"
      }
    }
  }
}
"#;

    let result = optimize(source.to_string(), config_json);
    
    println!("Result with all false conditions:\n{}", result);

    // All exports should be null
    assert!(result.contains("AccountBookFilled: ()=>(null)") || result.contains("AccountBookFilled: ()=>null"), 
            "AccountBookFilled should be null");
    assert!(result.contains("UserOutlined: ()=>(null)") || result.contains("UserOutlined: ()=>null"), 
            "UserOutlined should be null");
    assert!(result.contains("DeleteOutlined: ()=>(null)") || result.contains("DeleteOutlined: ()=>null"), 
            "DeleteOutlined should be null");

    // No module references should remain in the exports
    assert!(!result.contains("_AccountBookFilled__WEBPACK_IMPORTED_MODULE_0__[\"default\"]"), 
            "No module references should remain in exports");
    assert!(!result.contains("_UserOutlined__WEBPACK_IMPORTED_MODULE_1__[\"default\"]"), 
            "No module references should remain in exports");
    assert!(!result.contains("_DeleteOutlined__WEBPACK_IMPORTED_MODULE_2__[\"default\"]"), 
            "No module references should remain in exports");
}

#[test]
fn test_antd_icons_mixed_with_other_libraries() {
    // Test @ant-design/icons mixed with other libraries
    let source = r#"
(self["webpackChunkmodule_federation_react_example_remote"] = self["webpackChunkmodule_federation_react_example_remote"] || []).push([["shared-libs"], {
  "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    __webpack_require__.d(__webpack_exports__, {
      UserOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */ _UserOutlined__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */)
    });
  },
  "../../../node_modules/.pnpm/lodash-es/index.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    __webpack_require__.d(__webpack_exports__, {
      debounce: () => (/* @common:if [condition="treeShake.lodash-es.debounce"] */ _debounce__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */)
    });
  }
}]);
"#;

    let config_json = r#"
{
  "treeShake": {
    "@ant-design/icons": {
      "UserOutlined": true
    },
    "lodash-es": {
      "debounce": false
    }
  }
}
"#;

    let result = optimize(source.to_string(), config_json);
    
    println!("Result with mixed libraries:\n{}", result);

    // Ant Design icon should be kept
    assert!(result.contains("UserOutlined: ()=>(_UserOutlined__WEBPACK_IMPORTED_MODULE_0__") || 
            result.contains("UserOutlined: ()=>_UserOutlined__WEBPACK_IMPORTED_MODULE_0__"), 
            "UserOutlined should keep module reference when true");

    // Lodash function should be null
    assert!(result.contains("debounce: ()=>(null)") || result.contains("debounce: ()=>null"), 
            "debounce should be null when false");
}