use collections::string::String;

use super::ast;


// The main top-level interface to the parser:
pub fn parse_expression(source: &str) -> ParseResult {
    peg::expression(source)
}

type ParseResult = Result<ast::Expression, String>;


// Private implementation innards below:
peg_file! peg("sappho.rustpeg");


mod tests {
    use super::super::ast::{Expression, Literal};
    use super::{ParseResult, parse_expression};


    fn check_parse_expectation(input: &str, expectation: ParseResult) {
        let result = parse_expression(input);
        assert!(result == expectation,
                "Parse expectation failure:\nInput: {:?}\nExpectation: {:?}\nResult: {:?}\n",
                input, expectation, result);
    }


    macro_rules! test_parse_expectation {
        ( $name:ident : $input:expr => $expectation:expr ) => {
            #[test]
            fn $name () { check_parse_expectation( $input, $expectation ) }
        }
    }


    // Test cases:
    test_parse_expectation! {
        literal_true : "true" => Ok(Expression::Literal(Literal::Bool(true)))
    }
    test_parse_expectation! {
        literal_false : "false" => Ok(Expression::Literal(Literal::Bool(false)))
    }
    test_parse_expectation! {
        empty_object : "object {}" => Ok(Expression::Object)
    }
}
