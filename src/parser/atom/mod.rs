mod boolean;
mod identifier;
mod number;
mod text;

pub use self::boolean::boolean;
pub use self::identifier::{identifier, symbol};
pub use self::number::number;
pub use self::text::{character, text};

use combine::{ParseResult, Parser};
use value::Atom;

pub fn atom(input: &str) -> ParseResult<Atom, &str> {
    use self::boolean::boolean;
    use self::identifier::symbol;
    use self::number::number;
    use self::text::{character, text};
    use combine::{parser, try};

    (try(parser(boolean)).map(Atom::Bool))
        .or(parser(number).map(Atom::Number))
        .or(try(parser(character)).map(Atom::Char))
        .or(parser(text).map(Atom::Text))
        .or(parser(symbol).map(Atom::Symbol))
        .parse_stream(input)
}

#[cfg(tests)]
parser_tests_mod!(tests, atom, include_dir!("src/parser/test-vectors/atom/"));
