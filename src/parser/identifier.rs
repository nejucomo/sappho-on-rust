use ast::Identifier;
use combine::ParseResult;
use value::Symbol;

pub fn identifier(input: &str) -> ParseResult<Identifier, &str> {
    use combine::char::{alpha_num, char, letter};
    use combine::combinator::many;
    use combine::{parser, Parser};
    use parser::keywords::KEYWORDS;

    let head = letter().or(char('_'));
    let tail = alpha_num().or(char('_'));

    head.and(many(tail))
        .map(|t: (char, String)| t.0.to_string() + &t.1)
        .then(|id| {
            parser(move |input| {
                use combine::primitives::{Consumed, Error, ParseError, StreamOnce};

                let _: &str = input; // Require &str as input type.

                if !KEYWORDS.contains(&id.as_str()) {
                    Ok((Identifier(id.clone()), Consumed::Empty(input)))
                } else {
                    let position = input.position();
                    let err = ParseError::new(
                        position,
                        Error::Message(
                            format!(
                                "expected identifer, \
                                 found keyword {:?}",
                                id
                            ).into(),
                        ),
                    );
                    Err(Consumed::Empty(err))
                }
            })
        })
        .parse_stream(input)
}

pub fn symbol(input: &str) -> ParseResult<Symbol, &str> {
    use combine::char::char;
    use combine::{parser, Parser};
    use value::Symbol;

    char('.')
        .with(parser(identifier).map(|id| Symbol(id.0)))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    mod identifier {
        use combine::parser;
        use parser::identifier;

        parser_accept_reject_tests!(
            || parser(identifier),
            include_dir!("src/parser/test-vectors/identifier/")
        );
    }

    mod symbol {
        use combine::parser;
        use parser::symbol;

        parser_accept_reject_tests!(
            || parser(symbol),
            include_dir!("src/parser/test-vectors/symbol/")
        );
    }
}
