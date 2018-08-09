use ast::GenExpr;
use combine::{ParseResult, Parser};
use value::Symbol;

pub enum ApplicationPostfix<T> {
    LookupAPF(Symbol),
    FuncAPF(GenExpr<T>),
}

pub struct AppPostfix<P>(pub P);

impl<'a, P, T> Parser for AppPostfix<P>
where
    P: Parser<Input = &'a str, Output = T>,
    T: Clone,
{
    type Input = &'a str;
    type Output = ApplicationPostfix<T>;

    fn parse_stream(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        use self::ApplicationPostfix::{FuncAPF, LookupAPF};
        use combine::parser;
        use parser::subexpr::{ListExpr, ParensExpr};
        use parser::symbol;

        parser(symbol)
            .map(LookupAPF)
            .or(ParensExpr(self.0).or(ListExpr(self.0)).map(FuncAPF))
            .parse_stream(input)
    }
}
