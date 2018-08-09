use ast::{FuncUnOp, ProcUnOp, QueryUnOp};
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

macro_rules! def_unop_parser {
    ($parsename:ident, $out:ty, $parse:block) => {
        pub struct $parsename<'a>(PhantomData<&'a str>);

        impl<'a> $parsename<'a> {
            pub fn new() -> $parsename<'a> {
                $parsename(PhantomData)
            }
        }

        impl<'a> Parser for $parsename<'a> {
            type Input = &'a str;
            type Output = $out;

            fn parse_stream(
                &mut self,
                input: Self::Input,
            ) -> ParseResult<Self::Output, Self::Input> {
                ($parse).parse_stream(input)
            }
        }
    };
}

def_unop_parser!(FuncApplier, FuncUnOp, {
    use combine::char::char;
    use combine::{unexpected, value};

    char('!')
        .or(char('$'))
        .then(|c| {
            unexpected(format!(
                "{:?} operator not allowed in function expressions.",
                c
            ))
        })
        .with(value(FuncUnOp::NotValid))
});

def_unop_parser!(QueryApplier, QueryUnOp, {
    use combine::char::char;
    use combine::{unexpected, value};

    char('!')
        .with(unexpected("'!' operator not allowed in query expressions."))
        .with(value(QueryUnOp::NotValid))
        .or(char('$').map(|_| QueryUnOp::Query))
});

def_unop_parser!(ProcApplier, ProcUnOp, {
    use combine::char::char;

    (char('$').map(|_| ProcUnOp::Query)).or(char('!').map(|_| ProcUnOp::Mutate))
});
