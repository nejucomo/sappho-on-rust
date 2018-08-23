use ast::{FuncUnOp, ProcUnOp, QueryUnOp};
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

pub trait ParsesTo<'a>: Sized + Clone {
    type PTParser: Clone + Parser<Output = Self, Input = &'a str>;

    fn parser() -> Self::PTParser;
}

macro_rules! parsesto_impl {
    ($out:ty, $parsestruct:ident, $parse:expr) => {
        #[derive(Clone)]
        pub struct $parsestruct<'a>(PhantomData<&'a ()>);

        impl<'a> Parser for $parsestruct<'a> {
            type Output = $out;
            type Input = &'a str;

            fn parse_stream(
                &mut self,
                input: Self::Input,
            ) -> ParseResult<Self::Output, Self::Input> {
                $parse.parse_stream(input)
            }
        }

        impl<'a> ParsesTo<'a> for $out {
            type PTParser = $parsestruct<'a>;

            fn parser() -> Self::PTParser {
                $parsestruct(PhantomData)
            }
        }
    };
}

parsesto_impl!(FuncUnOp, FuncUnOpParser, {
    use combine::char::char;
    use combine::value;

    char('-').with(value(FuncUnOp::Invert))
});

parsesto_impl!(QueryUnOp, QueryUnOpParser, {
    use combine::char::char;
    use combine::value;

    char('$')
        .with(value(QueryUnOp::Resolve))
        .or(FuncUnOp::parser().map(QueryUnOp::FUO))
});

parsesto_impl!(ProcUnOp, ProcUnOpParser, {
    use combine::char::char;
    use combine::value;

    char('!')
        .with(value(ProcUnOp::Mutate))
        .or(QueryUnOp::parser().map(ProcUnOp::QUO))
});
