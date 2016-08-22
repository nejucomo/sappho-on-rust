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
    use combine::{ParseResult, Parser, Stream};
    use combine::combinator::{Eof, FnParser, Map};
    use super::identifier;

    #[test]
    fn test_identifier() {
        use combine::{Parser, parser};

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

    fn parser_only<I, A, B, F, G>(f: F) -> Map<(FnParser<I, F>, Eof<I>), G>
        where I: Stream,
              F: FnMut(I) -> ParseResult<A, I>,
              G: FnMut(A) -> B
    {
        use combine::{ParserExt, eof, parser};

        (parser(f), eof()).map(|t| t.0)
    }
}
