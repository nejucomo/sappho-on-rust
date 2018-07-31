use combine::combinator::{Many1, Or, Token};

pub fn space<'a>() -> Many1<Vec<char>, Token<&'a str>> {
    use combine::char::char;
    use combine::many1;

    many1::<Vec<_>, _>(char(' '))
}

pub fn linespace<'a>() -> Many1<Vec<char>, Or<Token<&'a str>, Token<&'a str>>> {
    use combine::char::char;
    use combine::{many1, Parser};

    many1::<Vec<_>, _>(char(' ').or(char('\n')))
}
