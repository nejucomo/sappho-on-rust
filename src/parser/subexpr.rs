/* Reused sub-expressions; the ast has self-identical child nodes sometimes, so reuse code. */

use ast::{GenExpr, SteppingStoneProcExpr};
use combine::ParseResult;

pub fn list_expr(input: &str) -> ParseResult<SteppingStoneProcExpr, &str> {
    use ast::{ProcExpr, SteppingStoneProcExpr};
    use combine::char::char;
    use combine::{between, parser, sep_end_by, Parser};
    use parser::space::{optlinespace, optspace};
    use parser::stepping_stone_proc_expr;

    between(
        char('[').skip(optlinespace()),
        char(']'),
        sep_end_by(
            parser(stepping_stone_proc_expr).skip(optspace()),
            char(',').skip(optlinespace()),
        ),
    ).map(GenExpr::List)
        .map(ProcExpr::GenExpr)
        .map(SteppingStoneProcExpr)
        .parse_stream(input)
}

pub fn parens_expr(input: &str) -> ParseResult<SteppingStoneProcExpr, &str> {
    use combine::char::char;
    use combine::{between, parser, Parser};
    use parser::space::optlinespace;
    use parser::stepping_stone_proc_expr;

    between(
        char('(').skip(optlinespace()),
        char(')'),
        parser(stepping_stone_proc_expr).skip(optlinespace()),
    ).parse_stream(input)
}
