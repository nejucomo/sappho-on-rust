use combine::ParseResult;

pub fn boolean(input: &str) -> ParseResult<bool, &str> {
    use combine::char::string;
    use combine::{value, Parser};

    string("true")
        .with(value(true))
        .or(string("false").with(value(false)))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::boolean;

    #[test]
    fn accepts() {
        use combine::parser;
        use parser::testutils::run_parser_repr_tests;

        run_parser_repr_tests(
            || parser(boolean),
            include_dir!("src/parser/test-vectors/boolean/"),
        );
    }

    #[test]
    fn rejects() {
        use combine::parser;
        use parser::testutils::run_parser_reject_tests;

        run_parser_reject_tests(
            || parser(boolean),
            include_str!("test-vectors/boolean/reject"),
        );
    }
}
