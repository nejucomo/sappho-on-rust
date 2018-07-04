#[cfg(test)]
use combine::{ParseResult, Parser};

macro_rules! define_keyword_parsers {
    ( $( ($testname:ident, $name:ident, $text:expr) ),* ) => {

        pub const KEYWORDS: [&'static str; 9] = [
            $( $text ),*
            ];

        $(
            #[cfg(test)]
            fn $name(input: &str) -> ParseResult<(), &str> {
                use combine::char::string;
                use combine::value;

                string($text)
                    .with(value(()))
                    .parse_stream(input)
            }

            #[cfg(test)]
            test_case_simple_parser!(
                $name,
                $testname,
                |_| ());
        )*
    }
}

define_keyword_parsers!(
    (test_kw_false, kw_false, "false"),
    (test_kw_true, kw_true, "true"),
    (test_kw_lambda, kw_lambda, "𝜆"),
    (test_kw_proc, kw_proc, "proc"),
    (test_kw_query, kw_query, "query"),
    (test_kw_let, kw_let, "let"),
    (test_kw_in, kw_in, "in"),
    (test_kw_from, kw_from, "from"),
    (test_kw_bind, kw_bind, "bind")
);
