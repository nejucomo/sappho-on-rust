use ast::Expr;
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::{between, parser, sep_end_by};
    use parser::atom;

    (parser(atom).map(Expr::Atom))
        .or(between(char('['), char(']'), sep_end_by(parser(expr), char(','))).map(Expr::List))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::expr;

    test_cases_debugrepr_parser!(
        expr,
        [
            (test_expr_false, "false"),
            (test_expr_list_empty, "list_empty"),
            (test_expr_list_zero, "list_zero")
        ]
    );
}
