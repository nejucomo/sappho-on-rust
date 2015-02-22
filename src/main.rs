#![feature(plugin)]
#![plugin(peg_syntax_ext)]


/*
fn main() {
    assert!(false, "not implemented");
}
*/

mod parse {
    // The main top-level interface to the parser:
    pub fn expression(source: &str) -> ParseResult<bool> {
        ParseResult::from_result(peg::expression(source))
    }

    // We re-implement Result so that it has a fmt::String impl...  :-<
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum ParseResult<T> {
        Ok(T),
        Err(String),
    }

    impl<T> ParseResult<T> {
        fn from_result(r: Result<T, String>) -> ParseResult<T> {
            /* Cumbersome boilerplate, since we're rewrapping Result
             * for fmt::String impl.
             */
            match r {
                Ok(v) => ParseResult::Ok(v),
                Err(e) => ParseResult::Err(e),
            }
        }
    }


    peg_file! peg("sappho.rustpeg");

    mod tests {
        use super::{ParseResult, expression};

        #[test]
        fn parse_expectations() {
            let cases = vec![
                ("true", ParseResult::Ok(true)),
                ("false", ParseResult::Ok(false)),
                ];

            for (input, expectation) in cases {
                let result = expression(input);
                assert!(result == expectation,
                        "Parse expectation failure:\nInput: {:?}\nExpectation: {:?}\nResult: {:?}\n",
                        input, expectation, result);
            }
        }
    }
}
