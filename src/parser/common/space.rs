use combine::combinator::{Many, Many1, Or, Token};
use combine::Parser;

pub fn sp<'a, P>(p: P) -> impl Clone + Parser<Output = P::Output, Input = &'a str>
where
    P: Clone + Parser<Input = &'a str>,
{
    p.skip(space())
}

pub fn lsp<'a, P>(p: P) -> impl Clone + Parser<Output = P::Output, Input = &'a str>
where
    P: Clone + Parser<Input = &'a str>,
{
    p.skip(linespace())
}

pub fn osp<'a, P>(p: P) -> impl Clone + Parser<Output = P::Output, Input = &'a str>
where
    P: Clone + Parser<Input = &'a str>,
{
    p.skip(optspace())
}

pub fn olsp<'a, P>(p: P) -> impl Clone + Parser<Output = P::Output, Input = &'a str>
where
    P: Clone + Parser<Input = &'a str>,
{
    p.skip(optlinespace())
}

fn space<'a>() -> Many1<Vec<char>, Token<&'a str>> {
    use combine::char::char;
    use combine::many1;

    many1::<Vec<_>, _>(char(' '))
}

fn linespace<'a>() -> Many1<Vec<char>, Or<Token<&'a str>, Token<&'a str>>> {
    use combine::char::char;
    use combine::{many1, Parser};

    many1::<Vec<_>, _>(char(' ').or(char('\n')))
}

fn optspace<'a>() -> Many<Vec<char>, Token<&'a str>> {
    use combine::char::char;
    use combine::many;

    many::<Vec<_>, _>(char(' '))
}

fn optlinespace<'a>() -> Many<Vec<char>, Or<Token<&'a str>, Token<&'a str>>> {
    use combine::char::char;
    use combine::{many, Parser};

    many::<Vec<_>, _>(char(' ').or(char('\n')))
}
