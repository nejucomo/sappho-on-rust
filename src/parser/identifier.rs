use combine::ParseResult;


pub fn identifier(input: &str) -> ParseResult<String, &str>
{
    use combine::{Parser, ParserExt, parser};
    use combine::char::{alpha_num, char, letter};
    use combine::combinator::many;
    use parser::keywords::KEYWORDS;

    let head = letter().or(char('_'));
    let tail = alpha_num().or(char('_'));

    head.and(many(tail))
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


pub fn symbol(input: &str) -> ParseResult<String, &str>
{
    use combine::{Parser, ParserExt, parser};
    use combine::char::char;

    char('.').with(parser(identifier)).parse_state(input)
}


#[cfg(test)]
mod tests {
    use super::{identifier, symbol};

    macro_rules! include_cases {
        ($p:expr) => {
            {
                let src = include_str!($p);
                assert_eq!('\n', src.chars().rev().next().unwrap());
                src[0..src.len()-1].split("\n")
            }
        }
    }

    macro_rules! include_parser_test_vector {
        ($name:expr, accept) => {
            include_cases!(concat!("test-vectors/", stringify!($name), ".accept"));
        };

        ($name:expr, reject) => {
            include_cases!(concat!("test-vectors/", stringify!($name), ".reject"));
        }
    }

    macro_rules! simple_parser_test_case {
        ($test_name:ident, $name:ident, $make_result:expr) => {
            #[test]
            fn $test_name() {
                use combine::{Parser, ParserExt, eof, parser};

                for s in include_parser_test_vector!($name, accept) {
                    assert_eq!(
                        parser($name).skip(eof()).parse(s),
                        Ok(($make_result(s), "")));
                }

                for s in include_parser_test_vector!($name, reject) {
                    assert!(
                        parser($name).skip(eof()).parse(s).is_err(),
                        "invalidly parsed {:?} as {}",
                        s,
                        stringify!($name));
                }
            }
        }
    }

    simple_parser_test_case!(test_identifier, identifier, |s: &str| s.to_string());
    simple_parser_test_case!(test_symbol, symbol, |s: &str| s[1..].to_string());
}
