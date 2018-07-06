use combine::{ParseResult, Parser, Stream};
use std::marker::PhantomData;

pub fn character(input: &str) -> ParseResult<char, &str> {
    use combine::char::char;
    use combine::{between, Parser};

    char('c')
        .with(between(char('\''), char('\''), char_lit('\'')).or(between(
            char('"'),
            char('"'),
            char_lit('"'),
        )))
        .parse_stream(input)
}

pub fn text(input: &str) -> ParseResult<String, &str> {
    use combine::char::char;
    use combine::{between, many, Parser};

    between(char('"'), char('"'), many(char_lit('"')))
        .or(between(char('\''), char('\''), many(char_lit('\''))))
        .parse_stream(input)
}

fn char_lit<I>(delim: char) -> CharLit<I>
where
    I: Stream<Item = char>,
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
where
    I: Stream<Item = char>,
{
    type Input = I;
    type Output = char;

    fn parse_stream(&mut self, input: Self::Input) -> ParseResult<char, Self::Input> {
        use combine::primitives::{Consumed, Error, Info, ParseError};

        let mut next = input.clone();
        match next.uncons() {
            Ok(c) => match c {
                _ if c == self.delim => Err(Consumed::Empty(ParseError::new(
                    input.position(),
                    Error::Unexpected(Info::from(c)),
                ))),

                '\\' => parse_escape(self.delim, next),

                _ => Ok((c, Consumed::Consumed(next))),
            },
            Err(e) => Err(Consumed::Empty(ParseError::new(input.position(), e))),
        }
    }
}

fn parse_escape<I>(delim: char, input: I) -> ParseResult<char, I>
where
    I: Stream<Item = char>,
{
    use combine::primitives::{Consumed, Error, Info, ParseError};

    let mut next = input.clone();
    match next.uncons() {
        Ok(c) => {
            let ok = |c| Ok((c, Consumed::Consumed(next.clone())));

            match c {
                '\\' => ok('\\'),
                '0' => ok('\x00'),
                'a' => ok('\x07'),
                'b' => ok('\x08'),
                'n' => ok('\n'),
                'r' => ok('\r'),
                't' => ok('\t'),
                'x' => parse_hex_escape(2, next.clone()),
                'u' => parse_hex_escape(4, next.clone()),
                'U' => parse_hex_escape(8, next.clone()),
                c if c == delim => ok(c),
                _ => Err(Consumed::Consumed(ParseError::new(
                    input.position(),
                    Error::Unexpected(Info::from(c)),
                ))),
            }
        }
        Err(e) => Err(Consumed::Empty(ParseError::new(input.position(), e))),
    }
}

fn parse_hex_escape<I>(digits: usize, input: I) -> ParseResult<char, I>
where
    I: Stream<Item = char>,
{
    use combine::char::hex_digit;
    use combine::count_min_max;
    use std::char;
    use std::u32;

    count_min_max(digits, digits, hex_digit())
        .map(|digs: String| char::from_u32(u32::from_str_radix(&digs, 16).unwrap()).unwrap())
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::{character, text};

    test_cases_text_parser!(
        character,
        [
            (test_character_backslash, "backslash"),
            (test_character_doublequote, "doublequote"),
            (test_character_greek_lambda, "greek_lambda"),
            (test_character_lambda, "lambda"),
            (test_character_newline, "newline"),
            (test_character_singlequote, "singlequote"),
            (test_character_x, "x")
        ]
    );

    test_cases_text_parser!(
        text,
        [
            (test_text_backslash, "backslash"),
            (test_text_doublequote, "doublequote"),
            (test_text_foo_bar, "foo_bar"),
            (test_text_greek_lambda, "greek_lambda"),
            (test_text_lambda, "lambda"),
            (test_text_newline, "newline"),
            (test_text_singlequote, "singlequote"),
            (test_text_x, "x")
        ]
    );

    #[test]
    fn text_reject() {
        use combine::{eof, parser, Parser};

        for input in include_cases!("test-vectors/text/reject") {
            let res = parser(text).skip(eof()).parse(input);
            assert!(res.is_err(), "Incorrectly parsed as text: {:?}", input);
        }
    }

    #[test]
    fn character_reject() {
        use combine::{eof, parser, Parser};

        for input in include_cases!("test-vectors/character/reject") {
            let res = parser(character).skip(eof()).parse(input);
            assert!(res.is_err(), "Incorrectly parsed as character: {:?}", input);
        }
    }
}
