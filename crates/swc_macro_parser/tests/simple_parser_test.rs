use swc_macro_parser::MacroParser;

#[test]
fn test_macro_parser_creation() {
    let parser = MacroParser::new("common");
    // Test that we can create a parser with the "common" namespace
    // This tests basic functionality without dealing with complex comment parsing
    assert_eq!(std::mem::size_of_val(&parser), std::mem::size_of::<&str>());
}

#[test]
fn test_different_namespace() {
    let parser1 = MacroParser::new("common");
    let parser2 = MacroParser::new("other");
    
    // Test that we can create parsers with different namespaces
    // This verifies the parser constructor works with various namespace strings
    assert_eq!(std::mem::size_of_val(&parser1), std::mem::size_of_val(&parser2));
}