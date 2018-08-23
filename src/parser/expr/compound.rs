use ast::{BinaryOperator, CompoundExpr, UnaryOperator};
use combine::Parser;
use parser::expr::tepi::TopExprParseInfo;
use value::Symbol;

pub fn compound_expr<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::terminal::space::optspace;

    left_associative(
        times_expr::<T>().skip(optspace()),
        char('+').skip(optspace()).with(times_expr::<T>()),
        |left, right| {
            T::wrap_compound(CompoundExpr::BinOp(
                BinaryOperator::Plus,
                Box::new(left),
                Box::new(right),
            ))
        },
    )
}

fn times_expr<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::terminal::space::optspace;

    left_associative(
        funcapp().skip(optspace()),
        char('*').skip(optspace()).with(funcapp()),
        |left, right| {
            T::wrap_compound(CompoundExpr::BinOp(
                BinaryOperator::Times,
                Box::new(left),
                Box::new(right),
            ))
        },
    )
}

pub enum ApplicationPostFix<T> {
    LookupAPF(Symbol),
    FuncAPF(T),
}

fn funcapp<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use super::leftassoc::left_associative;
    use ast::CompoundExpr::{FuncApp, LookupApp};
    use parser::terminal::space::optspace;

    left_associative(
        applicand().skip(optspace()),
        optspace().with(app_postfix()),
        |x, apf| {
            T::wrap_compound(match apf {
                LookupAPF(sym) => LookupApp(Box::new(x), sym),
                FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
            })
        },
    )
}

pub fn app_postfix<'a, T>() -> impl Clone + Parser<Output = ApplicationPostFix<T>, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use combine::Parser;
    use parser::atom::symbol;

    symbol()
        .map(LookupAPF)
        .or(parens_expr().or(list_expr()).map(FuncAPF))
}

fn applicand<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use super::lambda::lambda_expr;

    lambda_expr().or(unary_application()).or(unary_applicand())
}

fn unary_application<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use combine::char::char;

    char('-')
        .with(T::parser())
        .map(|x| CompoundExpr::UnApp(UnaryOperator::Negate, Box::new(x)))
        .map(T::wrap_compound)
}

fn unary_applicand<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use parser::atom::{atom, identifier};

    parens_expr()
        .or(list_expr())
        .or(atom().map(CompoundExpr::Atom).map(T::wrap_compound))
        .or(identifier().map(CompoundExpr::Deref).map(T::wrap_compound))
}

fn list_expr<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use combine::char::char;
    use combine::{between, sep_end_by, Parser};
    use parser::terminal::space::{optlinespace, optspace};

    between(
        char('[').skip(optlinespace()),
        char(']'),
        sep_end_by(T::parser().skip(optspace()), char(',').skip(optlinespace())),
    ).map(CompoundExpr::List)
        .map(T::wrap_compound)
}

fn parens_expr<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use combine::char::char;
    use combine::{between, Parser};
    use parser::terminal::space::optlinespace;

    between(
        char('(').skip(optlinespace()),
        char(')'),
        T::parser().skip(optlinespace()),
    )
}
