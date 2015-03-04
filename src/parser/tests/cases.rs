use super::super::super::ast::{
    Function,
    FuncRule,
    Object,
    Pattern,
    Properties,
};
use super::framework::{
    iexpr,
};

// Test cases:
test_parse_expectations! {
    literal_true
        : &["true"]
        => Some(iexpr(true));

    literal_false
        : &["false"]
        => Some(iexpr(false));

    dereference
        : &["x"]
        => Some(iexpr("x"));

    dangling_keyword
        : &["object"]
        => None;

    empty_object
        : &["object {}",
            "object { }",
            "object {\n}"]
        => Some(iexpr(Object::empty()));

    object_braces_malformed
        : &["object\n{}",
            "object\t{}",
            "object{}"]
        => None;

    empty_func
        : &["object { func { } }",
            "object {func {}}",
            "object { func {} }",
            "object {func {}}",
            "object {\n  func {\n  }\n}",

            // func-only-object shorthand:
            "func { }",
            "func {}",
            "func {\n}"]
        => Some(iexpr(Function::empty()));

    identity_func
        : &["object { func { x -> x } }",
            "object {func { x -> x }}",
            "object { func {x -> x} }",
            "object {func {x -> x}}",
            "object {\n  func {\n    x -> x\n  }\n}",
            "object {\n  func x -> x\n}",

            // func-only-object shorthand:
            "func { x -> x }",
            "func {x -> x}",
            "func {\n  x -> x\n}",
            "func x -> x"]
        => Some(
            iexpr(
                FuncRule {
                    pattern: Pattern::Bind("x".to_string()),
                    body: iexpr("x"),
                }));

    func_braces_malformed
        : &["object {func{}}",
            "object {func\n{}}",
            "func{ }",
            "func\n{}"]
        => None;

    properties
        : &["object { prop .t -> true; prop .f -> false; prop (x) -> x }"]
        => Some(
            iexpr(
                Properties::from_items(
                    vec![
                        ("t".to_string(),
                         Box::new(iexpr(true))),
                        ("f".to_string(),
                         Box::new(iexpr(false)))],
                    Some(
                        ("x".to_string(),
                         Box::new(iexpr("x")))))));

    concrete_properties
        : &["object { prop .t -> true; prop .f -> false }"]
        => Some(
            iexpr(
                Properties::from_items(
                    vec![
                        ("t".to_string(),
                         Box::new(iexpr(true))),
                        ("f".to_string(),
                         Box::new(iexpr(false)))],
                    None)))
}
