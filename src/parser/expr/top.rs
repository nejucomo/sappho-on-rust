use ast::Expr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

pub fn expr<'a>() -> ExprParser<'a> {
    ExprParser {
        _marker: PhantomData,
    }
}

#[derive(Clone)]
pub struct ExprParser<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> Parser for ExprParser<'a> {
    type Input = &'a str;
    type Output = Expr;

    fn parse_stream(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        use parser::expr::compound::plus_expr;

        plus_expr().parse_stream(input)
    }
}
