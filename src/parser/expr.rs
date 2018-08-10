use ast::{BinaryOperator, FuncExpr, GenExpr, ProcExpr, QueryExpr};
use combine::{ParseResult, Parser};
use parser::unaryapp::unary_application;
use std::marker::PhantomData;

pub fn func_expr(input: &str) -> ParseResult<FuncExpr, &str> {
    gen_expr(&func_expr).map(FuncExpr).parse_input(input)
}

pub fn query_expr(input: &str) -> ParseResult<QueryExpr, &str> {
    use combine::char::char;
    use combine::parser;
    use parser::space::optspace;

    gen_expr(&query_expr)
        .map(QueryExpr::GExpr)
        .or(char('$')
            .with(optspace())
            .with(parser(query_expr))
            .map(|x| QueryExpr::Query(Box::new(x))))
        .parse_input(input)
}

pub fn proc_expr(input: &str) -> ParseResult<ProcExpr, &str> {
    use combine::char::char;
    use combine::parser;
    use parser::space::optspace;

    gen_expr(&proc_expr)
        .map(ProcExpr::Gexpr)
        .or(char('$')
            .with(optspace())
            .with(parser(proc_expr))
            .map(|x| ProcExpr::Query(Box::new(x))))
        .or(char('!')
            .with(optspace())
            .with(parser(proc_expr))
            .map(|x| ProcExpr::Mutate(Box::new(x))))
        .parse_input(input)
}

def_ge_parser!(gen_expr, GenExprParser, |applier| {
    use combine::char::char;
    use parser::leftassoc::left_associative;
    use parser::space::optspace;

    left_associative(
        times_expr(applier).skip(optspace()),
        char('+').skip(optspace()).with(times_expr(applier)),
        |left, right| GenExpr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right)),
    )
});

def_ge_parser!(times_expr, TimesExprParser, |applier| {
    use combine::char::char;
    use parser::leftassoc::left_associative;
    use parser::space::optspace;

    left_associative(
        func_app(applier).skip(optspace()),
        char('*').skip(optspace()).with(func_app(applier)),
        |left, right| GenExpr::BinOp(BinaryOperator::Times, Box::new(left), Box::new(right)),
    )
});

def_ge_parser!(func_app, FuncAppParser, |applier| {
    use super::postapp::app_postfix;
    use super::postapp::ApplicationPostfix::{FuncAPF, LookupAPF};
    use ast::GenExpr::{FuncApp, LookupApp};
    use parser::leftassoc::left_associative;
    use parser::space::optspace;

    left_associative(
        applicand(applier).skip(optspace()),
        optspace().with(app_postfix(applier)),
        |x, apf| match apf {
            LookupAPF(sym) => LookupApp(Box::new(x), sym),
            FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
        },
    )
});

def_ge_parser!(applicand, ApplicandParser, |applier| {
    use combine::parser;
    use parser::lambda::lambda_expr;

    parser(lambda_expr)
        .or(unary_application(applier))
        .or(unary_applicand(applier))
});

def_ge_parser!(unary_applicand, UnaryApplicandParser, |applier| {
    use combine::parser;
    use parser::subexpr::{list_expr, parens_expr};
    use parser::{atom, identifier};

    parens_expr(applier)
        .or(list_expr(applier))
        .or(parser(atom).map(GenExpr::Atom))
        .or(parser(identifier).map(GenExpr::Deref))
});

#[cfg(test)]
mod tests {
    use super::func_expr;
    use combine::parser;

    parser_accept_reject_tests!(
        || parser(func_expr),
        include_dir!("src/parser/test-vectors/expr/")
    );

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

                func_expr().and_then(|x| match x {
                    GenExpr::Atom(a) => Ok(a),
                    _ => Err(MyError(format!("Expected atom found {:?}", x))),
                })
            },
            include_dir!("src/parser/test-vectors/atom/"),
        );
    }
}
