use super::super::{
    Callable,
    Expression,
    Literal,
    parse_expression,
};


#[test]
fn parse_smoketest() {
    let input = "true";

    let expectation =
        Some(
            Expression::Apps(
                Callable::Literal(
                    Literal::Bool(true)),
                vec![]));

    let result = parse_expression(input);
    assert!(result.as_ref().ok() == expectation.as_ref(),
            "Parse expectation failure:\nInput: {:?}\nExpectation: {:?}\nResult: {:?}\n",
            input, expectation, result);
}
