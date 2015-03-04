use super::super::super::ast::{
    Expression,
    Function,
    FuncRule,
    Literal,
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
        => iexpr(true);

    literal_false
        : &["false"]
        => iexpr(false);

    dereference
        : &["x"]
        => iexpr("x");

    dangling_keyword
        : &["object"]
        => None;

    empty_object
        : &["object {}",
            "object { }",
            "object {\n}"]
        => iexpr(Object::empty());

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
        => iexpr(Function::empty());

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
        => iexpr(
            Function(
                vec![
                    FuncRule {
                        pattern: Pattern::Bind("x".to_string()),
                        body: Expression::Dereference("x".to_string()),
                    }]));

    func_braces_malformed
        : &["object {func{}}",
            "object {func\n{}}",
            "func{ }",
            "func\n{}"]
        => None;

    properties
        : &["object { prop .t -> true; prop .f -> false; prop (x) -> x }"]
        => iexpr(
            Properties::from_items(
                vec![
                    ("t".to_string(),
                     Box::new(Expression::Literal(Literal::Bool(true)))),
                    ("f".to_string(),
                     Box::new(Expression::Literal(Literal::Bool(false))))],
                Some(
                    ("x".to_string(),
                     Box::new(Expression::Dereference("x".to_string()))))));

    concrete_properties
        : &["object { prop .t -> true; prop .f -> false }"]
        => iexpr(
            Properties::from_items(
                vec![
                    ("t".to_string(),
                     Box::new(Expression::Literal(Literal::Bool(true)))),
                    ("f".to_string(),
                     Box::new(Expression::Literal(Literal::Bool(false))))],
                None))
}
