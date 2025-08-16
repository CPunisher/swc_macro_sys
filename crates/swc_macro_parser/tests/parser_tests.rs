use swc_core::{
    common::{FileName, SourceMap, comments::SingleThreadedComments, sync::Lrc},
    ecma::parser::{EsSyntax, Parser, StringInput, Syntax},
};
use swc_macro_parser::MacroParser;

fn parse_js_with_comments(source: &str) -> SingleThreadedComments {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source.to_string());
    let comments = SingleThreadedComments::default();
    let _program = Parser::new(
        Syntax::Es(EsSyntax::default()),
        StringInput::from(&*fm),
        Some(&comments),
    )
    .parse_program()
    .unwrap();
    comments
}

#[test]
fn test_basic_macro_parsing() {
    let source = r#"
        // @common:test
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.namespace, "common");
    assert_eq!(macro_node.directive, "test");
    assert!(macro_node.attrs.is_empty());
}

#[test]
fn test_macro_with_attributes() {
    let source = r#"
        // @common:transform[mode="production" optimize="true"]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.namespace, "common");
    assert_eq!(macro_node.directive, "transform");
    assert_eq!(macro_node.attrs.len(), 2);
    assert_eq!(macro_node.attrs.get("mode").unwrap(), "production");
    assert_eq!(macro_node.attrs.get("optimize").unwrap(), "true");
}

#[test]
fn test_namespace_filtering() {
    let source = r#"
        // @common:test1
        // @other:test2
        // @common:test3
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 2);
    assert_eq!(macros[0].1.directive, "test1");
    assert_eq!(macros[1].1.directive, "test3");
    
    // Test with different namespace
    let parser_other = MacroParser::new("other");
    let macros_other = parser_other.parse(&parse_js_with_comments(source));
    assert_eq!(macros_other.len(), 1);
    assert_eq!(macros_other[0].1.directive, "test2");
}

#[test]
fn test_multiple_attributes() {
    let source = r#"
        // @common:config[env="test" debug="true" version="1.0.0" flag="enabled"]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.attrs.len(), 4);
    assert_eq!(macro_node.attrs.get("env").unwrap(), "test");
    assert_eq!(macro_node.attrs.get("debug").unwrap(), "true");
    assert_eq!(macro_node.attrs.get("version").unwrap(), "1.0.0");
    assert_eq!(macro_node.attrs.get("flag").unwrap(), "enabled");
}

#[test]
fn test_empty_attributes() {
    let source = r#"
        // @common:test[]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert!(macro_node.attrs.is_empty());
}

#[test]
fn test_attributes_with_spaces() {
    let source = r#"
        // @common:test[ key1 = "value1"  key2="value2"  key3 ="value3"]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.attrs.len(), 3);
    assert_eq!(macro_node.attrs.get("key1").unwrap(), "value1");
    assert_eq!(macro_node.attrs.get("key2").unwrap(), "value2");
    assert_eq!(macro_node.attrs.get("key3").unwrap(), "value3");
}

#[test]
fn test_no_macros() {
    let source = r#"
        // This is just a regular comment
        /* Another regular comment */
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 0);
}

#[test]
fn test_malformed_macros_ignored() {
    let source = r#"
        // @:test
        // @common:
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    // All malformed macros should be ignored
    assert_eq!(macros.len(), 0);
}

#[test]
fn test_malformed_attributes_ignored() {
    // Malformed attributes are ignored, but the macro itself is still valid
    let source = r#"
        // @common:test[key="value
        // @common:another[key=unquoted_value]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    // Both macros should be parsed, but with no attributes since they're malformed
    assert_eq!(macros.len(), 2);
    assert_eq!(macros[0].1.directive, "test");
    assert!(macros[0].1.attrs.is_empty());
    assert_eq!(macros[1].1.directive, "another");
    assert!(macros[1].1.attrs.is_empty());
}

#[test]
fn test_valid_macro_without_attributes() {
    // @common without a directive is malformed, but @common:test is valid
    let source = r#"
        // @common:test
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.namespace, "common");
    assert_eq!(macro_node.directive, "test");
    assert!(macro_node.attrs.is_empty());
}

#[test]
fn test_multiline_comments() {
    let source = r#"
        /*
         * @common:test[mode="dev"]
         */
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.directive, "test");
    assert_eq!(macro_node.attrs.get("mode").unwrap(), "dev");
}

#[test]
fn test_trailing_comments() {
    let source = r#"
        const x = 1; // @common:inline[type="trailing"]
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.directive, "inline");
    assert_eq!(macro_node.attrs.get("type").unwrap(), "trailing");
}

#[test]
fn test_complex_directive_names() {
    let source = r#"
        // @test:tree-shake[aggressive="true"]
        // @test:dead_code_elimination
        // @test:bundle123[version="2"]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("test");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 3);
    assert_eq!(macros[0].1.directive, "tree-shake");
    assert_eq!(macros[1].1.directive, "dead_code_elimination");
    assert_eq!(macros[2].1.directive, "bundle123");
}

#[test]
fn test_attributes_with_special_characters() {
    let source = r#"
        // @common:test[path="/usr/local/bin" url="https://example.com"]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.attrs.get("path").unwrap(), "/usr/local/bin");
    assert_eq!(macro_node.attrs.get("url").unwrap(), "https://example.com");
}

#[test]
fn test_limitation_square_brackets_in_values() {
    // Current limitation: values containing ']' will be truncated
    // because the regex stops at the first ']' character
    let source = r#"
        // @common:test[regex="[a-z]*"]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    // This demonstrates the current limitation - the value is truncated at the first ']'
    assert_eq!(macro_node.attrs.get("regex").unwrap(), "[a-z");
}

#[test]
fn test_real_world_define_inline_format() {
    // Test the actual format from the user's example
    let source = r#"
        {/* @common:define-inline [value="device.orientation" default="portrait"] */}
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    assert_eq!(macro_node.namespace, "common");
    assert_eq!(macro_node.directive, "define-inline");
    assert_eq!(macro_node.attrs.len(), 2);
    assert_eq!(macro_node.attrs.get("value").unwrap(), "device.orientation");
    assert_eq!(macro_node.attrs.get("default").unwrap(), "portrait");
}

#[test]
fn test_quoted_attributes_only() {
    // The current regex requires quoted values, unquoted values should be ignored
    let source = r#"
        // @common:test[quoted="value" unquoted=value]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    // Only quoted attributes should be parsed
    assert_eq!(macro_node.attrs.len(), 1);
    assert_eq!(macro_node.attrs.get("quoted").unwrap(), "value");
    assert!(!macro_node.attrs.contains_key("unquoted"));
}

#[test]
fn test_comment_removal_after_parsing() {
    let source = r#"
        // @common:test
        // This is a regular comment
        // @common:another
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let original_comment_count = {
        let (leading, trailing) = comments.borrow_all();
        leading.values().map(|v| v.len()).sum::<usize>() + 
        trailing.values().map(|v| v.len()).sum::<usize>()
    };
    
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    // Should have found 2 macros
    assert_eq!(macros.len(), 2);
    
    // Check that macro comments were removed but regular comment remains
    let (leading, trailing) = comments.borrow_all();
    let remaining_comment_count = leading.values().map(|v| v.len()).sum::<usize>() + 
                                 trailing.values().map(|v| v.len()).sum::<usize>();
    
    assert_eq!(remaining_comment_count, original_comment_count - 2);
}

#[test]
fn test_macro_node_debug_format() {
    let source = r#"
        // @common:test[key="value"]
        const x = 1;
    "#;
    
    let comments = parse_js_with_comments(source);
    let parser = MacroParser::new("common");
    let macros = parser.parse(&comments);
    
    assert_eq!(macros.len(), 1);
    let (_, macro_node) = &macros[0];
    
    // Test that MacroNode can be debug formatted (this tests the Debug derive)
    let debug_str = format!("{macro_node:?}");
    assert!(debug_str.contains("MacroNode"));
    assert!(debug_str.contains("test"));
    assert!(debug_str.contains("common"));
} 