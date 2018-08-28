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
    use parser::common::space::osp;

    left_associative(
        osp(times_expr()),
        osp(char('+')).with(times_expr()),
        |left, right| Expr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right)),
    )
}

fn times_expr<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::common::space::osp;

    left_associative(
        osp(funcapp()),
        osp(char('*')).with(funcapp()),
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
    use parser::common::space::osp;

    left_associative(osp(applicand()), osp(app_postfix()), |x, apf| match apf {
        LookupAPF(sym) => LookupApp(Box::new(x), sym),
        FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
    })
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
    use parser::common::space::osp;

    osp(OP::parser())
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
    use combine::{sep_end_by, Parser};
    use parser::common::brackets::bracketed;
    use parser::common::space::{olsp, osp};
    use parser::expr::expr;

    bracketed('[', ']', sep_end_by(osp(expr()), olsp(char(',')))).map(Expr::List)
}

fn parens_expr<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use parser::common::brackets::bracketed;
    use parser::expr::expr;

    bracketed('(', ')', expr())
}
