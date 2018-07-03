use ast::Expr;
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::parser;
    use parser::boolean;
    use value::Atom;

    parser(boolean)
        .map(|b| Expr::Atom(Atom::Bool(b)))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::expr;

    test_cases_debugrepr_parser!(expr, [(test_expr_false, "false"), (test_expr_true, "true")]);
}
