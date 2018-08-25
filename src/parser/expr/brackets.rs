use combine::Parser;

pub fn bracketed<'a, P>(
    open: char,
    close: char,
    inner: P,
) -> impl Clone + Parser<Output = P::Output, Input = &'a str>
where
    P: Clone + Parser<Input = &'a str>,
{
    use combine::between;
    use combine::char::char;
    use parser::terminal::space::optlinespace;

    between(
        char(open).skip(optlinespace()),
        optlinespace().with(char(close)),
        inner,
    )
}
