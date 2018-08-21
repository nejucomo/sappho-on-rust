use ast::{Expr, FunctionDefinition, LambdaDefinition, QueryDefinition};
use combine::Parser;

pub fn lambda_expr<'a>() -> impl Clone + Parser<Output = Expr, Input = &'a str> {
    use combine::Parser;

    kw_lambda_expr()
        .or(querydef().map(LambdaDefinition::from))
        .map(Expr::Lambda)
}

fn kw_lambda_expr<'a>() -> impl Clone + Parser<Output = LambdaDefinition, Input = &'a str> {
    use combine::Parser;
    use parser::terminal::keywords::Keyword;
    use parser::terminal::space::space;

    Keyword::Lambda
        .parser()
        .with(space())
        .with(funcdef().map(LambdaDefinition::from).or(squigglydef()))
}

fn squigglydef<'a>() -> impl Clone + Parser<Output = LambdaDefinition, Input = &'a str> {
    use combine::char::char;
    use combine::{between, Parser};
    use parser::terminal::space::optlinespace;

    between(
        char('{').skip(optlinespace()),
        optlinespace().with(char('}')),
        querydef().map(LambdaDefinition::from),
    )
}

fn funcdef<'a>() -> impl Clone + Parser<Output = FunctionDefinition, Input = &'a str> {
    use ast::FunctionDefinition;
    use combine::char::char;
    use combine::Parser;
    use parser::atom::identifier;
    use parser::expr::expr;
    use parser::terminal::space::{linespace, space};

    identifier()
        .and(linespace().with(char('→')).with(space()).with(expr()))
        .map(|(ident, expr)| FunctionDefinition(ident, Box::new(expr)))
}

fn querydef<'a>() -> impl Clone + Parser<Output = QueryDefinition, Input = &'a str> {
    use ast::QueryDefinition;
    use combine::Parser;
    use parser::expr::expr;
    use parser::terminal::keywords::Keyword;
    use parser::terminal::space::space;

    Keyword::Query
        .parser()
        .with(space())
        .with(expr())
        .map(|x| QueryDefinition(Box::new(x)))
}
