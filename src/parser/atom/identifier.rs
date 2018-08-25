use ast::Identifier;
use combine::Parser;
use value::Symbol;

pub fn identifier<'a>() -> impl Clone + Parser<Output = Identifier, Input = &'a str> {
    use combine::char::{alpha_num, char, letter};
    use combine::combinator::many;
    use combine::{parser, Parser};
    use parser::common::keywords::KEYWORDS;

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
}

pub fn symbol<'a>() -> impl Clone + Parser<Output = Symbol, Input = &'a str> {
    use combine::char::char;
    use combine::Parser;
    use value::Symbol;

    char('.').with(identifier().map(|id| Symbol(id.0)))
}

#[cfg(test)]
mod tests {
    mod identifier {
        use parser::atom::identifier::identifier;

        parser_accept_reject_tests!(
            identifier,
            include_dir!("src/parser/test-vectors/identifier/")
        );
    }

    mod symbol {
        use parser::atom::identifier::symbol;

        parser_accept_reject_tests!(symbol, include_dir!("src/parser/test-vectors/symbol/"));
    }
}
