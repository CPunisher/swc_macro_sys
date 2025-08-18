use swc_macro_wasm::{optimize, optimize_with_prune_result_json};

#[test]
fn test_undefined_conditions_wasm_safe_defaults() {
    let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([["chunk"], {
  "./icons.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.d(__webpack_exports__, {
      ExplicitFalse: () => (/* @common:if [condition="treeShake.@ant-design/icons.ExplicitFalse"] */ _ExplicitFalse__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
      ExplicitTrue: () => (/* @common:if [condition="treeShake.@ant-design/icons.ExplicitTrue"] */ _ExplicitTrue__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
      UndefinedIcon: () => (/* @common:if [condition="treeShake.@ant-design/icons.UndefinedIcon"] */ _UndefinedIcon__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */),
      MissingLibraryIcon: () => (/* @common:if [condition="treeShake.@missing-lib/icons.SomeIcon"] */ _MissingLibraryIcon__WEBPACK_IMPORTED_MODULE_3__["default"] /* @common:endif */)
    });
  }
}]);
"#;

    // Config that only defines some conditions
    let config_json = r#"
{
  "treeShake": {
    "@ant-design/icons": {
      "ExplicitFalse": false,
      "ExplicitTrue": true
    }
  }
}
"#;

    let result = optimize(source.to_string(), config_json);
    
    println!("WASM optimization result:\n{}", result);

    // Test explicit conditions work as expected
    assert!(result.contains("ExplicitFalse: ()=>null"), 
            "ExplicitFalse should be null when condition is false");
    assert!(result.contains("ExplicitTrue: ()=>_ExplicitTrue__WEBPACK_IMPORTED_MODULE_1__"), 
            "ExplicitTrue should keep module reference when condition is true");
    
    // CRITICAL: Test undefined conditions default to true (safe behavior)
    assert!(result.contains("UndefinedIcon: ()=>_UndefinedIcon__WEBPACK_IMPORTED_MODULE_2__"), 
            "UndefinedIcon should keep module reference when condition is undefined (safe default)");
    assert!(result.contains("MissingLibraryIcon: ()=>_MissingLibraryIcon__WEBPACK_IMPORTED_MODULE_3__"), 
            "MissingLibraryIcon should keep module reference when library is not in config (safe default)");

    // Verify undefined conditions do NOT become null
    assert!(!result.contains("UndefinedIcon: ()=>null"), 
            "UndefinedIcon should NOT be null - undefined conditions must default to true for safety");
    assert!(!result.contains("MissingLibraryIcon: ()=>null"), 
            "MissingLibraryIcon should NOT be null - missing library should default to safe behavior");
}

#[test]
fn test_completely_empty_config_wasm() {
    let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([["chunk"], {
  "./test.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.d(__webpack_exports__, {
      Icon1: () => (/* @common:if [condition="treeShake.@ant-design/icons.Icon1"] */ _Icon1__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
      Icon2: () => (/* @common:if [condition="treeShake.lodash.debounce"] */ _debounce__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
      Icon3: () => (/* @common:if [condition="treeShake.@emotion/react.css"] */ _css__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */)
    });
  }
}]);
"#;

    // Completely empty config
    let config_json = "{}";

    let result_json = optimize_with_prune_result_json(source, config_json);
    let result: serde_json::Value = serde_json::from_str(&result_json).expect("Should parse result JSON");
    
    println!("Empty config result:\n{}", serde_json::to_string_pretty(&result).unwrap());

    let optimized_source = result["optimized_source"].as_str().unwrap();
    
    // With empty config, ALL conditions should default to true (safe behavior)
    assert!(optimized_source.contains("Icon1: ()=>_Icon1__WEBPACK_IMPORTED_MODULE_0__"), 
            "Icon1 should keep module reference with empty config (safe default)");
    assert!(optimized_source.contains("Icon2: ()=>_debounce__WEBPACK_IMPORTED_MODULE_1__"), 
            "Icon2 should keep module reference with empty config (safe default)");
    assert!(optimized_source.contains("Icon3: ()=>_css__WEBPACK_IMPORTED_MODULE_2__"), 
            "Icon3 should keep module reference with empty config (safe default)");

    // Verify NONE become null
    assert!(!optimized_source.contains("Icon1: ()=>null"), 
            "Icon1 should NOT be null with empty config");
    assert!(!optimized_source.contains("Icon2: ()=>null"), 
            "Icon2 should NOT be null with empty config");
    assert!(!optimized_source.contains("Icon3: ()=>null"), 
            "Icon3 should NOT be null with empty config");

    // Verify pruning was skipped (no chunk characteristics)
    if let Some(prune_result) = result.get("prune_result") {
        if let Some(skip_reason) = prune_result.get("skip_reason") {
            assert!(skip_reason.as_str().unwrap().contains("No modules pruned"), 
                    "Pruning should be skipped with empty config");
        }
        if let Some(pruned_count) = prune_result.get("pruned_count") {
            assert_eq!(pruned_count.as_u64().unwrap(), 0, 
                      "No modules should be pruned with empty config");
        }
    }
}

#[test]
fn test_mixed_defined_undefined_conditions_wasm() {
    let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([["chunk"], {
  "./mixed.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.d(__webpack_exports__, {
      DefinedFalse: () => (/* @common:if [condition="treeShake.@ant-design/icons.DefinedFalse"] */ _DefinedFalse__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
      DefinedTrue: () => (/* @common:if [condition="treeShake.@ant-design/icons.DefinedTrue"] */ _DefinedTrue__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
      UndefinedInSameLib: () => (/* @common:if [condition="treeShake.@ant-design/icons.UndefinedInSameLib"] */ _UndefinedInSameLib__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */),
      CompletelyMissingLib: () => (/* @common:if [condition="treeShake.@totally-missing/lib.SomeExport"] */ _CompletelyMissingLib__WEBPACK_IMPORTED_MODULE_3__["default"] /* @common:endif */)
    });
  }
}]);
"#;

    let config_json = r#"
{
  "treeShake": {
    "@ant-design/icons": {
      "DefinedFalse": false,
      "DefinedTrue": true
    }
  }
}
"#;

    let result = optimize(source.to_string(), config_json);
    
    println!("Mixed conditions result:\n{}", result);

    // Defined conditions should work as specified
    assert!(result.contains("DefinedFalse: ()=>null"), 
            "DefinedFalse should be null when explicitly set to false");
    assert!(result.contains("DefinedTrue: ()=>_DefinedTrue__WEBPACK_IMPORTED_MODULE_1__"), 
            "DefinedTrue should keep module reference when explicitly set to true");
    
    // Undefined conditions in the same library should default to true (safe)
    assert!(result.contains("UndefinedInSameLib: ()=>_UndefinedInSameLib__WEBPACK_IMPORTED_MODULE_2__"), 
            "UndefinedInSameLib should keep module reference when not defined in config (safe default)");
    
    // Completely missing library should default to true (safe)
    assert!(result.contains("CompletelyMissingLib: ()=>_CompletelyMissingLib__WEBPACK_IMPORTED_MODULE_3__"), 
            "CompletelyMissingLib should keep module reference when library is not in config (safe default)");

    // Verify only the explicitly false condition becomes null
    assert!(!result.contains("UndefinedInSameLib: ()=>null"), 
            "UndefinedInSameLib should NOT be null - undefined should default to safe behavior");
    assert!(!result.contains("CompletelyMissingLib: ()=>null"), 
            "CompletelyMissingLib should NOT be null - missing library should default to safe behavior");
}