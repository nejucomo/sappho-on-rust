use combine::ParseResult;


pub fn identifier(input: &str) -> ParseResult<String, &str>
{
    use combine::{Parser, ParserExt, parser};
    use combine::char::{alpha_num, char, letter};
    use combine::combinator::many;
    use parser::keywords::KEYWORDS;

    let head = letter().or(char('_'));
    let tail = alpha_num().or(char('_'));

    (head, many(tail))
        .map(|t: (char, String)| t.0.to_string() + &t.1)
        .then(|id| parser(move |input| {
            use combine::primitives::{Consumed, Error, ParseError, StreamOnce};

            let _: &str = input; // Require &str as input type.

            if !KEYWORDS.contains(&id.as_str()) {
                Ok((id.clone(), Consumed::Empty(input)))
            } else {
                let position = input.position();
                let err = ParseError::new(
                    position,
                    Error::Message(
                        format!(
                            "expected identifer, found keyword {:?}",
                            id)
                        .into()));
                Err((Consumed::Empty(err)))
            }
        }))
        .parse_state(input)
}


#[cfg(test)]
mod tests {
    use super::identifier;

    #[test]
    fn test_identifier() {
        use combine::{Parser, ParserExt, eof, parser};

        macro_rules! include_cases {
            ($p:expr) => {
                {
                    let src = include_str!($p);
                    assert_eq!('\n', src.chars().rev().next().unwrap());
                    src[0..src.len()-1].split("\n")
                }
            }
        }

        let parser_only = |f| (parser(f), eof()).map(|t| t.0);

        for s in include_cases!("test-vectors/identifier.accept") {
            assert_eq!(
                parser_only(identifier).parse(s),
                Ok((s.to_string(), "")));
        }

        for s in include_cases!("test-vectors/identifier.reject") {
            assert!(
                parser_only(identifier).parse(s).is_err(),
                "invalidly parsed {:?} as identifier",
                s);
        }
    }
}
