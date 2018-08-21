use ast::{BinaryOperator, Expr, UnaryOperator};
use combine::Parser;
use value::Symbol;

pub fn plus_expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::terminal::space::optspace;

    left_associative(
        times_expr().skip(optspace()),
        char('+').skip(optspace()).with(times_expr()),
        |left, right| Expr::BinOp(BinaryOperator::Plus, Box::new(left), Box::new(right)),
    )
}

fn times_expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use super::leftassoc::left_associative;
    use combine::char::char;
    use parser::terminal::space::optspace;

    left_associative(
        funcapp().skip(optspace()),
        char('*').skip(optspace()).with(funcapp()),
        |left, right| Expr::BinOp(BinaryOperator::Times, Box::new(left), Box::new(right)),
    )
}

fn funcapp<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
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

pub enum ApplicationPostFix {
    LookupAPF(Symbol),
    FuncAPF(Expr),
}

pub fn app_postfix<'a>() -> impl Clone + Parser<Output = ApplicationPostFix, Input = &'a str> {
    use self::ApplicationPostFix::{FuncAPF, LookupAPF};
    use combine::Parser;
    use parser::atom::symbol;

    symbol()
        .map(LookupAPF)
        .or(parens_expr().or(list_expr()).map(FuncAPF))
}

fn applicand<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use super::lambda::lambda_expr;

    lambda_expr()
        .or(unary_application().map(|(op, x)| Expr::UnApp(op, x)))
        .or(unary_applicand())
}

fn unary_application<'a>(
) -> impl Clone + Parser<Output = (UnaryOperator, Box<Expr>), Input = &'a str> {
    use combine::char::char;
    use parser::terminal::space::optspace;

    ((char('$').map(|_| UnaryOperator::Query)).or(char('!').map(|_| UnaryOperator::Mutate)))
        .skip(optspace())
        .and(unary_applicand())
        .map(|(op, x)| (op, Box::new(x)))
}

fn unary_applicand<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use parser::atom::{atom, identifier};

    parens_expr()
        .or(list_expr())
        .or(atom().map(Expr::Atom))
        .or(identifier().map(Expr::Deref))
}

fn list_expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
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

fn parens_expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
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
