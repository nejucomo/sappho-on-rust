use ast::Expr;
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::parser;
    use parser::atom;

    parser(atom).map(Expr::Atom).parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::expr;

    test_cases_debugrepr_parser!(expr, [(test_expr_false, "false")]);
}
