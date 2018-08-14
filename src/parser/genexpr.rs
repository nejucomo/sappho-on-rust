use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

def_ge_parser!(gen_expr, GenExprParser, |f| {
    use ast::{BinaryOperator, GenExpr};
    use combine::char::char;
    use combine::Parser;
    use parser::leftassoc::left_associative;
    use parser::space::optspace;
    use parser::timesexpr::times_expr;

    left_associative(
        times_expr(f).skip(optspace()),
        char('+').skip(optspace()).with(times_expr(f)),
        |left, right| GenExpr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right)),
    )
});
