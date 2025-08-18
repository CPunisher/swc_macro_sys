use swc_macro_wasm::optimize;

#[test]
fn test_lodash_export_removal() {
    // Simplified lodash-es webpack chunk with macro comments
    let source = r#"
(self["webpackChunk_mf_react_host"] = self["webpackChunk_mf_react_host"] || []).push([["lodash"], {
"../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js": 
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  add: () => (/* @common:if [condition="treeShake.lodash-es.add"] */ /* reexport safe */ _add_js__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
  after: () => (/* @common:if [condition="treeShake.lodash-es.after"] */ /* reexport safe */ _after_js__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
  assign: () => (/* @common:if [condition="treeShake.lodash-es.assign"] */ /* reexport safe */ _assign_js__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */),
  capitalize: () => (/* @common:if [condition="treeShake.lodash-es.capitalize"] */ /* reexport safe */ _capitalize_js__WEBPACK_IMPORTED_MODULE_3__["default"] /* @common:endif */),
  debounce: () => (/* @common:if [condition="treeShake.lodash-es.debounce"] */ /* reexport safe */ _debounce_js__WEBPACK_IMPORTED_MODULE_4__["default"] /* @common:endif */),
  delay: () => (/* @common:if [condition="treeShake.lodash-es.delay"] */ /* reexport safe */ _delay_js__WEBPACK_IMPORTED_MODULE_5__["default"] /* @common:endif */),
  random: () => (/* @common:if [condition="treeShake.lodash-es.random"] */ /* reexport safe */ _random_js__WEBPACK_IMPORTED_MODULE_6__["default"] /* @common:endif */),
  sortBy: () => (/* @common:if [condition="treeShake.lodash-es.sortBy"] */ /* reexport safe */ _sortBy_js__WEBPACK_IMPORTED_MODULE_7__["default"] /* @common:endif */),
  "default": () => (/* @common:if [condition="treeShake.lodash-es.default"] */ /* reexport safe */ _lodash_default_js__WEBPACK_IMPORTED_MODULE_8__["default"] /* @common:endif */)
});

var _add_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/add.js");
var _after_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/after.js");
var _assign_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/assign.js");
var _capitalize_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/capitalize.js");
var _debounce_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/debounce.js");
var _delay_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/delay.js");
var _random_js__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/random.js");
var _sortBy_js__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/sortBy.js");
var _lodash_default_js__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__("../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.default.js");

})
}]);
"#;

    // Config with add, after, assign set to false
    // Keep capitalize, debounce, delay, random, sortBy, default
    let config = r#"{
        "treeShake": {
            "lodash-es": {
                "add": false,
                "after": false, 
                "assign": false,
                "capitalize": true,
                "debounce": true,
                "delay": true,
                "random": true,
                "sortBy": true,
                "default": true
            }
        }
    }"#;

    // Run optimization
    let optimized = optimize(source.to_string(), config);

    println!("Original size: {} bytes", source.len());
    println!("Optimized size: {} bytes", optimized.len());
    println!("Size reduction: {:.2}%", 
        ((source.len() as f64 - optimized.len() as f64) / source.len() as f64) * 100.0);

    // Verify that false exports are replaced with null
    assert!(optimized.contains("add: ()=>(null)") || optimized.contains("add: ()=>null"), 
        "add export should be null but found: {}", 
        extract_export_line(&optimized, "add"));
    
    assert!(optimized.contains("after: ()=>(null)") || optimized.contains("after: ()=>null"), 
        "after export should be null but found: {}", 
        extract_export_line(&optimized, "after"));
    
    assert!(optimized.contains("assign: ()=>(null)") || optimized.contains("assign: ()=>null"), 
        "assign export should be null but found: {}", 
        extract_export_line(&optimized, "assign"));

    // Verify that true exports still have their module references
    assert!(optimized.contains("_capitalize_js__WEBPACK_IMPORTED_MODULE_3__"), 
        "capitalize export should have module reference");
    
    assert!(optimized.contains("_debounce_js__WEBPACK_IMPORTED_MODULE_4__"), 
        "debounce export should have module reference");
    
    assert!(optimized.contains("_delay_js__WEBPACK_IMPORTED_MODULE_5__"), 
        "delay export should have module reference");
    
    assert!(optimized.contains("_random_js__WEBPACK_IMPORTED_MODULE_6__"), 
        "random export should have module reference");
    
    assert!(optimized.contains("_sortBy_js__WEBPACK_IMPORTED_MODULE_7__"), 
        "sortBy export should have module reference");

    // Verify macro comments are removed
    assert!(!optimized.contains("@common:if"), "Macro comments should be removed");
    assert!(!optimized.contains("@common:endif"), "Macro comments should be removed");

    println!("\n✅ Test passed! Exports marked as false are replaced with null.");
}

fn extract_export_line(content: &str, export_name: &str) -> String {
    content.lines()
        .find(|line| line.contains(&format!("{}: ", export_name)))
        .unwrap_or("not found")
        .to_string()
}

#[test]
fn test_lodash_with_all_exports_false() {
    // Test case where all exports are false
    let source = r#"
__webpack_require__.d(__webpack_exports__, {
  add: () => (/* @common:if [condition="treeShake.lodash-es.add"] */ _add_js__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
  after: () => (/* @common:if [condition="treeShake.lodash-es.after"] */ _after_js__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */)
});
"#;

    let config = r#"{
        "treeShake": {
            "lodash-es": {
                "add": false,
                "after": false
            }
        }
    }"#;

    let optimized = optimize(source.to_string(), config);

    // Both exports should be null
    assert!(optimized.contains("add: ()=>(null)") || optimized.contains("add: ()=>null"), 
        "add export should be null");
    assert!(optimized.contains("after: ()=>(null)") || optimized.contains("after: ()=>null"), 
        "after export should be null");

    println!("\n✅ Test with all exports false passed!");
}

#[test]
fn test_lodash_with_missing_config() {
    // Test case where some exports are not in config (should default to true/keep)
    let source = r#"
__webpack_require__.d(__webpack_exports__, {
  add: () => (/* @common:if [condition="treeShake.lodash-es.add"] */ _add_js__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
  after: () => (/* @common:if [condition="treeShake.lodash-es.after"] */ _after_js__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
  unknown: () => (/* @common:if [condition="treeShake.lodash-es.unknown"] */ _unknown_js__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */)
});
"#;

    let config = r#"{
        "treeShake": {
            "lodash-es": {
                "add": false,
                "after": true
            }
        }
    }"#;

    let optimized = optimize(source.to_string(), config);

    // add should be null (explicitly false)
    assert!(optimized.contains("add: ()=>(null)") || optimized.contains("add: ()=>null"), 
        "add export should be null");
    
    // after should keep module reference (explicitly true)
    assert!(optimized.contains("_after_js__WEBPACK_IMPORTED_MODULE_1__"), 
        "after export should have module reference");
    
    // unknown should keep module reference (not in config, defaults to true)
    assert!(optimized.contains("_unknown_js__WEBPACK_IMPORTED_MODULE_2__"), 
        "unknown export should have module reference (default to keep when not in config)");

    println!("\n✅ Test with missing config passed!");
}