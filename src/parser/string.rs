use std::marker::PhantomData;
use combine::{ParseResult, Parser, Stream};


pub fn character(input: &str) -> ParseResult<char, &str>
{
    use combine::{Parser, between, char};

    between(char('\''), char('\''), char_lit('\''))
        .parse_state(input)
}


pub fn string(input: &str) -> ParseResult<String, &str>
{
    use combine::{Parser, between, char, many};

    between(char('"'), char('"'), many(char_lit('"')))
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
        use combine::primitives::{Consumed, ParseError};

        let mut next = input.clone();
        match next.uncons() {
            Ok(c) => match c {
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

/*
            if c == 'u' {
                Box::new(
                    parser(
                        |input| {
                            between(char('{'), char('}'), many1(hex_digit()))
                                .and_then(|esc: String| {
                                    use std::char::from_u32;
                                    
                                    u32::from_str_radix(esc.as_str(), 16)
                                        .map(from_u32)
                                })
                                .parse_state(input)
                        }))
            } else {
                Box::new(
                    match c {
                    })
            }
        })
*/
}


#[cfg(test)]
mod tests {
    use super::{character, string};

    test_case_simple_parser!(
        test_character, character,
        |s: &str| s.chars().nth(1).unwrap());

    test_case_simple_parser!(
        test_string, string,
        |s: &str| s.to_string());
}
