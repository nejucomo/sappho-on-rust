use ast::Expr;
use combine::ParseResult;

pub fn lambda_expr(input: &str) -> ParseResult<Expr, &str> {
    use ast::FunctionDefinition;
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
            linespace()
                .with(char('→'))
                .with(space())
                .with(parser(expr)),
        )
        .map(|(ident, expr)| Expr::Lambda(FunctionDefinition(ident, Box::new(expr)).into()))
        .parse_stream(input)
}
