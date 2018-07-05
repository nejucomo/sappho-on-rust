use combine::{ParseResult, Parser};
use value::Atom;

pub fn atom(input: &str) -> ParseResult<Atom, &str> {
    use combine::parser;
    use parser::{boolean, character, number, symbol, text};

    (parser(boolean).map(Atom::Bool))
        .or(parser(number).map(Atom::Number))
        .or(parser(character).map(Atom::Char))
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
}
