use ast::Expr;
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::{between, parser, sep_end_by};
    use parser::{atom, identifier};

    (parser(atom).map(Expr::Atom))
        .or(parser(identifier).map(Expr::Deref))
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
            (test_expr_list_zero, "list_zero"),
            (
                test_expr_list_zero_trailing_comma,
                "list_zero_trailing_comma"
            ),
            (test_expr_deref_x, "deref_x")
        ]
    );

    #[test]
    fn expr_reject() {
        use combine::{eof, parser, Parser};

        for input in include_cases!("test-vectors/expr/reject") {
            let res = parser(expr).skip(eof()).parse(input);
            assert!(res.is_err(), "Incorrectly parsed as expr: {:?}", input);
        }
    }

}
