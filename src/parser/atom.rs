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

    test_cases_debugrepr_parser!(
        atom,
        [
            (test_atom_false, "false"),
            (test_atom_true, "true"),
            (test_atom_zero, "zero"),
            (test_atom_char_x, "char_x"),
            (test_atom_text_foo, "text_foo"),
            (test_atom_symbol_sym, "symbol_sym")
        ]
    );
}
