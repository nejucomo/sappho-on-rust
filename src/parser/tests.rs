use super::super::ast::{
    Expression,
    Function,
    FuncRule,
    Literal,
    Object,
    Pattern,
    Properties,
};
use super::{
    ParseResult,
    parse_expression,
};


fn check_parse_expectation(inputs: &[&str], expectation: ParseResult) {
    for input in inputs.iter() {
        let result = parse_expression(input);
        assert!(result == expectation,
                "Parse expectation failure:\nInput: {:?}\nExpectation: {:?}\nResult: {:?}\n",
                input, expectation, result);
    }
}


macro_rules! test_parse_expectations {
    ( $( $name:ident : $inputs:expr => $expectation:expr );* ) => {
        $(
            #[test]
            fn $name () { check_parse_expectation( $inputs, $expectation ) }
        )*
    }
}


// Test cases:
test_parse_expectations! {
    literal_true
        : &["true"]
        => Ok(Expression::Literal(Literal::Bool(true)));

    literal_false
        : &["false"]
        => Ok(Expression::Literal(Literal::Bool(false)));

    dereference
        : &["x"]
        => Ok(Expression::Dereference("x".to_string()));

    empty_object
        : &["object {}",
            "object { }",
            "object {\n}"]
        => Ok(Expression::Object(Object::empty()));

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
        => Ok(
            Expression::Object(
                Object::from_func(
                    Function(
                        vec![
                            FuncRule {
                                pattern: Pattern::Bind("x".to_string()),
                                body: Expression::Dereference("x".to_string()),
                            }]))));

    properties
        : &["object { prop .t -> true; prop .f -> false; prop (x) -> x }"]
        => Ok(
            Expression::Object(
                Object::from_properties(
                    Properties::from_items(
                        vec![
                            ("t".to_string(),
                             Box::new(Expression::Literal(Literal::Bool(true)))),
                            ("f".to_string(),
                             Box::new(Expression::Literal(Literal::Bool(false))))],
                        Some(
                            ("x".to_string(),
                             Box::new(Expression::Dereference("x".to_string()))))))));

    concrete_properties
        : &["object { prop .t -> true; prop .f -> false }"]
        => Ok(
            Expression::Object(
                Object::from_properties(
                    Properties::from_items(
                        vec![
                            ("t".to_string(),
                             Box::new(Expression::Literal(Literal::Bool(true)))),
                            ("f".to_string(),
                             Box::new(Expression::Literal(Literal::Bool(false))))],
                        None))))
}
