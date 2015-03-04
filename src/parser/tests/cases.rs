use super::super::super::ast::{
    Function,
    FuncRule,
    Object,
    Pattern,
    Properties,
};
use super::framework::{
    expr,
    propitem,
    qapp,
    query,
};

// Test cases:
test_parse_expectations! {
    literal_true
        : &["true"]
        => Some(expr(true));

    literal_false
        : &["false"]
        => Some(expr(false));

    dereference
        : &["x"]
        => Some(expr("x"));

    dangling_keywords
        : &[
            "func",
            "object",
            "prop",
            "query",
            ]
        => None;

    empty_object
        : &["object {}",
            "object { }",
            "object {\n}"]
        => Some(expr(Object::empty()));

    object_braces_malformed
        : &["object\n{}",
            "object\t{}",
            "object{}"]
        => None;

    query_false
        : &["object { query -> false }",
            "object {\n query ->\n  false\n}",
            "query -> false"]
        => Some(expr(query(false)));

    query_query_x
        : &["object { query -> $x }",
            "object {\n query ->\n  $x\n}",
            "query -> $x"]
        => Some(expr(qapp("x")));

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
        => Some(expr(Function::empty()));

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
            "func x -> x",
            "func x ->\n  x"]
        => Some(
            expr(
                FuncRule {
                    pattern: Pattern::Bind("x".to_string()),
                    body: expr("x"),
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
            expr(
                Properties::from_items(
                    vec![
                        propitem("t", expr(true)),
                        propitem("f", expr(false)),
                        ],
                    Some(propitem("x", expr("x"))))));

    concrete_properties
        : &["object { prop .t -> true; prop .f -> false }",
            "object {\n prop .t ->\n  true;\n prop .f ->\n  false\n}"]
        => Some(
            expr(
                Properties::from_items(
                    vec![
                        propitem("t", expr(true)),
                        propitem("f", expr(false)),
                        ],
                    None)));

    bad_arrows
        // Notice that newline immediately after an arrow is acceptable.
        : &["func x-> x",
            "func x ->x",
            "func x\n-> x",
            "object { prop .foo-> bar }",
            "object { prop .foo ->bar }",
            "object { prop .foo\n-> bar }"]
        => None
}
