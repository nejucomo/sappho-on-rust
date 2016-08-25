use combine::{Stream, ParseResult};


pub fn identifier<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    use combine::{Parser, ParserExt};
    use combine::char::{alpha_num, char, letter};
    use combine::combinator::many;

    let head = letter().or(char('_'));
    let tail = alpha_num().or(char('_'));

    (head, many(tail))
        .map(|t: (char, String)| t.0.to_string() + &t.1)
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
