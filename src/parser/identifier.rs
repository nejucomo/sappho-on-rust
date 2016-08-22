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

        let parser_only = |f| (parser(f), eof()).map(|t| t.0);

        let pos_cases = vec![
            "x",
            "foo",
            "foo42",
            "foo_bar",
            "_blah",
            "x__y",
            ];

        for s in pos_cases {
            assert_eq!(
                parser_only(identifier).parse(s),
                Ok((s.to_string(), "")));
        }

        let neg_cases = vec![
            "",
            "4",
            "42x",
            "x y",
            ];

        for s in neg_cases {
            assert!(
                parser_only(identifier).parse(s).is_err(),
                "invalidly parsed {:?} as identifier",
                s);
        }
    }
}
