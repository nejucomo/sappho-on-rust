use ast::{Expr, FunctionDefinition, LambdaDefinition, QueryDefinition};
use combine::ParseResult;

pub fn lambda_expr(input: &str) -> ParseResult<Expr, &str> {
    use combine::{parser, Parser};

    parser(kw_lambda_expr)
        .or(parser(querydef).map(LambdaDefinition::from))
        .map(Expr::Lambda)
        .parse_stream(input)
}

fn kw_lambda_expr(input: &str) -> ParseResult<LambdaDefinition, &str> {
    use combine::{parser, Parser};
    use parser::keywords::Keyword;
    use parser::space::space;

    Keyword::Lambda
        .parser()
        .with(space())
        .with(
            parser(funcdef)
                .map(LambdaDefinition::from)
                .or(parser(squigglydef)),
        )
        .parse_stream(input)
}

fn squigglydef(input: &str) -> ParseResult<LambdaDefinition, &str> {
    use combine::char::char;
    use combine::{between, parser, Parser};
    use parser::space::optlinespace;

    between(
        char('{').skip(optlinespace()),
        optlinespace().with(char('}')),
        parser(querydef).map(LambdaDefinition::from),
    ).parse_stream(input)
}

fn funcdef(input: &str) -> ParseResult<FunctionDefinition, &str> {
    use ast::FunctionDefinition;
    use combine::char::char;
    use combine::{parser, Parser};
    use parser::atom::identifier;
    use parser::expr::expr;
    use parser::space::{linespace, space};

    parser(identifier)
        .and(
            linespace()
                .with(char('→'))
                .with(space())
                .with(parser(expr)),
        )
        .map(|(ident, expr)| FunctionDefinition(ident, Box::new(expr)))
        .parse_stream(input)
}

fn querydef(input: &str) -> ParseResult<QueryDefinition, &str> {
    use ast::QueryDefinition;
    use combine::{parser, Parser};
    use parser::expr::expr;
    use parser::keywords::Keyword;
    use parser::space::space;

    Keyword::Query
        .parser()
        .with(space())
        .with(parser(expr))
        .map(|x| QueryDefinition(Box::new(x)))
        .parse_stream(input)
}
