mod compound;
mod lambda;
mod leftassoc;
mod parsesto;
mod pattern;
mod scopecheck;
mod top;

pub use self::top::{expr, proc_expr};

#[cfg(test)]
pub use self::top::{func_expr, query_expr};

#[cfg(test)]
mod tests {
    macro_rules! def_expr_kind_test_mod {
        ($pname:ident, $path:expr) => {
            mod $pname {
                mod common {
                    use parser::$pname;

                    parser_accept_reject_tests!(
                        $pname,
                        include_dir!("src/parser/test-vectors/expr/common")
                    );
                }

                mod specialized {
                    use parser::$pname;

                    parser_accept_reject_tests!($pname, include_dir!($path));
                }

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
                            use parser::$pname;

                            $pname().and_then(|x| match x {
                                Expr::Atom(a) => Ok(a),
                                _ => Err(MyError(format!("Expected atom found {:?}", x))),
                            })
                        },
                        include_dir!("src/parser/test-vectors/atom/"),
                    );
                }
            }
        };
    }

    def_expr_kind_test_mod!(func_expr, "src/parser/test-vectors/expr/func");
    def_expr_kind_test_mod!(query_expr, "src/parser/test-vectors/expr/query");
    def_expr_kind_test_mod!(proc_expr, "src/parser/test-vectors/expr/proc");
}
