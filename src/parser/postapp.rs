use ast::Expr;
use combine::ParseResult;
use value::Symbol;

pub enum ApplicationPostFix {
    LookupAPF(Symbol),
    FuncAPF(Expr),
}

pub fn app_postfix(input: &str) -> ParseResult<ApplicationPostFix, &str> {
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use combine::char::char;
    use combine::{between, parser, Parser};
    use parser::listexpr::listexpr;
    use parser::{expr, symbol};

    parser(symbol)
        .map(LookupAPF)
        .or(between(char('('), char(')'), parser(expr))
            .or(parser(listexpr))
            .map(FuncAPF))
        .parse_stream(input)
}
