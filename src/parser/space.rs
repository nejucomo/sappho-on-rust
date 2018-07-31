use combine::combinator::{Many, Many1, Or, Token};

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

pub fn optspace<'a>() -> Many<Vec<char>, Token<&'a str>> {
    use combine::char::char;
    use combine::many;

    many::<Vec<_>, _>(char(' '))
}

pub fn optlinespace<'a>() -> Many<Vec<char>, Or<Token<&'a str>, Token<&'a str>>> {
    use combine::char::char;
    use combine::{many, Parser};

    many::<Vec<_>, _>(char(' ').or(char('\n')))
}
