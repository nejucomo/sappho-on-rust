/* Reused sub-expressions; the ast has self-identical child nodes sometimes, so reuse code. */

use ast::Expr;
use combine::ParseResult;

pub fn list_expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::{between, parser, sep_end_by, Parser};
    use parser::expr;

    between(char('['), char(']'), sep_end_by(parser(expr), char(',')))
        .map(Expr::List)
        .parse_stream(input)
}

pub fn parens_expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::{between, parser, Parser};
    use parser::expr;

    between(char('('), char(')'), parser(expr)).parse_stream(input)
}
