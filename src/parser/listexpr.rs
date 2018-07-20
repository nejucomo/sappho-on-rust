use ast::Expr;
use combine::ParseResult;

pub fn listexpr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::{between, parser, sep_end_by, Parser};
    use parser::expr;

    between(char('['), char(']'), sep_end_by(parser(expr), char(',')))
        .map(Expr::List)
        .parse_stream(input)
}
