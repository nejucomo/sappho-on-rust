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

#[cfg(tests)]
parser_tests_mod!(tests, atom, include_dir!("src/parser/test-vectors/atom/"));
