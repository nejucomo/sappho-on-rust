use super::super::super::ast::{
    DGrammar,
    Expression,
    Function,
    List,
    Object,
    Pattern,
    Proc,
    Properties,
    StatementBlock,
};
use super::framework::{
    // see mod.rs for test_parse_expectations! macro.
    dgram,
    dgram_patitem,
    pgram,
    pgram_papp,
    pgram_qapp,
    propitem,
    qgram,
    qgram_qapp,
    query,
};

// Test cases:
test_parse_expectations! {
    literal_true
        : &["true"]
        => Some(dgram(true));

    literal_false
        : &["false"]
        => Some(dgram(false));

    dereference
        : &["x"]
        => Some(dgram("x"));

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
        => Some(dgram(Object::empty()));

    object_braces_malformed
        : &["object\n{}",
            "object\t{}",
            "object{}"]
        => None;

    proc_returns_false
        : &["object { proc { return false } }",
            "object {proc {return false}}",
            "proc { return false }"]
        => Some(
            dgram(
                Proc(
                    StatementBlock::Return(
                        Box::new(
                            pgram(false))))));

    proc_with_specialized_applications
        : &["proc { return [!x, $y, z] }"]
        => Some(
            dgram(
                Proc(
                    StatementBlock::Return(
                        Box::new(
                            pgram(
                                vec![
                                    pgram_papp("x"),
                                    pgram_qapp("y"),
                                    pgram("z")]))))));

    query_to_false
        : &["object { query -> false }",
            "object {\n query ->\n  false\n}",
            "query -> false"]
        => Some(dgram(query(false)));

    query_to_qqapp
        : &["object { query -> $x }",
            "object {\n query ->\n  $x\n}",
            "query -> $x"]
        => Some(dgram(query(qgram_qapp("x"))));

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
        => Some(dgram(Function::empty()));

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
            dgram(
                dgram_patitem(
                    Pattern::Bind("x".to_string()),
                    "x")));

    func_braces_malformed
        : &["object {func{}}",
            "object {func\n{}}",
            "func{ }",
            "func\n{}"]
        => None;

    properties
        : &["object { prop .t -> true; prop .f -> false; prop (x) -> x }"]
        => Some(
            dgram(
                Properties::from_items(
                    vec![
                        propitem("t", dgram(true)),
                        propitem("f", dgram(false)),
                        ],
                    Some(propitem("x", dgram("x"))))));

    concrete_properties
        : &["object { prop .t -> true; prop .f -> false }",
            "object {\n prop .t ->\n  true;\n prop .f ->\n  false\n}"]
        => Some(
            dgram(
                Properties::from_items(
                    vec![
                        propitem("t", dgram(true)),
                        propitem("f", dgram(false)),
                        ],
                    None)));

    query_and_func
        : &["object { query -> $x; func x -> x }"]
        => Some(
            dgram(
                Object {
                    proc_: None,
                    query: Some(query(qgram_qapp("x"))),
                    func: Function(
                        vec![
                            dgram_patitem(
                                Pattern::Bind("x".to_string()),
                                "x")]),
                    props: Properties::empty(),
                }));

    query_and_props_with_var_prop
        : &["object { query -> $x; prop .t -> true; prop (x) -> x }"]
        => Some(
            dgram(
                Object {
                    proc_: None,
                    query: Some(query(qgram_qapp("x"))),
                    func: Function::empty(),
                    props: Properties::from_items(
                        vec![
                            propitem("t", dgram(true)),
                            ],
                        Some(propitem("x", dgram("x"))))
                }));

    query_and_props_without_var_prop
        : &["object { query -> $x; prop .t -> true }"]
        => Some(
            dgram(
                Object {
                    proc_: None,
                    query: Some(query(qgram_qapp("x"))),
                    func: Function::empty(),
                    props: Properties::from_items(
                        vec![
                            propitem("t", dgram(true)),
                            ],
                        None)
                }));

    func_and_props_with_varprop
        : &["object { func x -> x; prop .t -> true; prop (x) -> x }"]
        => Some(
            dgram(
                Object {
                    proc_: None,
                    query: None,
                    func: Function(
                        vec![
                            dgram_patitem(
                                Pattern::Bind("x".to_string()),
                                "x")]),
                    props: Properties::from_items(
                        vec![
                            propitem("t", dgram(true)),
                            ],
                        Some(propitem("x", dgram("x")))),
                }));

    func_and_props_without_varprop
        : &["object { func x -> x; prop .t -> true }"]
        => Some(
            dgram(
                Object {
                    proc_: None,
                    query: None,
                    func: Function(
                        vec![
                            dgram_patitem(
                                Pattern::Bind("x".to_string()),
                                "x")]),
                    props: Properties::from_items(
                        vec![
                            propitem("t", dgram(true)),
                            ],
                        None),
                }));

    full_object
        : &["object { proc { return !x }; query -> $x; func x -> x; prop .t -> true; prop (x) -> x }"]
        => Some(
            dgram(
                Object {
                    proc_: Some(
                        Proc(
                            StatementBlock::Return(
                                Box::new(
                                    pgram_papp("x"))))),
                    query: Some(query(qgram_qapp("x"))),
                    func: Function(
                        vec![
                            dgram_patitem(
                                Pattern::Bind("x".to_string()),
                                "x")]),
                    props: Properties::from_items(
                        vec![
                            propitem("t", dgram(true)),
                            ],
                        Some(propitem("x", dgram("x")))),
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
        => Some(DGrammar::Expr(Expression::List(List(vec![]))));

    list_expression_single
        : &["[false]", "[ false ]", "[\n false\n]"]
        => Some(dgram(vec![false]));

    list_expression_pair
        : &["[false,true]",
            "[false, true]",
            "[ false, true ]",
            "[\n  false,\n  true\n]"]
        => Some(dgram(vec![false, true]));

    query_list_expression_single
        : &["query -> [$x]"]
        => Some(dgram(query(vec![qgram_qapp("x")])));

    query_list_expression_pair
        : &["query -> [$x, y]"]
        => Some(dgram(query(vec![qgram_qapp("x"), qgram("y")])))
}
