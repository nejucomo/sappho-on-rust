use combine::{ParseResult, Parser};
use std::marker::PhantomData;

pub fn unary_application<'a, F>(f: &'static F) -> UnaryApplicationParser<'a, F> {
    UnaryApplicationParser {
        f: f,
        _phantom: PhantomData,
    }
}

pub struct UnaryApplicationParser<'a, F: 'static> {
    f: &'static F,
    _phantom: PhantomData<&'a str>,
}

impl<'a, F, T> Parser for UnaryApplicationParser<'a, F>
where
    F: 'static + Fn(&str) -> ParseResult<T, &str>,
{
    type Input = &'a str;
    type Output = T;

    fn parse_stream(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        use ast::GenExpr;
        use combine::parser;
        use parser::expr::unary_applicand;
        use parser::space::optspace;

        parser(self.f)
            .skip(optspace())
            .and(unary_applicand(parser(self.f)))
            .map(|(op, x)| GenExpr::UnApp(op, Box::new(x)))
            .parse_stream(input)
    }
}
