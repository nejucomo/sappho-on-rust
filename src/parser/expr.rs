use ast::Expr;
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::parser;
    use parser::{boolean, character, number, symbol, text};
    use value::Atom;

    (parser(boolean).map(Atom::Bool))
        .or(parser(number).map(Atom::Number))
        .or(parser(character).map(Atom::Char))
        .or(parser(text).map(Atom::Text))
        .or(parser(symbol).map(Atom::Symbol))
        .map(Expr::Atom)
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::expr;

    test_cases_debugrepr_parser!(expr, [(test_expr_false, "false"), (test_expr_true, "true")]);
}
