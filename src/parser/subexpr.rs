/* Reused sub-expressions; the ast has self-identical child nodes sometimes, so reuse code. */

use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

def_ge_parser!(list_expr, ListExprParser, |f| {
    use combine::char::char;
    use combine::parser;
    use combine::{between, sep_end_by, Parser};
    use parser::space::{optlinespace, optspace};

    between(
        char('[').skip(optlinespace()),
        char(']'),
        sep_end_by(parser(f).skip(optspace()), char(',').skip(optlinespace())),
    ).map(GenExpr::List)
});

def_ge_parser!(parens_expr, ParensExprParser, |f| {
    use combine::char::char;
    use combine::{between, Parser};
    use parser::genexpr::gen_expr;
    use parser::space::optlinespace;

    between(
        char('(').skip(optlinespace()),
        char(')'),
        gen_expr(f).skip(optlinespace()),
    )
});
