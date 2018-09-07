use ast::{BinaryOperator, Expr};
use combine::Parser;
use parser::expr::parsesto::ParsesTo;
use parser::expr::scopecheck::ScopeCheck;
use value::Symbol;

pub fn top_expr<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    plus_expr(sc)
}

fn plus_expr<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::common::space::osp;

    left_associative(
        osp(times_expr(sc.clone())),
        osp(char('+')).with(times_expr(sc)),
        |left, right| Expr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right)),
    )
}

fn times_expr<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::common::space::osp;

    left_associative(
        osp(funcapp(sc.clone())),
        osp(char('*')).with(funcapp(sc)),
        |left, right| Expr::BinOp(BinaryOperator::Times, Box::new(left), Box::new(right)),
    )
}

fn funcapp<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use super::leftassoc::left_associative;
    use ast::Expr::{FuncApp, LookupApp};
    use parser::common::space::osp;

    left_associative(
        osp(applicand(sc.clone())),
        osp(app_postfix(sc)),
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

pub fn app_postfix<'a, OP>(
    sc: ScopeCheck,
) -> impl Clone + Parser<Output = ApplicationPostFix<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use combine::Parser;
    use parser::atom::symbol;

    symbol()
        .map(LookupAPF)
        .or(parens_expr(sc.clone()).or(list_expr(sc)).map(FuncAPF))
}

fn applicand<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use super::lambda::lambda_expr;

    lambda_expr(sc.clone())
        .or(unary_application(sc.clone()))
        .or(unary_applicand(sc))
}

fn unary_application<'a, OP>(
    sc: ScopeCheck,
) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use parser::common::space::osp;

    osp(OP::parser())
        .and(unary_applicand(sc))
        .map(|(op, x)| Expr::UnApp(op, Box::new(x)))
}

fn unary_applicand<'a, OP>(
    sc: ScopeCheck,
) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use parser::atom::atom;
    use parser::expr::scopecheck::deref;

    parens_expr(sc.clone())
        .or(list_expr(sc.clone()))
        .or(atom().map(Expr::Atom))
        .or(deref(sc))
}

fn list_expr<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use combine::char::char;
    use combine::{sep_end_by, Parser};
    use parser::common::brackets::bracketed;
    use parser::common::space::{olsp, osp};
    use parser::expr::expr;

    bracketed('[', ']', sep_end_by(osp(expr(sc)), olsp(char(',')))).map(Expr::List)
}

fn parens_expr<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str>
where
    OP: ParsesTo<'a>,
{
    use parser::common::brackets::bracketed;
    use parser::expr::expr;

    bracketed('(', ')', expr(sc))
}
