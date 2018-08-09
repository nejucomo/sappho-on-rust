/* Reused sub-expressions; the ast has self-identical child nodes sometimes, so reuse code. */

use ast::GenExpr;
use combine::{ParseResult, Parser};

def_ge_parser!(ListExpr, |applier| {
    use combine::char::char;
    use combine::{between, sep_end_by};
    use parser::expr::GenExprParser;
    use parser::space::{optlinespace, optspace};

    between(
        char('[').skip(optlinespace()),
        char(']'),
        sep_end_by(
            GenExprParser(applier).skip(optspace()),
            char(',').skip(optlinespace()),
        ),
    ).map(GenExpr::List)
});

def_ge_parser!(ParensExpr, |applier| {
    use combine::between;
    use combine::char::char;
    use parser::expr::GenExprParser;
    use parser::space::optlinespace;

    between(
        char('(').skip(optlinespace()),
        char(')'),
        GenExprParser(applier).skip(optlinespace()),
    )
});
