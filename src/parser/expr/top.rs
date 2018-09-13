use ast::{Expr, Identifier, ProcUnOp};
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
pub fn func_expr<'a, I, S>(bindings: I) -> ExprParser<'a, FuncUnOp>
where
    I: IntoIterator<Item = S>,
    Identifier: From<S>,
{
    expr(ScopeCheck::with_implicit_bindings(bindings))
}

#[cfg(test)]
pub fn query_expr<'a, I, S>(bindings: I) -> ExprParser<'a, QueryUnOp>
where
    I: IntoIterator<Item = S>,
    Identifier: From<S>,
{
    expr(ScopeCheck::with_implicit_bindings(bindings))
}

pub fn proc_expr<'a, I, S>(bindings: I) -> ExprParser<'a, ProcUnOp>
where
    I: IntoIterator<Item = S>,
    Identifier: From<S>,
{
    expr(ScopeCheck::with_implicit_bindings(bindings))
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
