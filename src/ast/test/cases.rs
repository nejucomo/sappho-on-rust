use super::super::parse_verified_expression;

use super::framework::{
    // see $crate::ast::test::framework for test_parse_expectations! macro.
    expr,
};


// Test cases:
test_parse_expectations! {
    [parse_verified_expression];

    smokecheck
        : &["true"]
        => Some(expr(true))
}
