use ast::Expr;
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::parser;
    use parser::{boolean, number};
    use value::Atom;

    (parser(boolean).map(|b| Atom::Bool(b)))
        .or(parser(number).map(|n| Atom::Number(n)))
        .map(|a| Expr::Atom(a))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::expr;

    test_cases_debugrepr_parser!(expr, [(test_expr_false, "false"), (test_expr_true, "true")]);
}
