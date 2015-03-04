use super::super::super::ast::{
    Expression,
    FuncRule,
    Function,
    List,
    Object,
    Pattern,
    Proc,
    ProcExpression,
    Properties,
    PureLeafExpression,
    StatementBlock,
};
use super::framework::{
    expr,
    expr_list,
    propitem,
    qapp,
    qexpr,
    qexpr_list,
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
            "proc",
            "prop",
            "query",
            "return",
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

    query_to_false
        : &["object { query -> false }",
            "object {\n query ->\n  false\n}",
            "query -> false"]
        => Some(expr(query(false)));

    query_to_qapp
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

    query_and_func
        : &["object { query -> $x; func x -> x }"]
        => Some(
            expr(
                Object {
                    proc_: None,
                    query: Some(query(qapp("x"))),
                    func: Function(
                        vec![
                            FuncRule {
                                pattern: Pattern::Bind("x".to_string()),
                                body: expr("x"),
                            }]),
                    props: Properties::empty(),
                }));

    query_and_props_with_var_prop
        : &["object { query -> $x; prop .t -> true; prop (x) -> x }"]
        => Some(
            expr(
                Object {
                    proc_: None,
                    query: Some(query(qapp("x"))),
                    func: Function::empty(),
                    props: Properties::from_items(
                        vec![
                            propitem("t", expr(true)),
                            ],
                        Some(propitem("x", expr("x"))))
                }));

    query_and_props_without_var_prop
        : &["object { query -> $x; prop .t -> true }"]
        => Some(
            expr(
                Object {
                    proc_: None,
                    query: Some(query(qapp("x"))),
                    func: Function::empty(),
                    props: Properties::from_items(
                        vec![
                            propitem("t", expr(true)),
                            ],
                        None)
                }));

    func_and_props_with_varprop
        : &["object { func x -> x; prop .t -> true; prop (x) -> x }"]
        => Some(
            expr(
                Object {
                    proc_: None,
                    query: None,
                    func: Function(
                        vec![
                            FuncRule {
                                pattern: Pattern::Bind("x".to_string()),
                                body: expr("x"),
                            }]),
                    props: Properties::from_items(
                        vec![
                            propitem("t", expr(true)),
                            ],
                        Some(propitem("x", expr("x")))),
                }));

    func_and_props_without_varprop
        : &["object { func x -> x; prop .t -> true }"]
        => Some(
            expr(
                Object {
                    proc_: None,
                    query: None,
                    func: Function(
                        vec![
                            FuncRule {
                                pattern: Pattern::Bind("x".to_string()),
                                body: expr("x"),
                            }]),
                    props: Properties::from_items(
                        vec![
                            propitem("t", expr(true)),
                            ],
                        None),
                }));

    full_object
        : &["object { proc { return !x }; query -> $x; func x -> x; prop .t -> true; prop (x) -> x }"]
        => Some(
            expr(
                Object {
                    proc_: Some(
                        Proc(
                            StatementBlock::Return(
                                Box::new(
                                    ProcExpression::ProcApp(
                                        Box::new(
                                            ProcExpression::PLE(
                                                PureLeafExpression::Dereference(
                                                    "x".to_string())))))))),
                    query: Some(query(qapp("x"))),
                    func: Function(
                        vec![
                            FuncRule {
                                pattern: Pattern::Bind("x".to_string()),
                                body: expr("x"),
                            }]),
                    props: Properties::from_items(
                        vec![
                            propitem("t", expr(true)),
                            ],
                        Some(propitem("x", expr("x")))),
                }));

    bad_arrows
        // Notice that newline immediately after an arrow is acceptable.
        : &["func x-> x",
            "func x ->x",
            "func x\n-> x",
            "object { prop .foo-> bar }",
            "object { prop .foo ->bar }",
            "object { prop .foo\n-> bar }"]
        => None;

    list_expression_empty
        : &["[]", "[ ]", "[\n]"]
        => Some(Expression::LE(List(vec![])));

    list_expression_single
        : &["[false]", "[ false ]", "[\n false\n]"]
        => Some(expr_list(vec![false]));

    list_expression_pair
        : &["[false,true]",
            "[false, true]",
            "[ false, true ]",
            "[\n  false,\n  true\n]"]
        => Some(expr_list(vec![false, true]));

    query_list_expression_single
        : &["query -> [$x]"]
        => Some(expr(query(qexpr_list(vec![qapp("x")]))));

    query_list_expression_pair
        : &["query -> [$x, y]"]
        => Some(expr(query(qexpr_list(vec![qapp("x"), qexpr("y")]))))
}
