use ast::{BinaryOperator, Expr, UnaryOperator};
use combine::{ParseResult, Parser};

pub fn expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::parser;
    use parser::space::optspace;

    left_associative!(
        parser(times_expr).skip(optspace()),
        char('+').skip(optspace()).with(parser(times_expr)),
        |left, right| Expr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right))
    ).parse_stream(input)
}

fn times_expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::char::char;
    use combine::parser;
    use parser::space::optspace;

    left_associative!(
        parser(funcapp).skip(optspace()),
        char('*').skip(optspace()).with(parser(funcapp)),
        |left, right| Expr::BinOp(BinaryOperator::Times, Box::new(left), Box::new(right))
    ).parse_stream(input)
}

fn funcapp(input: &str) -> ParseResult<Expr, &str> {
    use combine::parser;

    use super::postapp::app_postfix;
    use super::postapp::ApplicationPostFix::{FuncAPF, LookupAPF};
    use ast::Expr::{FuncApp, LookupApp};
    use parser::space::optspace;

    left_associative!(
        parser(applicand).skip(optspace()),
        optspace().with(parser(app_postfix)),
        |x, apf| match apf {
            LookupAPF(sym) => LookupApp(Box::new(x), sym),
            FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
        }
    ).parse_stream(input)
}

fn applicand(input: &str) -> ParseResult<Expr, &str> {
    use combine::parser;
    use parser::lambda::lambda_expr;
    use parser::subexpr::{list_expr, parens_expr};
    use parser::{atom, identifier};

    parser(list_expr)
        .or(parser(parens_expr))
        .or(parser(lambda_expr))
        .or(parser(unary_application).map(|(op, x)| Expr::UnApp(op, x)))
        .or(parser(atom).map(Expr::Atom))
        .or(parser(identifier).map(Expr::Deref))
        .parse_stream(input)
}

fn unary_application(input: &str) -> ParseResult<(UnaryOperator, Box<Expr>), &str> {
    use combine::char::char;
    use combine::parser;
    use parser::space::optspace;

    ((char('$').map(|_| UnaryOperator::Query)).or(char('!').map(|_| UnaryOperator::Mutate)))
        .skip(optspace())
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
