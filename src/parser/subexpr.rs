/* Reused sub-expressions; the ast has self-identical child nodes sometimes, so reuse code. */

use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

def_ge_parser!(list_expr, ListExprParser, |applier| {
    use combine::char::char;
    use combine::{between, sep_end_by};
    use parser::expr::gen_expr;
    use parser::space::{optlinespace, optspace};

    between(
        char('[').skip(optlinespace()),
        char(']'),
        sep_end_by(
            gen_expr(applier).skip(optspace()),
            char(',').skip(optlinespace()),
        ),
    ).map(GenExpr::List)
});

def_ge_parser!(parens_expr, ParensExprParser, |applier| {
    use combine::between;
    use combine::char::char;
    use parser::expr::gen_expr;
    use parser::space::optlinespace;

    between(
        char('(').skip(optlinespace()),
        char(')'),
        gen_expr(applier).skip(optlinespace()),
    )
});
