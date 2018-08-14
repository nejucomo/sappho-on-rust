use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

def_ge_parser!(times_expr, TimesExprParser, |f| {
    use ast::{BinaryOperator, GenExpr};
    use combine::char::char;
    use parser::funcapp::funcapp;
    use parser::leftassoc::left_associative;
    use parser::space::optspace;

    left_associative(
        funcapp(f).skip(optspace()),
        char('*').skip(optspace()).with(funcapp(f)),
        |left, right| GenExpr::BinOp(BinaryOperator::Times, Box::new(left), Box::new(right)),
    )
});
