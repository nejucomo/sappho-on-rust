use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;
use value::Symbol;

pub enum ApplicationPostfix<T> {
    LookupAPF(Symbol),
    FuncAPF(GenExpr<T>),
}

pub fn app_postfix<'a, F, T>(f: &'static F) -> AppPostfixParser<'a, F> {
    AppPostfixParser {
        f: f,
        _phantom: PhantomData,
    }
}

pub struct AppPostfixParser<'a, F: 'static> {
    f: &'static F,
    _phantom: PhantomData<&'a str>,
}

impl<'a, F, T> Parser for AppPostfixParser<'a, F>
where
    F: 'static + Fn(&str) -> ParseResult<T, &str>,
{
    type Input = &'a str;
    type Output = ApplicationPostfix<T>;

    fn parse_stream(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        use self::ApplicationPostfix::{FuncAPF, LookupAPF};
        use combine::parser;
        use parser::subexpr::{list_expr, parens_expr};
        use parser::symbol;

        parser(symbol)
            .map(LookupAPF)
            .or(parens_expr(self.f).or(list_expr(self.0)).map(FuncAPF))
            .parse_stream(input)
    }
}
