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

    test_case_simple_parser!(
        identifier, test_identifier,
        |s: &str| s.to_string());

    test_case_simple_parser!(
        symbol, test_symbol,
        |s: &str| s[1..].to_string());
}
