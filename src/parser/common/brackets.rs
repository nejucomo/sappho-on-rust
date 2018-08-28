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
    use parser::common::space::olsp;

    between(olsp(char(open)), olsp(char(close)), inner)
}
