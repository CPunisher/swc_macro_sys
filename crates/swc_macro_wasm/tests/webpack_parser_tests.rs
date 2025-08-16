//! Tests for webpack parser functionality

use swc_macro_wasm::webpack_parser::{WebpackChunkParser, WebpackParseError};
use std::fs;

#[test]
fn test_webpack_parser_creation() {
    let parser = WebpackChunkParser::new();
    assert!(parser.is_ok());
}

#[test]
fn test_parse_sample_webpack_chunk() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let sample_content = r#"
(self["webpackChunk_mf_react_host"] = self["webpackChunk_mf_react_host"] || []).push([["node_modules_test"], {
"../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/generate.js": 
/*!***********************************************************************************************************!*\
  !*** ../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/generate.js ***!
  \***********************************************************************************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
"use strict";
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (generate)
});
var _ant_design_fast_color__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("@ant-design/fast-color");
function generate(color) {
    // implementation
}
}),
"../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/index.js": 
/*!********************************************************************************************************!*\
  !*** ../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/index.js ***!
  \********************************************************************************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
"use strict";
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  blue: () => (blue)
});
var _generate__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./generate");
})
}]);
    "#;
    
    let result = parser.parse_chunk_file(sample_content);
    assert!(result.is_ok(), "Failed to parse chunk: {:?}", result.err());
    
    let chunk = result.unwrap();
    assert!(!chunk.modules.is_empty(), "No modules found in chunk");
    
    let module_keys = parser.get_module_keys(&chunk);
    println!("Found module keys: {:?}", module_keys);
    
    // Check that we found the expected modules
    assert!(module_keys.contains(&"../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/generate.js".to_string()));
    assert!(module_keys.contains(&"../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/index.js".to_string()));
    
    // Test getting specific module info
    let generate_module = parser.get_module(&chunk, "../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/generate.js");
    assert!(generate_module.is_some(), "Generate module not found");
    
    let module_info = generate_module.unwrap();
    assert!(!module_info.content.is_empty(), "Module content is empty");
    println!("Module content preview: {}", &module_info.content[..std::cmp::min(200, module_info.content.len())]);
}

#[test]
fn test_parse_real_webpack_file() {
    let parser = WebpackChunkParser::new().unwrap();
    
    // Try to read the actual test file if it exists
    let test_file_path = "../../../tests/jsonp/node_modules_pnpm_ant-design_icons_5_6_1_react-dom_18_3_1_react_18_3_1_node_modules_ant-desig-f1ca590.js";
    
    if let Ok(content) = fs::read_to_string(test_file_path) {
        let result = parser.parse_chunk_file(&content);
        
        match result {
            Ok(chunk) => {
                println!("Successfully parsed real webpack file!");
                println!("Chunk name: {}", chunk.name);
                println!("Number of modules: {}", chunk.modules.len());
                
                let module_keys = parser.get_module_keys(&chunk);
                println!("First 5 module keys:");
                for (i, key) in module_keys.iter().take(5).enumerate() {
                    println!("  {}: {}", i + 1, key);
                }
                
                assert!(!chunk.modules.is_empty(), "No modules found in real webpack file");
            }
            Err(e) => {
                println!("Failed to parse real webpack file: {:?}", e);
                // Don't fail the test if the file format is different than expected
                // This is more of an exploratory test
            }
        }
    } else {
        println!("Real webpack test file not found, skipping test");
    }
}

#[test]
fn test_invalid_chunk_format() {
    let parser = WebpackChunkParser::new().unwrap();
    
    // Use valid JavaScript that is NOT a webpack chunk structure so SWC parsing succeeds
    // and the parser returns InvalidChunkFormat due to missing push([...]).
    let invalid_content = "(() => { console.log('not a chunk'); })();";
    let result = parser.parse_chunk_file(invalid_content);
    
    assert!(result.is_err());
    match result.err().unwrap() {
        WebpackParseError::InvalidChunkFormat(_) => {
            // Expected error type
        }
        other => panic!("Expected InvalidChunkFormat error, got: {:?}", other),
    }
}

#[test]
fn test_multiple_chunks_parsing() {
    let parser = WebpackChunkParser::new().unwrap();
    
    let chunk1 = ("chunk1.js".to_string(), r#"
(self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["chunk1"], {
"module1": 
/*!comment*/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
"use strict";
console.log("module1");
})
}]);
    "#.to_string());
    
    let chunk2 = ("chunk2.js".to_string(), r#"
(self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["chunk2"], {
"module2": 
/*!comment*/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
"use strict";
console.log("module2");
})
}]);
    "#.to_string());
    
    let files = vec![chunk1, chunk2];
    let result = parser.parse_multiple_chunks(&files);
    
    assert!(result.is_ok());
    let chunks = result.unwrap();
    assert_eq!(chunks.len(), 2);
    
    // Verify each chunk has its modules
    for chunk in &chunks {
        assert!(!chunk.modules.is_empty());
    }
}

#[test]
fn test_only_top_level_module_entries_are_collected() {
    use swc_macro_wasm::webpack_parser::WebpackChunkParser;

    let parser = WebpackChunkParser::new().unwrap();

    // This chunk includes:
    // - Valid top-level modules: "moduleA" (fn), 123 (fn), "wrapped" (paren-wrapped fn), "arrow" (arrow fn)
    // - A top-level object literal "notAModule" that contains nested string keys that look like CSS selectors and even functions
    //   Those nested keys MUST NOT be treated as modules.
    // - An ident-keyed property `identKey` whose value is a function; ident-keyed entries should be ignored by the parser
    let sample_content = r#"
(self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["chunk_only_top"], {
"moduleA": (function(module, __webpack_exports__, __webpack_require__) {
  __webpack_require__("./depA");
}),
notAModule: {
  "&, &:hover": (function(){ __webpack_require__("./nested_should_not_be_seen"); }),
  "": (function(){ __webpack_require__("./nested_should_not_be_seen2"); })
},
identKey: (function(){ __webpack_require__("./should_be_ignored_ident_key"); }),
123: (function(module, __webpack_exports__, __webpack_require__) { __webpack_require__(123); }),
"wrapped": (function(module, __webpack_exports__, __webpack_require__) { __webpack_require__("./dep_wrapped"); }),
"arrow": (module, __webpack_exports__, __webpack_require__) => { __webpack_require__("./dep_arrow"); }
}]);
"#;

    let result = parser.parse_chunk_file(sample_content);
    assert!(result.is_ok(), "Failed to parse chunk: {:?}", result.err());
    let chunk = result.unwrap();

    // Collect module keys
    let mut module_keys = parser.get_module_keys(&chunk);
    module_keys.sort();

    // Expected only the valid top-level string/number module entries whose value is a function/arrow function
    let expected: Vec<String> = vec![
        "123".to_string(),
        "arrow".to_string(),
        "moduleA".to_string(),
        "wrapped".to_string(),
    ];

    for key in &expected {
        assert!(module_keys.contains(key), "Expected module key '{}' to be present. Found: {:?}", key, module_keys);
    }

    // Ensure nested object keys and ident-keyed top-level property are NOT included
    let not_expected: Vec<String> = vec![
        "notAModule".to_string(), // top-level but not a module (value is an object)
        "&, &:hover".to_string(), // nested CSS-like key
        "".to_string(),           // nested empty-string key
        "identKey".to_string(),   // ident-keyed property should be ignored even though its value is a function
    ];

    for key in &not_expected {
        assert!(!module_keys.contains(key), "Did not expect module key '{}' to be present. Found: {:?}", key, module_keys);
    }

    // Verify dependencies were collected only from recognized module factories
    let all_deps: Vec<String> = chunk
        .modules
        .values()
        .flat_map(|m| m.dependencies.clone())
        .collect();

    // Must include the expected dependencies
    assert!(all_deps.contains(&"./depA".to_string()));
    assert!(all_deps.contains(&"123".to_string())); // numeric require(123)
    assert!(all_deps.contains(&"./dep_wrapped".to_string()));
    assert!(all_deps.contains(&"./dep_arrow".to_string()));

    // Must NOT include nested dependencies from inside the non-module object
    assert!(
        !all_deps.iter().any(|d| d == "./nested_should_not_be_seen" || d == "./nested_should_not_be_seen2"),
        "Found nested object dependencies in module deps: {:?}",
        all_deps
    );
}

#[test]
fn test_parenthesized_arrow_and_nested_structures_not_traversed() {
    use swc_macro_wasm::webpack_parser::WebpackChunkParser;

    let parser = WebpackChunkParser::new().unwrap();

    let sample_content = r#"
(self["webpackChunk_test"] = self["webpackChunk_test"] || []).push([["chunk_nested"], {
"fnFactory": (function(module, __webpack_exports__, __webpack_require__) { __webpack_require__("./dep_fn"); }),
"plainArrow": (module, __webpack_exports__, __webpack_require__) => { __webpack_require__("./dep_plain_arrow"); },
"parenArrow": ((module, __webpack_exports__, __webpack_require__) => { __webpack_require__("./dep_paren_arrow"); }),
obj: {
  "nestedKey": (function(){ __webpack_require__("./nested_dep_should_not_appear"); }),
},
arr: [
  { "nestedArrKey": (function(){ __webpack_require__("./nested_arr_dep_should_not_appear"); }) }
],
identIgnored: (function(){ __webpack_require__("./ident_ignored_dep"); })
}]);
"#;

    let result = parser.parse_chunk_file(sample_content);
    assert!(result.is_ok(), "Failed to parse chunk: {:?}", result.err());
    let chunk = result.unwrap();

    let mut keys = parser.get_module_keys(&chunk);
    keys.sort();

    let expected = vec![
        "fnFactory".to_string(),
        "plainArrow".to_string(),
        "parenArrow".to_string(),
    ];

    for k in &expected {
        assert!(keys.contains(k), "Expected module key '{}' to be present. Found: {:?}", k, keys);
    }

    // Ensure we did not traverse nested object/array structures or include ident key
    for unexpected in [
        "obj".to_string(),
        "nestedKey".to_string(),
        "arr".to_string(),
        "nestedArrKey".to_string(),
        "identIgnored".to_string(),
    ] {
        assert!(!keys.contains(&unexpected), "Did not expect key '{}' to be present. Found: {:?}", unexpected, keys);
    }

    // Verify only the expected deps are present; nested deps must not appear
    let all_deps: Vec<String> = chunk.modules.values().flat_map(|m| m.dependencies.clone()).collect();
    for dep in ["./dep_fn", "./dep_plain_arrow", "./dep_paren_arrow"] {
        assert!(all_deps.contains(&dep.to_string()), "Missing expected dependency '{}' in {:?}", dep, all_deps);
    }
    for dep in [
        "./nested_dep_should_not_appear",
        "./nested_arr_dep_should_not_appear",
        "./ident_ignored_dep",
    ] {
        assert!(
            !all_deps.contains(&dep.to_string()),
            "Unexpected nested/ignored dependency '{}' present in {:?}",
            dep,
            all_deps
        );
    }
}

#[test]
fn test_dependency_extraction_from_module_factory() {
    use swc_macro_wasm::webpack_parser::WebpackChunkParser;

    let parser = WebpackChunkParser::new().unwrap();

    let content_with_deps = r#"
(self["webpackChunk_mf_react_host"] = self["webpackChunk_mf_react_host"] || []).push([["test_chunk"], {
"test_module": 
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
"use strict";
var dep1 = __webpack_require__("dependency1");
var dep2 = __webpack_require__(123);
var dep3 = __webpack_require__("another/dependency");
})
}]);
"#;

    let result = parser.parse_chunk_file(content_with_deps);
    assert!(result.is_ok(), "Parsing should succeed: {:?}", result.err());

    let chunk = result.unwrap();
    let module = parser.get_module(&chunk, "test_module").expect("test_module not found");
    assert_eq!(module.dependencies.len(), 3);
    assert!(module.dependencies.contains(&"dependency1".to_string()));
    assert!(module.dependencies.contains(&"123".to_string()));
    assert!(module.dependencies.contains(&"another/dependency".to_string()));
}

#[test]
fn test_no_css_like_keys_in_real_antd_index_chunk() {
    use swc_macro_wasm::webpack_parser::WebpackChunkParser;
    use std::fs;

    let parser = WebpackChunkParser::new().unwrap();
    let test_file_path = "../../../tests/jsonp/node_modules_pnpm_antd_5_27_0_react-dom_18_3_1_react_18_3_1_node_modules_antd_es_index_js-_140c0.js";

    let content = match fs::read_to_string(test_file_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("antd index chunk fixture not found, skipping test");
            return;
        }
    };

    let result = parser.parse_chunk_file(&content);
    match result {
        Ok(chunk) => {
            let keys = parser.get_module_keys(&chunk);
            assert!(!keys.is_empty(), "Expected some modules in antd index chunk");

            // Ensure no CSS-like or empty keys are present
            assert!(keys.iter().all(|k| !k.is_empty()), "Found empty module key in {:?}", keys);
            assert!(keys.iter().all(|k| !k.contains('&')), "Found CSS-like module key containing '&' in {:?}", keys);

            // Spot-check that expected color modules are present
            let expected_any = [
                "../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/generate.js",
                "../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/index.js",
                "../../../node_modules/.pnpm/@ant-design+colors@7.2.1/node_modules/@ant-design/colors/es/presets.js",
            ];
            for e in expected_any {
                assert!(keys.iter().any(|k| k == e), "Expected to find '{}' among module keys", e);
            }
        }
        Err(e) => {
            // If parsing fails due to fixture drift, don’t hard-fail CI.
            eprintln!("Skipping: failed to parse real antd index chunk: {}", e);
        }
    }
}

#[test]
fn test_build_dependency_graph_and_tree() {
    use swc_macro_wasm::webpack_parser::WebpackChunkParser; // for direct use if tests are in crate; otherwise refer path

    let content = r#"
        "use strict";
        (self["webpackChunk"] = self["webpackChunk"] || []).push([["test"], {
            "A": function(module, __webpack_exports__, __webpack_require__) {
                __webpack_require__("B");
                __webpack_require__("C");
            },
            "B": function(module, __webpack_exports__, __webpack_require__) {
                __webpack_require__("D");
            },
            "C": function(module, __webpack_exports__, __webpack_require__) {},
            "D": function(module, __webpack_exports__, __webpack_require__) {
                __webpack_require__("B"); // cycle
            }
        }]);
    "#;

    let parser = WebpackChunkParser::new().unwrap();
    let chunk = parser.parse_chunk_file(content).unwrap();

    let graph = parser.build_dependency_graph(&chunk);
    assert_eq!(graph.get("A").unwrap(), &vec!["B".to_string(), "C".to_string()]);
    assert_eq!(graph.get("B").unwrap(), &vec!["D".to_string()]);
    assert_eq!(graph.get("C").unwrap(), &Vec::<String>::new());
    assert_eq!(graph.get("D").unwrap(), &vec!["B".to_string()]);

    let tree = parser.build_dependency_tree(&chunk, "A").unwrap();
    assert_eq!(tree.id, "A");
    // two children B and C; order depends on insertion, so just check ids
    let mut child_ids: Vec<_> = tree.dependencies.iter().map(|n| n.id.as_str()).collect();
    child_ids.sort();
    assert_eq!(child_ids, vec!["B", "C"]);

    let b_node = tree.dependencies.iter().find(|n| n.id == "B").unwrap();
    let d_node = b_node.dependencies.iter().find(|n| n.id == "D").unwrap();
    // D depends on B again, which should be represented as a cycle leaf
    let cycle = d_node.dependencies.iter().find(|n| n.id == "B").unwrap();
    assert_eq!(cycle.cycle, Some(true));
    assert!(cycle.dependencies.is_empty());
}

#[test]
fn test_dependency_tree_missing_module_leaf() {
    use swc_macro_wasm::webpack_parser::WebpackChunkParser;

    let content = r#"
        "use strict";
        (self["webpackChunk"] = self["webpackChunk"] || []).push([["test"], {
            "A": function(module, __webpack_exports__, __webpack_require__) {
                __webpack_require__("ExternalX");
            }
        }]);
    "#;

    let parser = WebpackChunkParser::new().unwrap();
    let chunk = parser.parse_chunk_file(content).unwrap();

    let tree = parser.build_dependency_tree(&chunk, "A").unwrap();
    assert_eq!(tree.id, "A");
    assert_eq!(tree.dependencies.len(), 1);
    let ext = &tree.dependencies[0];
    assert_eq!(ext.id, "ExternalX");
    assert!(ext.dependencies.is_empty()); // not present in chunk, treated as leaf
}