use ast::Expr;
use combine::ParseResult;

pub fn lambda_expr(input: &str) -> ParseResult<Expr, &str> {
    use ast::{FunctionDefinition, LambdaDefinition};
    use combine::char::char;
    use combine::{parser, Parser};
    use parser::keywords::Keyword;
    use parser::space::{linespace, space};
    use parser::{expr, identifier};

    Keyword::Lambda
        .parser()
        .with(space())
        .with(parser(identifier))
        .and(
            space()
                .with(char('→'))
                .with(linespace())
                .with(parser(expr)),
        )
        .map(|(ident, expr)| {
            Expr::Lambda(LambdaDefinition {
                func: Some(FunctionDefinition(ident, Box::new(expr))),
            })
        })
        .parse_stream(input)
}
