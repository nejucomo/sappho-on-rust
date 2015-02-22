#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(collections)]


extern crate collections;


/*
fn main() {
    assert!(false, "not implemented");
}
*/

mod parse {
    use collections::string::String;

    // The main top-level interface to the parser:
    pub fn expression(source: &str) -> ParseResult<bool> {
        peg::expression(source)
    }

    type ParseResult<T> = Result<T, String>;


    peg_file! peg("sappho.rustpeg");

    mod tests {
        use super::{ParseResult, expression};

        fn check_parse_expectation(input: &str, expectation: ParseResult<bool>) {
            let result = expression(input);
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

        test_parse_expectation! { literal_true  : "true"  => Ok(true)  }
        test_parse_expectation! { literal_false : "false" => Ok(false) }
    }
}
