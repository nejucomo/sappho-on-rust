use ast::{CompoundExpr, FunctionDefinition, LambdaDefinition, QueryDefinition};
use combine::Parser;
use parser::expr::tepi::TopExprParseInfo;

pub fn lambda_expr<'a, T>() -> impl Clone + Parser<Output = T, Input = &'a str>
where
    T: TopExprParseInfo<'a>,
{
    use combine::Parser;

    kw_lambda_expr()
        .or(querydef().map(LambdaDefinition::from))
        .map(CompoundExpr::Lambda)
        .map(T::wrap_compound)
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
    use parser::expr::top::func_expr;
    use parser::terminal::space::{linespace, space};

    identifier()
        .and(
            linespace()
                .with(char('→'))
                .with(space())
                .with(func_expr()),
        )
        .map(|(ident, expr)| FunctionDefinition(ident, Box::new(expr)))
}

fn querydef<'a>() -> impl Clone + Parser<Output = QueryDefinition, Input = &'a str> {
    use ast::QueryDefinition;
    use combine::Parser;
    use parser::expr::top::query_expr;
    use parser::terminal::keywords::Keyword;
    use parser::terminal::space::space;

    Keyword::Query
        .parser()
        .with(space())
        .with(query_expr())
        .map(|x| QueryDefinition(Box::new(x)))
}
