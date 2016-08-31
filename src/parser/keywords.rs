use combine::{ParseResult, Parser};


macro_rules! define_keyword_parsers {
    ( $( ($name:ident, $testname:ident, $text:expr) ),* ) => {

        pub const KEYWORDS: [&'static str; 7] = [
            $( $text ),*
            ];

        $(
            fn $name(input: &str) -> ParseResult<(), &str> {
                use combine::{ParserExt, string, value};

                string($text)
                    .with(value(()))
                    .parse_state(input)
            }

            test_case_simple_parser!(
                $testname,
                $name,
                |_| ());
        )*
    }
}


define_keyword_parsers!(
    (kw_lambda, test_kw_lambda, "𝜆"),
    (kw_proc,   test_kw_proc,   "proc"),
    (kw_query,  test_kw_query,  "query"),
    (kw_let,    test_kw_let,    "let"),
    (kw_in,     test_kw_in,     "in"),
    (kw_from,   test_kw_from,   "from"),
    (kw_bind,   test_kw_bind,   "bind")
);
