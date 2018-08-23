use ast::{Expr, ProcUnOp};
use combine::{ParseResult, Parser};
use parser::expr::parsesto::ParsesTo;
use std::marker::PhantomData;

pub fn expr<'a, OP>() -> ExprParser<'a, OP> {
    ExprParser {
        _marker_life: PhantomData,
        _marker_op: PhantomData,
    }
}

pub fn proc_expr<'a>() -> ExprParser<'a, ProcUnOp> {
    expr()
}

#[derive(Clone)]
pub struct ExprParser<'a, OP> {
    _marker_life: PhantomData<&'a ()>,
    _marker_op: PhantomData<OP>,
}

impl<'a, OP> Parser for ExprParser<'a, OP>
where
    OP: ParsesTo<'a>,
{
    type Input = &'a str;
    type Output = Expr<OP>;

    fn parse_stream(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        use parser::expr::compound::top_expr;

        top_expr().parse_stream(input)
    }
}
