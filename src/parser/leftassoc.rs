use combine::{ParseResult, Parser};

pub fn left_associative<L, S, C>(
    left: L,
    subsequent: S,
    combine: C,
) -> LeftAssociativeParser<L, S, C> {
    LeftAssociativeParser {
        left: left,
        subsequent: subsequent,
        combine: combine,
    }
}

pub struct LeftAssociativeParser<L, S, C> {
    left: L,
    subsequent: S,
    combine: C,
}

impl<'a, LP, LO, SP, SO, CF, CO> Parser for LeftAssociativeParser<LP, SP, CF>
where
    LP: Parser<Input = &'a str, Output = LO>,
    SP: Parser<Input = &'a str, Output = SO>,
    CF: Fn(LO, SO) -> CO,
{
    type Input = &'a str;
    type Output = CO;

    fn parse_stream(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        use combine::many;
        // FIXME: Can we avoid creating the intermediate vec qos?
        self.left.then(|lval| {
            many(self.subsequent).map(move |subs: Vec<_>| {
                // FIXME: Can we move-capture app so we don't need a clone?
                subs.into_iter().fold(lval.clone(), self.combine)
            })
        })
    }
}
