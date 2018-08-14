use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;
use value::Symbol;

#[derive(Clone, Debug)]
pub enum ApplicationPostFix<T>
where
    T: Clone,
{
    LookupAPF(Symbol),
    FuncAPF(GenExpr<T>),
}

def_parser!(app_postfix, AppPostfixParser, ApplicationPostFix, |f| {
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use combine::{parser, Parser};
    use parser::subexpr::{list_expr, parens_expr};
    use parser::symbol;

    parser(symbol)
        .map(LookupAPF)
        .or(parens_expr(f).or(list_expr(f)).map(FuncAPF))
});
