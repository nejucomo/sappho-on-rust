/* Reused sub-expressions; the ast has self-identical child nodes sometimes, so reuse code. */

use ast::Expr;
use combine::ParseResult;

pub fn list_expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::{between, parser, sep_end_by, Parser};
    use parser::expr;
    use parser::space::{optlinespace, optspace};

    between(
        char('[').skip(optlinespace()),
        char(']'),
        sep_end_by(
            parser(expr).skip(optspace()),
            char(',').skip(optlinespace()),
        ),
    ).map(Expr::List)
        .parse_stream(input)
}

pub fn parens_expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::{between, parser, Parser};
    use parser::expr;
    use parser::space::optlinespace;

    between(
        char('(').skip(optlinespace()),
        char(')'),
        parser(expr).skip(optlinespace()),
    ).parse_stream(input)
}
