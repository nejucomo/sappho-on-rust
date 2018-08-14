use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

def_ge_parser!(applicand, ApplicandParser, |f| {
    use parser::lambda::lambda_expr;
    use parser::unaryapplicand::unary_applicand;

    lambda_expr(f).or(unary_applicand(f))
});
