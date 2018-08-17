use combine::ParseResult;

pub fn boolean(input: &str) -> ParseResult<bool, &str> {
    use combine::char::string;
    use combine::{value, Parser};

    string("true")
        .with(value(true))
        .or(string("false").with(value(false)))
        .parse_stream(input)
}

#[cfg(tests)]
parser_tests_mod!(
    tests,
    boolean,
    include_dir!("src/parser/test-vectors/boolean/")
);
