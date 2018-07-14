use combine::{ParseResult, Parser};
use value::Atom;

pub fn atom(input: &str) -> ParseResult<Atom, &str> {
    use combine::{parser, try};
    use parser::{boolean, character, number, symbol, text};

    (try(parser(boolean)).map(Atom::Bool))
        .or(parser(number).map(Atom::Number))
        .or(try(parser(character)).map(Atom::Char))
        .or(parser(text).map(Atom::Text))
        .or(parser(symbol).map(Atom::Symbol))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::atom;

    #[test]
    fn accepts() {
        use combine::parser;
        use parser::testutils::run_parser_repr_tests;

        run_parser_repr_tests(
            || parser(atom),
            include_dir!("src/parser/test-vectors/atom/"),
        );
    }

    #[test]
    fn rejects() {
        use combine::parser;
        use parser::testutils::run_parser_reject_tests;

        run_parser_reject_tests(|| parser(atom), include_str!("test-vectors/atom/reject"));
    }
}
