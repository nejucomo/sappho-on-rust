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
    mod text {
        use parser::text;

        #[test]
        fn accepts() {
            use combine::parser;
            use parser::testutils::run_parser_repr_tests;

            run_parser_repr_tests(
                || parser(text),
                include_dir!("src/parser/test-vectors/text/"),
            );
        }

        #[test]
        fn rejects() {
            use combine::parser;
            use parser::testutils::run_parser_reject_tests;

            run_parser_reject_tests(|| parser(text), include_str!("test-vectors/text/reject"));
        }
    }

    mod character {
        use parser::character;

        #[test]
        fn accepts() {
            use combine::parser;
            use parser::testutils::run_parser_repr_tests;

            run_parser_repr_tests(
                || parser(character),
                include_dir!("src/parser/test-vectors/character/"),
            );
        }

        #[test]
        fn rejects() {
            use combine::parser;
            use parser::testutils::run_parser_reject_tests;

            run_parser_reject_tests(
                || parser(character),
                include_str!("test-vectors/character/reject"),
            );
        }
    }
}
