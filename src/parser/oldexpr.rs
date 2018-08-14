use ast::{BinaryOperator, FuncExpr, GenExpr, SteppingStoneProcExpr};
use combine::{ParseResult, Parser};

fn sswrap(ge: GenExpr<SteppingStoneProcExpr>) -> SteppingStoneProcExpr {
    use ast::ProcExpr;

    SteppingStoneProcExpr(ProcExpr::GenExpr(ge))
}

pub fn gen_expr(input: &str) -> ParseResult<FuncExpr, &str> {
    use ast::GenExpr;
    use combine::char::char;
    use combine::parser;
    use parser::leftassoc::left_associative;
    use parser::space::optspace;

    left_associative(
        parser(times_expr).skip(optspace()),
        char('+').skip(optspace()).with(parser(times_expr)),
        |left, right| {
            sswrap(GenExpr::BinOp(
                BinaryOperator::Plus,
                Box::new(left),
                Box::new(right),
            ))
        },
    ).parse_stream(input)
}

fn times_expr(input: &str) -> ParseResult<SteppingStoneProcExpr, &str> {
    use ast::GenExpr;
    use combine::char::char;
    use combine::parser;
    use parser::leftassoc::left_associative;
    use parser::space::optspace;

    left_associative(
        parser(funcapp).skip(optspace()),
        char('*').skip(optspace()).with(parser(funcapp)),
        |left, right| {
            sswrap(GenExpr::BinOp(
                BinaryOperator::Times,
                Box::new(left),
                Box::new(right),
            ))
        },
    ).parse_stream(input)
}

fn funcapp(input: &str) -> ParseResult<SteppingStoneProcExpr, &str> {
    use super::postapp::app_postfix;
    use super::postapp::ApplicationPostFix::{FuncAPF, LookupAPF};
    use ast::GenExpr::{FuncApp, LookupApp};
    use combine::parser;
    use parser::leftassoc::left_associative;
    use parser::space::optspace;

    left_associative(
        parser(applicand).skip(optspace()),
        optspace().with(parser(app_postfix)),
        |x, apf| {
            sswrap(match apf {
                LookupAPF(sym) => LookupApp(Box::new(x), sym),
                FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
            })
        },
    ).parse_stream(input)
}

fn applicand(input: &str) -> ParseResult<SteppingStoneProcExpr, &str> {
    use combine::parser;
    use parser::lambda::lambda_expr;

    parser(lambda_expr)
        .or(parser(unary_application))
        .or(parser(unary_applicand))
        .parse_stream(input)
}

fn unary_application(input: &str) -> ParseResult<SteppingStoneProcExpr, &str> {
    use ast::ProcExpr;
    use combine::char::char;
    use combine::parser;
    use parser::space::optspace;

    char('!')
        .skip(optspace())
        .with(parser(unary_applicand))
        .map(|x| ProcExpr::Mutate(Box::new(x)))
        .map(SteppingStoneProcExpr)
        .or(char('$')
            .skip(optspace())
            .with(parser(unary_applicand))
            .map(|x| ProcExpr::Query(Box::new(x)))
            .map(SteppingStoneProcExpr))
        .parse_stream(input)
}

fn unary_applicand(input: &str) -> ParseResult<SteppingStoneProcExpr, &str> {
    use ast::GenExpr;
    use combine::parser;
    use parser::subexpr::{list_expr, parens_expr};
    use parser::{atom, identifier};

    parser(parens_expr)
        .or(parser(list_expr))
        .or(parser(atom).map(|x| sswrap(GenExpr::Atom(x))))
        .or(parser(identifier).map(|x| sswrap(GenExpr::Deref(x))))
        .parse_stream(input)
}
