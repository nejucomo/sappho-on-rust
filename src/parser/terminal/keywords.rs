macro_rules! define_keyword {
    ( $( ($name:ident, $testname:ident, $text:expr) ),* ) => {

        #[derive(Clone, Debug)]
        pub enum Keyword {
            $( $name ),*
        }

        impl Keyword {
            pub fn as_str(&self) -> &'static str {
                match *self {
                    $( Keyword::$name => &($text) ),*
                }
            }

            /* Silly method to ensure code coverage in main() */
            pub fn all() -> Vec<Keyword> {
                let mut v = Vec::new();
                $(
                    v.push(Keyword::$name);
                )*
                v
            }
        }

        pub const KEYWORDS: [&'static str; 9] = [
            $( $text ),*
        ];

        #[cfg(test)]
        mod tests {
            $(
                mod $testname {
                    #[test]
                    fn accepts() {
                        use parser::testutils::run_parser_repr_tests;
                        use parser::terminal::keywords::Keyword;

                        let casename = &stringify!($name).to_lowercase();

                        run_parser_repr_tests(
                            || Keyword::$name.parser(),
                            include_dir!("src/parser/test-vectors/keywords").get_dir(casename).expect(
                                &format!("src/parser/test-vectors/keywords/{}", casename),
                            ),
                        );
                    }

                    #[test]
                    fn rejects() {
                        use parser::testutils::run_parser_reject_tests;
                        use parser::terminal::keywords::Keyword;

                        let casename = stringify!($name).to_lowercase();

                        run_parser_reject_tests(
                            || Keyword::$name.parser(),
                            include_dir!("src/parser/test-vectors/keywords/")
                                .get_dir(&casename)
                                .expect(&format!("{:?} missing reject file", casename)),
                        );
                    }
                }
            )*
        }
    }
}

define_keyword!(
    (False, kw_false, "false"),
    (True, kw_true, "true"),
    (Lambda, kw_lambda, "𝜆"),
    (Proc, kw_proc, "proc"),
    (Query, kw_query, "query"),
    (Let, kw_let, "let"),
    (In, kw_in, "in"),
    (From, kw_from, "from"),
    (Bind, kw_bind, "bind")
);

use combine::{ParseResult, Parser, Stream};
use std::fmt;
use std::marker::PhantomData;

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone)]
pub struct KeywordParser<I> {
    keyword: Keyword,
    _marker: PhantomData<I>,
}

impl Keyword {
    pub fn parser<I>(self) -> KeywordParser<I> {
        KeywordParser {
            keyword: self,
            _marker: PhantomData,
        }
    }
}

impl<I> Parser for KeywordParser<I>
where
    I: Stream<Item = char>,
{
    type Input = I;
    type Output = ();

    fn parse_stream(&mut self, input: Self::Input) -> ParseResult<(), Self::Input> {
        use combine::char::string;
        use combine::value;

        string(self.keyword.as_str())
            .with(value(()))
            .parse_stream(input)
    }
}
