use ast::FuncExpr;
use combine::{ParseResult, Parser};

pub fn func_expr(input: &str) -> ParseResult<FuncExpr, &str> {
    use combine::Parser;
    use parser::genexpr::gen_expr;

    gen_expr(&func_expr).map(FuncExpr);
}

#[cfg(test)]
mod tests {
    use super::func_expr;

    parser_accept_reject_tests!(func_expr, include_dir!("src/parser/test-vectors/expr/"));

    #[test]
    fn accepts_atom_cases() {
        use ast::GenExpr;
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
                use ast::{ProcExpr, SteppingStoneProcExpr};

                parser(func_expr).and_then(|x| match x {
                    SteppingStoneProcExpr(ProcExpr::GenExpr(GenExpr::Atom(a))) => Ok(a),
                    _ => Err(MyError(format!("Expected atom found {:?}", x))),
                })
            },
            include_dir!("src/parser/test-vectors/atom/"),
        );
    }
}
