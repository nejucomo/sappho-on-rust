use super::super::parse_verified_expression;

use super::framework::{
    // see $crate::ast::test::framework for test_parse_expectations! macro.
    expr,
    papp,
    proc_return,
    qapp,
    query,
};


// Test cases:
test_parse_expectations! {
    [parse_verified_expression];

    smokecheck
        : &["true"]
        => Some(expr(true));

    // procapp is invalid in D/Q contexts and only valid in P contexts:
    invalid_procapp_in_deterministic_context
        : &["!true"]
        => None;

    invalid_procapp_in_query_context
        : &["query -> !true"]
        => None;

    valid_procapp_in_proc_context
        : &["proc { return !true }"]
        => Some(expr(proc_return(papp(true))));

    // queryapp is invalid in D contexts and only valid in Q/P contexts:
    invalid_queryapp_in_deterministic_context
        : &["$true"]
        => None;

    valid_queryapp_in_proc_context
        : &["proc { return $true }"]
        => Some(expr(proc_return(qapp(true))));

    valid_queryapp_in_query_context
        : &["query -> $true"]
        => Some(expr(query(qapp(true))))
}
