use ast::{Expr, UnaryApplication, UnaryOperator};
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use ast::LookupApplication;
    use combine::{many, parser};
    use parser::symbol;
    use value::Symbol;

    parser(applicand)
        .then(|app| {
            // FIXME: Can we make syms an iterator to avoid excessive allocation/copy?
            many(parser(symbol)).map(move |syms: Vec<Symbol>| {
                use std::clone::Clone;

                // FIXME: Can we move-capture app so we don't need a clone?
                syms.into_iter().fold(app.clone(), |x, sym| {
                    Expr::LookupApp(LookupApplication(Box::new(x), sym))
                })
            })
        })
        .parse_stream(input)
}

fn applicand(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::{between, parser, sep_end_by};
    use parser::{atom, identifier};

    (between(char('['), char(']'), sep_end_by(parser(expr), char(','))).map(Expr::List))
        .or(parser(unary_application).map(Expr::UnApp))
        .or(parser(atom).map(Expr::Atom))
        .or(parser(identifier).map(Expr::Deref))
        .parse_stream(input)
}

fn unary_application(input: &str) -> ParseResult<UnaryApplication, &str> {
    use combine::char::char;
    use combine::parser;

    ((char('$').map(|_| UnaryOperator::Query)).or(char('!').map(|_| UnaryOperator::Mutate)))
        .and(parser(expr))
        .map(|(op, x)| UnaryApplication(op, Box::new(x)))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::expr;

    #[test]
    fn accepts() {
        use combine::parser;
        use parser::testutils::run_parser_repr_tests;

        run_parser_repr_tests(
            || parser(expr),
            include_dir!("src/parser/test-vectors/expr/"),
        );
    }

    #[test]
    fn rejects() {
        use combine::parser;
        use parser::testutils::run_parser_reject_tests;

        run_parser_reject_tests(|| parser(expr), include_str!("test-vectors/expr/reject"));
    }

    #[test]
    fn accepts_atom_cases() {
        use ast::Expr;
        use combine::{parser, Parser};
        use parser::testutils::run_parser_repr_tests;
        use std::error::Error;
        use std::fmt;

        #[derive(Debug)]
        struct MyError(String);

        impl fmt::Display for MyError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl Error for MyError {
            fn description(&self) -> &str {
                &self.0
            }
        }

        run_parser_repr_tests(
            || {
                parser(expr).and_then(|x| match x {
                    Expr::Atom(a) => Ok(a),
                    _ => Err(MyError(format!("Expected atom found {:?}", x))),
                })
            },
            include_dir!("src/parser/test-vectors/atom/"),
        );
    }
}
