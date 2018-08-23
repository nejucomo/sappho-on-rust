mod compound;
mod lambda;
mod leftassoc;
mod tepi;
mod top;

pub use self::top::func_expr;

#[cfg(test)]
mod tests {
    use super::func_expr;

    parser_accept_reject_tests!(func_expr, include_dir!("src/parser/test-vectors/expr/"));

    #[test]
    fn accepts_atom_cases() {
        use ast::CompoundExpr;
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
                use ast::FuncExpr;

                func_expr().and_then(|x| match x {
                    FuncExpr(CompoundExpr::Atom(a)) => Ok(a),
                    _ => Err(MyError(format!("Expected atom found {:?}", x))),
                })
            },
            include_dir!("src/parser/test-vectors/atom/"),
        );
    }
}
