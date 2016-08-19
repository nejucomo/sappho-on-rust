use combine::{Stream, ParseResult};


pub fn identifier<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    use combine::{Parser, ParserExt};
    use combine::char::{alpha_num, char, letter};
    use combine::combinator::many;

    let tail = alpha_num(); //.or(char('_'));

    many(tail)
        .parse_state(input)
}


#[cfg(test)]
mod tests {
    use super::identifier;

    #[test]
    fn test_identifier() {
        use combine::{Parser, parser};

        let cases = vec![
            "x",
            "foo",
            "foo42",
            "foo_bar",
            ];

        for s in cases {
            assert_eq!(
                parser(identifier).parse(s),
                Ok((s.to_string(), "")));
        }
    }
}
