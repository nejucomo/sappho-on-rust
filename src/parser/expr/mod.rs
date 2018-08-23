mod compound;
mod lambda;
mod leftassoc;
mod parsesto;
mod top;

pub use self::top::{expr, proc_expr};

#[cfg(test)]
mod tests {
    use super::proc_expr;

    parser_accept_reject_tests!(proc_expr, include_dir!("src/parser/test-vectors/expr/"));

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
                proc_expr().and_then(|x| match x {
                    Expr::Atom(a) => Ok(a),
                    _ => Err(MyError(format!("Expected atom found {:?}", x))),
                })
            },
            include_dir!("src/parser/test-vectors/atom/"),
        );
    }
}
