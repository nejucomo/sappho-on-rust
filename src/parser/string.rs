use std::marker::PhantomData;
use combine::{ParseResult, Parser, Stream};


pub fn character(input: &str) -> ParseResult<char, &str>
{
    use combine::{Parser, ParserExt, between, char};

    char('c')
        .with(
            between(
                char('\''), char('\''), char_lit('\''))
                .or(between(char('"'), char('"'), char_lit('"'))))
        .parse_state(input)
}


pub fn string(input: &str) -> ParseResult<String, &str>
{
    use combine::{Parser, ParserExt, between, char, many};

    between(char('"'), char('"'), many(char_lit('"')))
        .or(between(char('\''), char('\''), many(char_lit('\''))))
        .parse_state(input)
}


fn char_lit<I>(delim: char) -> CharLit<I>
    where I: Stream<Item = char>
{
    CharLit {
        delim: delim,
        _marker: PhantomData,
    }
}


struct CharLit<I> {
    delim: char,
    _marker: PhantomData<I>,
}


impl<I> Parser for CharLit<I>
    where I: Stream<Item = char>
{
    type Input = I;
    type Output = char;

    fn parse_state(&mut self, input: Self::Input) -> ParseResult<char, Self::Input>
    {
        use combine::primitives::{Consumed, Error, Info, ParseError};

        let mut next = input.clone();
        match next.uncons() {
            Ok(c) => match c {
                _ if c == self.delim =>
                    Err(
                        Consumed::Empty(
                            ParseError::new(
                                input.position(),
                                Error::Unexpected(Info::from(c))))),

                '\\' => parse_escape(self.delim, next),

                _ => Ok((c, Consumed::Consumed(next))),
            },
            Err(e) => Err(Consumed::Empty(ParseError::new(input.position(), e))),
        }
    }
}


fn parse_escape<I>(delim: char, input: I) -> ParseResult<char, I>
    where I: Stream<Item = char>
{
    use combine::primitives::{Consumed, Error, Info, ParseError};

    let mut next = input.clone();
    match next.uncons() {
        Ok(c) => {
            let ok = |c| Ok((c, Consumed::Consumed(next)));

            match c {
                '\\' => ok('\\'),
                '0' => ok('\x00'),
                'a' => ok('\x07'),
                'b' => ok('\x08'),
                'n' => ok('\n'),
                'r' => ok('\r'),
                't' => ok('\t'),
                c if c == delim => ok(c),
                _ => Err(
                    Consumed::Consumed(
                        ParseError::new(
                            input.position(),
                            Error::Unexpected(Info::from(c))))),
            }
        },
        Err(e) => Err(Consumed::Empty(ParseError::new(input.position(), e))),
    }
}


#[cfg(test)]
mod tests {
    use super::{character, string};

    test_cases_string_parser!(
        character,
        [(test_character_backslash, "backslash"),
         (test_character_doublequote, "doublequote"),
         (test_character_greek_lambda, "greek_lambda"),
         (test_character_lambda, "lambda"),
         (test_character_newline, "newline"),
         (test_character_singlequote, "singlequote"),
         (test_character_x, "x")]);

    test_cases_string_parser!(
        string,
        [(test_string_backslash, "backslash"),
         (test_string_doublequote, "doublequote"),
         (test_string_foo_bar, "foo_bar"),
         (test_string_greek_lambda, "greek_lambda"),
         (test_string_lambda, "lambda"),
         (test_string_newline, "newline"),
         (test_string_singlequote, "singlequote"),
         (test_string_x, "x")]);
}
