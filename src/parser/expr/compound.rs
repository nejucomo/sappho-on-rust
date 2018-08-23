use ast::{BinaryOperator, Expr};
use combine::Parser;
use parser::expr::parsesto::ParsesTo;
use value::Symbol;

pub fn top_expr<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    plus_expr()
}

fn plus_expr<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::terminal::space::optspace;

    left_associative(
        times_expr().skip(optspace()),
        char('+').skip(optspace()).with(times_expr()),
        |left, right| Expr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right)),
    )
}

fn times_expr<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::terminal::space::optspace;

    left_associative(
        funcapp().skip(optspace()),
        char('*').skip(optspace()).with(funcapp()),
        |left, right| Expr::BinOp(BinaryOperator::Times, Box::new(left), Box::new(right)),
    )
}

fn funcapp<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use super::leftassoc::left_associative;
    use ast::Expr::{FuncApp, LookupApp};
    use parser::terminal::space::optspace;

    left_associative(
        applicand().skip(optspace()),
        optspace().with(app_postfix()),
        |x, apf| match apf {
            LookupAPF(sym) => LookupApp(Box::new(x), sym),
            FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
        },
    )
}

pub enum ApplicationPostFix<OP> {
    LookupAPF(Symbol),
    FuncAPF(Expr<OP>),
}

pub fn app_postfix<'a, OP>() -> impl Clone + Parser<Output = ApplicationPostFix<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use combine::Parser;
    use parser::atom::symbol;

    symbol()
        .map(LookupAPF)
        .or(parens_expr().or(list_expr()).map(FuncAPF))
}

fn applicand<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use super::lambda::lambda_expr;

    lambda_expr().or(unary_application()).or(unary_applicand())
}

fn unary_application<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use parser::terminal::space::optspace;

    OP::parser()
        .skip(optspace())
        .and(unary_applicand())
        .map(|(op, x)| Expr::UnApp(op, Box::new(x)))
}

fn unary_applicand<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use parser::atom::{atom, identifier};

    parens_expr()
        .or(list_expr())
        .or(atom().map(Expr::Atom))
        .or(identifier().map(Expr::Deref))
}

fn list_expr<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use combine::char::char;
    use combine::{between, sep_end_by, Parser};
    use parser::expr::expr;
    use parser::terminal::space::{optlinespace, optspace};

    between(
        char('[').skip(optlinespace()),
        char(']'),
        sep_end_by(expr().skip(optspace()), char(',').skip(optlinespace())),
    ).map(Expr::List)
}

fn parens_expr<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use combine::char::char;
    use combine::{between, Parser};
    use parser::expr::expr;
    use parser::terminal::space::optlinespace;

    between(
        char('(').skip(optlinespace()),
        char(')'),
        expr().skip(optlinespace()),
    )
}
