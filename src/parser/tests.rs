use super::super::ast::{Expression, Literal, Object};
use super::{ParseResult, parse_expression};


fn check_parse_expectation(input: &str, expectation: ParseResult) {
    let result = parse_expression(input);
    assert!(result == expectation,
            "Parse expectation failure:\nInput: {:?}\nExpectation: {:?}\nResult: {:?}\n",
            input, expectation, result);
}


macro_rules! test_parse_expectations {
    ( $( $name:ident : $input:expr => $expectation:expr );* ) => {
        $(
            #[test]
            fn $name () { check_parse_expectation( $input, $expectation ) }
        )*
    }
}


// Test cases:
test_parse_expectations! {
    literal_true
        : "true"
        => Ok(Expression::Literal(Literal::Bool(true)));

    literal_false
        : "false"
        => Ok(Expression::Literal(Literal::Bool(false)));

    dereference
        : "x"
        => Ok(Expression::Dereference("x".to_string()));

    empty_object
        : "object {}"
        => Ok(Expression::Object(Object::empty()))
}
