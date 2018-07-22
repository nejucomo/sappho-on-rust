use ast::Expr;
use combine::ParseResult;
use value::Symbol;

pub enum ApplicationPostFix {
    LookupAPF(Symbol),
    FuncAPF(Expr),
}

pub fn app_postfix(input: &str) -> ParseResult<ApplicationPostFix, &str> {
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use combine::{parser, Parser};
    use parser::subexpr::{list_expr, parens_expr};
    use parser::symbol;

    parser(symbol)
        .map(LookupAPF)
        .or(parser(parens_expr).or(parser(list_expr)).map(FuncAPF))
        .parse_stream(input)
}
