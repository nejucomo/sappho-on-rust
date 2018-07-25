use ast::Expr;
use combine::ParseResult;

pub fn lambda_expr(input: &str) -> ParseResult<Expr, &str> {
    use ast::{FunctionDefinition, LambdaDefinition};
    use combine::char::char;
    use combine::{many1, parser, Parser};
    use parser::keywords::Keyword;
    use parser::{expr, identifier};

    Keyword::Lambda
        .parser()
        .with(many1::<Vec<_>, _>(char(' ')))
        .with(parser(identifier))
        .and(
            many1::<Vec<_>, _>(char(' '))
                .with(char('→'))
                .with(many1::<Vec<_>, _>(char(' ').or(char('\n'))))
                .with(parser(expr)),
        )
        .map(|(ident, expr)| {
            Expr::Lambda(LambdaDefinition {
                func: Some(FunctionDefinition(ident, Box::new(expr))),
            })
        })
        .parse_stream(input)
}
