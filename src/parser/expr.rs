use ast::{Expr, UnaryOperator};
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use super::postapp::ApplicationPostFix::{FuncAPF, LookupAPF};
    use super::postapp::{app_postfix, ApplicationPostFix};
    use ast::Expr::{FuncApp, LookupApp};
    use combine::{many, parser};

    parser(applicand)
        .then(|app| {
            // FIXME: Can we make apfs an iterator to avoid excessive allocation/copy?
            many(parser(app_postfix)).map(move |apfs: Vec<ApplicationPostFix>| {
                use std::clone::Clone;

                // FIXME: Can we move-capture app so we don't need a clone?
                apfs.into_iter().fold(app.clone(), |x, apf| match apf {
                    LookupAPF(sym) => LookupApp(Box::new(x), sym),
                    FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
                })
            })
        })
        .parse_stream(input)
}

fn applicand(input: &str) -> ParseResult<Expr, &str> {
    use combine::parser;
    use parser::listexpr::listexpr;
    use parser::{atom, identifier};

    parser(listexpr)
        .or(parser(unary_application).map(|(op, x)| Expr::UnApp(op, x)))
        .or(parser(atom).map(Expr::Atom))
        .or(parser(identifier).map(Expr::Deref))
        .parse_stream(input)
}

fn unary_application(input: &str) -> ParseResult<(UnaryOperator, Box<Expr>), &str> {
    use combine::char::char;
    use combine::parser;

    ((char('$').map(|_| UnaryOperator::Query)).or(char('!').map(|_| UnaryOperator::Mutate)))
        .and(parser(expr))
        .map(|(op, x)| (op, Box::new(x)))
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
