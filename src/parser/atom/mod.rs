mod boolean;
mod identifier;
mod number;
mod text;

pub use self::boolean::boolean;
pub use self::identifier::{identifier, symbol};
pub use self::number::number;
pub use self::text::{character, text};

use combine::Parser;
use value::Atom;

pub fn atom<'a>() -> impl Clone + Parser<Output = Atom, Input = &'a str> {
    use self::boolean::boolean;
    use self::identifier::symbol;
    use self::number::number;
    use self::text::{character, text};
    use combine::try;

    (try(boolean()).map(Atom::Bool))
        .or(number().map(Atom::Number))
        .or(try(character()).map(Atom::Char))
        .or(text().map(Atom::Text))
        .or(symbol().map(Atom::Symbol))
}

#[cfg(tests)]
parser_tests_mod!(tests, atom, include_dir!("src/parser/test-vectors/atom/"));
