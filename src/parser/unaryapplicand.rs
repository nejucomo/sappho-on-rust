use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

def_top_parser!(unary_applicand, UnaryApplicandParser, |f, wrap| {
    use ast::GenExpr;
    use combine::parser;
    use parser::subexpr::{list_expr, parens_expr};
    use parser::{atom, identifier};

    list_expr(f)
        .or(parser(atom).map(|x| GenExpr::Atom(x)))
        .or(parser(identifier).map(|x| GenExpr::Deref(x)))
        .map(wrap)
        .or(parens_expr(f))
});
