use super::super::super::ast::{
    Function,
    Let,
    Object,
    Pattern,
    Proc,
    Properties,
    StatementBlock,
};
use super::framework::{
    // see mod.rs for test_parse_expectations! macro.
    dispatch,
    expr,
    lookup,
    papp,
    patitem,
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
            "in",
            "let",
            "object",
            "proc",
            "prop",
            "query",
            "return",
            ]
        => None;

    prop_lookup
        : &["a.b", "a .b", "a\n.b"]
        => Some(lookup("a", "b"));

    prop_dispatch
        : &["a.(b)", "a .(b)", "a\n.(b)"]
        => Some(dispatch("a", "b"));

    bad_prop_apps
        : &["a. (b)", "a. b"]
        => None;

    bad_uncallable_apps
        : &["object {}.prop", "$func x -> x", "!let x = 42 in x"]
        => None;

    parens_prop_lookup
        : &["(a).b", "(a) .b", "(\na\n)\n.b"]
        => Some(lookup(expr("a"), "b"));

    parens_uncallable_lookup
        : &["(object {}).b"]
        => Some(lookup(expr(Object::empty()), "b"));

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

    proc_returns_false
        : &["object { proc { return false } }",
            "object {proc {return false}}",
            "proc { return false }"]
        => Some(
            expr(
                Proc(
                    StatementBlock::Return(
                        Box::new(
                            expr(false))))));

    proc_with_specialized_applications
        : &["proc { return [!x, $y, z] }"]
        => Some(
            expr(
                Proc(
                    StatementBlock::Return(
                        Box::new(
                            expr(
                                vec![
                                    expr(papp("x")),
                                    expr(qapp("y")),
                                    expr("z")]))))));

    query_to_false
        : &["object { query -> false }",
            "object {\n query ->\n  false\n}",
            "query -> false"]
        => Some(expr(query(false)));

    query_to_qqapp
        : &["object { query -> $x }",
            "object {\n query ->\n  $x\n}",
            "query -> $x"]
        => Some(expr(query(qapp("x"))));

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
                patitem(
                    Pattern::Bind("x".to_string()),
                    expr("x"))));

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
                            patitem(
                                Pattern::Bind("x".to_string()),
                                expr("x"))]),
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
                            patitem(
                                Pattern::Bind("x".to_string()),
                                expr("x"))]),
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
                            patitem(
                                Pattern::Bind("x".to_string()),
                                expr("x"))]),
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
                                    expr(papp("x")))))),
                    query: Some(query(qapp("x"))),
                    func: Function(
                        vec![
                            patitem(
                                Pattern::Bind("x".to_string()),
                                expr("x"))]),
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
        => Some(expr(()));

    list_expression_single
        : &["[false]", "[ false ]", "[\n false\n]"]
        => Some(expr(vec![false]));

    list_expression_pair
        : &["[false,true]",
            "[false, true]",
            "[ false, true ]",
            "[\n  false,\n  true\n]"]
        => Some(expr(vec![false, true]));

    query_list_expression_single
        : &["query -> [$x]"]
        => Some(expr(query(vec![qapp("x")])));

    query_list_expression_pair
        : &["query -> [$x, y]"]
        => Some(expr(query(vec![expr(qapp("x")), expr("y")])));

    qapp_versus_propapp_precedence
        : &["query -> $x.p"]
        => Some(expr(query(lookup(qapp("x"), "p"))));

    papp_versus_propapp_precedence
        : &["proc { return !x.p }"]
        => Some(
            expr(
                Proc(
                    StatementBlock::Return(
                        Box::new(
                            expr(
                                lookup(papp("x"), "p")))))));

    expr_let_dctx
        : &["let { f = false; t = true } in [f, t]",
            "let {\n  f = false;\n  t = true\n}\nin [f, t]"]
        => Some(
            expr(
                Let {
                    bindings: vec![
                        patitem(
                            Pattern::Bind("f".to_string()),
                            expr(false)),
                        patitem(
                            Pattern::Bind("t".to_string()),
                            expr(true)),
                        ],
                    expr: Box::new(expr(vec!["f", "t"])),
                }));

    expr_let_singleton
        : &["let { f = false } in f",
            "let {\n  f = false\n}\nin f",
            "let f = false in f"]
        => Some(
            expr(
                Let {
                    bindings: vec![
                        patitem(
                            Pattern::Bind("f".to_string()),
                            expr(false)),
                        ],
                    expr: Box::new(expr("f")),
                }));

    expr_let_qctx
        : &["query -> let { x = a; y = $b } in [x, $y]"]
        => Some(
            expr(
                query(
                    Let {
                        bindings: vec![
                            patitem(
                                Pattern::Bind("x".to_string()),
                                expr("a")),
                            patitem(
                                Pattern::Bind("y".to_string()),
                                qapp("b")),
                            ],
                        expr: Box::new(
                            expr(
                                vec![
                                    expr("x"),
                                    expr(qapp("y")),
                                    ])),
                    })));

    expr_let_pctx
        : &["proc { return let { x = a; y = $b; z = !c } in [x, $y, !z] }"]
        => Some(
            expr(
                Proc(
                    StatementBlock::Return(
                        Box::new(
                            expr(
                                Let {
                                    bindings: vec![
                                        patitem(
                                            Pattern::Bind("x".to_string()),
                                            expr("a")),
                                        patitem(
                                            Pattern::Bind("y".to_string()),
                                            qapp("b")),
                                        patitem(
                                            Pattern::Bind("z".to_string()),
                                            papp("c")),
                                        ],
                                    expr: Box::new(
                                        expr(
                                            vec![
                                                expr("x"),
                                                expr(qapp("y")),
                                                expr(papp("z")),
                                                ])),
                                }))))))
}
