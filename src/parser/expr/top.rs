use ast::{Expr, ProcUnOp};
use combine::{ParseResult, Parser};
use parser::expr::parsesto::ParsesTo;
use parser::expr::scopecheck::ScopeCheck;
use std::marker::PhantomData;

#[cfg(test)]
use ast::{FuncUnOp, QueryUnOp};

pub fn expr<'a, OP>(sc: ScopeCheck) -> ExprParser<'a, OP> {
    ExprParser {
        sc: sc,
        _marker_life: PhantomData,
        _marker_op: PhantomData,
    }
}

#[cfg(test)]
pub fn func_expr<'a>() -> ExprParser<'a, FuncUnOp> {
    expr(ScopeCheck::new())
}

#[cfg(test)]
pub fn query_expr<'a>() -> ExprParser<'a, QueryUnOp> {
    expr(ScopeCheck::new())
}

pub fn proc_expr<'a>() -> ExprParser<'a, ProcUnOp> {
    expr(ScopeCheck::new())
}

/* An explicit ExprParser is necessary, rather than an `impl Parser`
 * return type to prevent a cycle (thus stack overflow) in type
 * checking.
 */
#[derive(Clone)]
pub struct ExprParser<'a, OP> {
    sc: ScopeCheck,
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

        top_expr(self.sc.clone()).parse_stream(input)
    }
}
