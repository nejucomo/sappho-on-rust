use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

def_ge_parser!(unary_applicand, UnaryApplicandParser, |f| {
    use ast::GenExpr;
    use combine::parser;
    use parser::subexpr::{list_expr, parens_expr};
    use parser::{atom, identifier};

    parens_expr(f)
        .or(list_expr(f))
        .or(parser(atom).map(|x| GenExpr::Atom(x)))
        .or(parser(identifier).map(|x| GenExpr::Deref(x)))
});
