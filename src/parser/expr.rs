use ast::{BinaryOperator, GenExpr};
use combine::{ParseResult, Parser};
use parser::unop::FuncApplier;

pub fn expr<'a>() -> GenExprParser<FuncApplier<'a>> {
    use parser::unop::FuncApplier;

    GenExprParser(FuncApplier::new())
}

def_ge_parser!(GenExprParser, |applier| {
    use combine::char::char;
    use parser::space::optspace;

    left_associative!(
        TimeExpr(applier).skip(optspace()),
        char('+').skip(optspace()).with(TimeExpr(applier)),
        |left, right| GenExpr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right))
    )
});

def_ge_parser!(TimeExpr, |applier| {
    use combine::char::char;
    use parser::space::optspace;

    left_associative!(
        FuncApp(applier).skip(optspace()),
        char('*').skip(optspace()).with(FuncApp(applier)),
        |left, right| GenExpr::BinOp(BinaryOperator::Times, Box::new(left), Box::new(right))
    )
});

def_ge_parser!(FuncApp, |applier| {
    use super::postapp::AppPostfix;
    use super::postapp::ApplicationPostfix::{FuncAPF, LookupAPF};
    use ast::GenExpr::{FuncApp, LookupApp};
    use parser::space::optspace;

    left_associative!(
        Applicand(applier).skip(optspace()),
        optspace().with(AppPostfix(applier)),
        |x, apf| match apf {
            LookupAPF(sym) => LookupApp(Box::new(x), sym),
            FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
        }
    )
});

def_ge_parser!(Applicand, |applier| {
    use combine::parser;
    use parser::lambda::lambda_expr;

    parser(lambda_expr)
        .or(UnaryApplication(applier))
        .or(UnaryApplicand(applier))
});

def_ge_parser!(UnaryApplication, |applier| {
    use parser::space::optspace;

    applier
        .skip(optspace())
        .and(UnaryApplicand(applier))
        .map(|(op, x)| GenExpr::UnApp(op, Box::new(x)))
});

def_ge_parser!(UnaryApplicand, |applier| {
    use combine::parser;
    use parser::subexpr::{ListExpr, ParensExpr};
    use parser::{atom, identifier};

    ParensExpr(applier)
        .or(ListExpr(applier))
        .or(parser(atom).map(GenExpr::Atom))
        .or(parser(identifier).map(GenExpr::Deref))
});

#[cfg(test)]
mod tests {
    use super::expr;

    parser_accept_reject_tests!(expr, include_dir!("src/parser/test-vectors/expr/"));

    #[test]
    fn accepts_atom_cases() {
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
                use ast::GenExpr;

                expr().and_then(|x| match x {
                    GenExpr::Atom(a) => Ok(a),
                    _ => Err(MyError(format!("Expected atom found {:?}", x))),
                })
            },
            include_dir!("src/parser/test-vectors/atom/"),
        );
    }
}
