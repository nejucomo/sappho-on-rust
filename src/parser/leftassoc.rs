use combine::{ParseResult, Parser};

pub struct LeftAssoc<P, Q, F> {
    left_parser: Box<FnMut() -> P>,
    subsequent_parser: Box<FnMut() -> Q>,
    combine: F,
}

pub fn left_associative<P, Q, F>(
    p: Box<FnMut() -> P>,
    q: Box<FnMut() -> Q>,
    f: F,
) -> LeftAssoc<P, Q, F> {
    LeftAssoc {
        left_parser: p,
        subsequent_parser: q,
        combine: f,
    }
}

impl<'a, P, PO, Q, QO, F> Parser for LeftAssoc<P, Q, F>
where
    P: Parser<Input = &'a str, Output = PO>,
    PO: Clone,
    Q: Parser<Input = &'a str, Output = QO>,
    F: Sized + Fn(PO, QO) -> PO,
{
    type Input = &'a str;
    type Output = PO;

    fn parse_stream(&mut self, input: &'a str) -> ParseResult<PO, &'a str> {
        (*self.left_parser)()
            .then(|first| {
                use combine::many;

                let c: &'a Fn(PO, QO) -> PO = &self.combine;

                // FIXME: Can we avoid creating the intermediate vec qos?
                many((*self.subsequent_parser)()).map(move |qos: Vec<QO>| {
                    // FIXME: Can we move-capture app so we don't need a clone?
                    qos.into_iter().fold(first.clone(), *c)
                })
            })
            .parse_stream(input)
    }
}
