mod lambda;
mod leftassoc;

use ast::{BinaryOperator, Expr, UnaryOperator};
use combine::Parser;
use value::Symbol;

pub fn expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use self::leftassoc::left_associative;
    use combine::char::char;
    use parser::terminal::space::optspace;

    left_associative(
        times_expr().skip(optspace()),
        char('+').skip(optspace()).with(times_expr()),
        |left, right| Expr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right)),
    )
}

fn times_expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use self::leftassoc::left_associative;
    use combine::char::char;
    use parser::terminal::space::optspace;

    left_associative(
        funcapp().skip(optspace()),
        char('*').skip(optspace()).with(funcapp()),
        |left, right| Expr::BinOp(BinaryOperator::Times, Box::new(left), Box::new(right)),
    )
}

fn funcapp<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use self::leftassoc::left_associative;
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use ast::Expr::{FuncApp, LookupApp};
    use parser::terminal::space::optspace;

    left_associative(
        applicand().skip(optspace()),
        optspace().with(app_postfix()),
        |x, apf| match apf {
            LookupAPF(sym) => LookupApp(Box::new(x), sym),
            FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
        },
    )
}

pub enum ApplicationPostFix {
    LookupAPF(Symbol),
    FuncAPF(Expr),
}

pub fn app_postfix<'a>() -> impl Clone + Parser<Output = ApplicationPostFix, Input = &'a str> {
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use combine::Parser;
    use parser::atom::symbol;

    symbol()
        .map(LookupAPF)
        .or(parens_expr().or(list_expr()).map(FuncAPF))
}

fn applicand<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use self::lambda::lambda_expr;

    lambda_expr()
        .or(unary_application().map(|(op, x)| Expr::UnApp(op, x)))
        .or(unary_applicand())
}

fn unary_application<'a>(
) -> impl Clone + Parser<Output = (UnaryOperator, Box<Expr>), Input = &'a str> {
    use combine::char::char;
    use parser::terminal::space::optspace;

    ((char('$').map(|_| UnaryOperator::Query)).or(char('!').map(|_| UnaryOperator::Mutate)))
        .skip(optspace())
        .and(unary_applicand())
        .map(|(op, x)| (op, Box::new(x)))
}

fn unary_applicand<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use parser::atom::{atom, identifier};

    parens_expr()
        .or(list_expr())
        .or(atom().map(Expr::Atom))
        .or(identifier().map(Expr::Deref))
}

fn list_expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use combine::char::char;
    use combine::{between, sep_end_by, Parser};
    use parser::expr::expr;
    use parser::terminal::space::{optlinespace, optspace};

    between(
        char('[').skip(optlinespace()),
        char(']'),
        sep_end_by(expr().skip(optspace()), char(',').skip(optlinespace())),
    ).map(Expr::List)
}

fn parens_expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use combine::char::char;
    use combine::{between, Parser};
    use parser::expr::expr;
    use parser::terminal::space::optlinespace;

    between(
        char('(').skip(optlinespace()),
        char(')'),
        expr().skip(optlinespace()),
    )
}

#[cfg(test)]
mod tests {
    use super::expr;

    parser_accept_reject_tests!(expr, include_dir!("src/parser/test-vectors/expr/"));

    #[test]
    fn accepts_atom_cases() {
        use ast::Expr;
        use combine::Parser;
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
                expr().and_then(|x| match x {
                    Expr::Atom(a) => Ok(a),
                    _ => Err(MyError(format!("Expected atom found {:?}", x))),
                })
            },
            include_dir!("src/parser/test-vectors/atom/"),
        );
    }
}
