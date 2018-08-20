use combine::Parser;

pub fn left_associative<'a, LP, LO, SP, SO, MF>(
    left: LP,
    subsequent: SP,
    merge: MF,
) -> impl Parser<Input = &'a str, Output = LO>
where
    LP: Parser<Input = &'a str, Output = LO>,
    SP: Clone + Parser<Input = &'a str, Output = SO>,
    MF: Clone + Fn(LO, SO) -> LO,
    LO: Clone,
{
    left.then(move |lval| {
        use combine::{many, value};

        value(lval).and(many(subsequent.clone()))
    }).map(move |(lval, subs): (_, Vec<_>)| subs.into_iter().fold(lval.clone(), merge.clone()))
}
