#[cfg(test)]
mod macro_content_removal_tests {
    use serde_json::json;
    use swc_macro_wasm::optimize_with_prune_result;

    #[test]
    fn test_macro_content_is_removed_when_condition_is_false() {
        let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["test-chunk"],
    {
        "module1": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            "use strict";
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "feature1": () => (/* @common:if [condition="treeShake.mylib.feature1"] */ feature1Func /* @common:endif */),
                "feature2": () => (/* @common:if [condition="treeShake.mylib.feature2"] */ feature2Func /* @common:endif */),
                "feature3": () => (/* @common:if [condition="treeShake.mylib.feature3"] */ feature3Func /* @common:endif */),
                "always": () => (alwaysFunc)
            });
            
            /* @common:if [condition="treeShake.mylib.feature1"] */
            function feature1Func() {
                return "feature1";
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.mylib.feature2"] */
            function feature2Func() {
                return "feature2";
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.mylib.feature3"] */
            function feature3Func() {
                return "feature3";
            }
            /* @common:endif */
            
            function alwaysFunc() {
                return "always";
            }
        }
    }
]);
"#.to_string();

        // Config that disables feature1 and feature3, but keeps feature2
        let config = json!({
            "treeShake": {
                "mylib": {
                    "feature1": false,
                    "feature2": true,
                    "feature3": false,
                    "chunk_characteristics": {
                        "entry_module_id": "module1"
                    }
                }
            }
        });

        let (optimized_source, _prune_result) = optimize_with_prune_result(source, config);
        
        // 1. Verify ALL macro comments are removed regardless of condition
        assert!(!optimized_source.contains("@common:if"), 
            "All @common:if comments should be removed");
        assert!(!optimized_source.contains("@common:endif"), 
            "All @common:endif comments should be removed");
        
        // 2. Verify FUNCTION BODIES for false conditions are REMOVED
        assert!(!optimized_source.contains("feature1Func"), 
            "feature1Func identifier should be removed when condition is false");
        assert!(!optimized_source.contains("function feature1Func"), 
            "feature1Func function definition should be removed");
        
        assert!(!optimized_source.contains("feature3Func"), 
            "feature3Func identifier should be removed when condition is false");
        assert!(!optimized_source.contains("function feature3Func"), 
            "feature3Func function definition should be removed");
        
        // 3. Verify FUNCTION BODIES for true conditions are KEPT (but without macro comments)
        assert!(optimized_source.contains("feature2Func"), 
            "feature2Func should be kept when condition is true");
        assert!(optimized_source.contains("function feature2Func"), 
            "feature2Func function definition should be kept");
        assert!(optimized_source.contains("return \"feature2\""), 
            "feature2 return statement should be kept");
        
        // 4. Verify non-conditional content is always kept
        assert!(optimized_source.contains("alwaysFunc"), 
            "alwaysFunc should always be kept");
        assert!(optimized_source.contains("function alwaysFunc"), 
            "alwaysFunc function definition should be kept");
    }

    #[test]
    fn test_inline_macro_content_removal() {
        let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["test"],
    {
        "icons": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            "use strict";
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (/* @common:if [condition="treeShake.@ant-design/icons.default"] */ generate /* @common:endif */),
                "HomeIcon": () => (/* @common:if [condition="treeShake.@ant-design/icons.HomeIcon"] */ HomeIcon /* @common:endif */),
                "UserIcon": () => (/* @common:if [condition="treeShake.@ant-design/icons.UserIcon"] */ UserIcon /* @common:endif */),
                "SettingsIcon": () => (/* @common:if [condition="treeShake.@ant-design/icons.SettingsIcon"] */ SettingsIcon /* @common:endif */)
            });
            
            /* @common:if [condition="treeShake.@ant-design/icons.default"] */
            function generate() {
                return "generate icons";
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.@ant-design/icons.HomeIcon"] */
            const HomeIcon = { icon: "home" };
            /* @common:endif */
            
            /* @common:if [condition="treeShake.@ant-design/icons.UserIcon"] */
            const UserIcon = { icon: "user" };
            /* @common:endif */
            
            /* @common:if [condition="treeShake.@ant-design/icons.SettingsIcon"] */
            const SettingsIcon = { icon: "settings" };
            /* @common:endif */
        }
    }
]);
"#.to_string();

        // Config that only keeps HomeIcon, removes everything else
        let config = json!({
            "treeShake": {
                "@ant-design/icons": {
                    "default": false,
                    "HomeIcon": true,
                    "UserIcon": false,
                    "SettingsIcon": false,
                    "chunk_characteristics": {
                        "entry_module_id": "icons"
                    }
                }
            }
        });

        let (optimized_source, _prune_result) = optimize_with_prune_result(source, config);
        
        // Verify macro comments are gone
        assert!(!optimized_source.contains("@common:if"));
        assert!(!optimized_source.contains("@common:endif"));
        
        // Verify disabled exports CONTENT are REMOVED (the export names might still be in __webpack_require__.d)
        assert!(!optimized_source.contains("function generate"), 
            "generate function should be removed (default: false)");
        assert!(!optimized_source.contains("const UserIcon"), 
            "UserIcon const declaration should be removed");
        assert!(!optimized_source.contains("const SettingsIcon"), 
            "SettingsIcon const declaration should be removed");
        assert!(!optimized_source.contains("icon: \"user\""), 
            "UserIcon content should be removed");
        assert!(!optimized_source.contains("icon: \"settings\""), 
            "SettingsIcon content should be removed");
        
        // Verify enabled export and its content is KEPT
        assert!(optimized_source.contains("HomeIcon"), 
            "HomeIcon should be kept (HomeIcon: true)");
        assert!(optimized_source.contains("icon: \"home\""), 
            "HomeIcon content should be kept");
    }

    #[test]
    fn test_complex_inline_expressions_in_macros() {
        let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["complex"],
    {
        "module": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "getData": () => (/* @common:if [condition="treeShake.lib.getData"] */ getData /* @common:endif */),
                "processData": () => (/* @common:if [condition="treeShake.lib.processData"] */ processData /* @common:endif */),
                "transformData": () => (/* @common:if [condition="treeShake.lib.transformData"] */ transformData /* @common:endif */)
            });
            
            // Inline conditional require
            /* @common:if [condition="treeShake.lib.processData"] */ var processor = __webpack_require__("processor"); /* @common:endif */
            
            /* @common:if [condition="treeShake.lib.getData"] */
            function getData() {
                return { data: "test" };
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.lib.processData"] */
            function processData(input) {
                return processor.process(input);
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.lib.transformData"] */
            function transformData(input) {
                const result = input;
                /* @common:if [condition="treeShake.lib.transformData"] */
                // This is a nested condition with the same value - should also be removed if false
                console.log("Transforming data");
                /* @common:endif */
                return result;
            }
            /* @common:endif */
            
            // Object with conditional properties
            const config = {
                /* @common:if [condition="treeShake.lib.getData"] */ enableGet: true, /* @common:endif */
                /* @common:if [condition="treeShake.lib.processData"] */ enableProcess: true, /* @common:endif */
                /* @common:if [condition="treeShake.lib.transformData"] */ enableTransform: true, /* @common:endif */
                baseConfig: "always"
            };
        }
    }
]);
"#.to_string();

        // Config that only enables getData
        let config = json!({
            "treeShake": {
                "lib": {
                    "getData": true,
                    "processData": false,
                    "transformData": false,
                    "chunk_characteristics": {
                        "entry_module_id": "module"
                    }
                }
            }
        });

        let (optimized_source, _prune_result) = optimize_with_prune_result(source, config);
        
        // No macro comments should remain
        assert!(!optimized_source.contains("@common:"));
        
        // processData related FUNCTION content should be REMOVED
        assert!(!optimized_source.contains("function processData"), 
            "processData function should be removed");
        assert!(!optimized_source.contains("processor.process"), 
            "processor.process call should be removed");
        assert!(!optimized_source.contains("enableProcess"), 
            "enableProcess property should be removed");
        
        // transformData related FUNCTION content should be REMOVED
        assert!(!optimized_source.contains("function transformData"), 
            "transformData function should be removed");
        assert!(!optimized_source.contains("Transforming data"), 
            "Nested condition content should be removed");
        assert!(!optimized_source.contains("enableTransform"), 
            "enableTransform property should be removed");
        
        // getData related content should be KEPT
        assert!(optimized_source.contains("getData"), 
            "getData should be kept");
        assert!(optimized_source.contains("function getData"), 
            "getData function should be kept");
    }

    #[test]
    fn test_export_definition_with_inline_macros() {
        // This tests the exact pattern from your example
        let source = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([
    ["test"],
    {
        "module": function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            "use strict";
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (/* @common:if [condition="treeShake.@ant-design/icons.default"] */ generate /* @common:endif */),
                "HomeOutlined": () => (/* @common:if [condition="treeShake.@ant-design/icons.HomeOutlined"] */ HomeOutlined /* @common:endif */),
                "UserOutlined": () => (/* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */ UserOutlined /* @common:endif */)
            });
            
            /* @common:if [condition="treeShake.@ant-design/icons.default"] */
            function generate(icon) {
                return { type: icon };
            }
            /* @common:endif */
            
            /* @common:if [condition="treeShake.@ant-design/icons.HomeOutlined"] */
            const HomeOutlined = generate("home");
            /* @common:endif */
            
            /* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */
            const UserOutlined = generate("user");
            /* @common:endif */
        }
    }
]);
"#.to_string();

        // Test with default: false, should remove the generate reference
        let config = json!({
            "treeShake": {
                "@ant-design/icons": {
                    "default": false,
                    "HomeOutlined": true,
                    "UserOutlined": true,
                    "chunk_characteristics": {
                        "entry_module_id": "module"
                    }
                }
            }
        });

        let (optimized_source, _prune_result) = optimize_with_prune_result(source, config);
        
        // Macro comments should be gone
        assert!(!optimized_source.contains("@common:"));
        
        // When default is false, the word "generate" should be removed from export definition
        // The export line should not contain the reference to generate
        assert!(!optimized_source.contains("=> (generate)"), 
            "Export definition for default should not contain 'generate' when condition is false");
        assert!(!optimized_source.contains("function generate"), 
            "generate function should be removed when default is false");
        
        // HomeOutlined and UserOutlined should be kept
        assert!(optimized_source.contains("HomeOutlined"), 
            "HomeOutlined should be kept");
        assert!(optimized_source.contains("UserOutlined"), 
            "UserOutlined should be kept");
    }
}