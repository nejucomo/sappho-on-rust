use ast::{FunctionDefinition, GenExpr, LambdaDefinition, QueryDefinition, SteppingStoneProcExpr};
use combine::ParseResult;

// FIXME: Remove stepping stone implementation that produces SteppingStoneProcExpr.
pub fn lambda_expr(input: &str) -> ParseResult<SteppingStoneProcExpr, &str> {
    use ast::{ProcExpr, SteppingStoneProcExpr};
    use combine::{parser, Parser};

    parser(kw_lambda_expr)
        .or(parser(querydef).map(LambdaDefinition::from))
        .map(GenExpr::Lambda)
        .map(ProcExpr::GenExpr)
        .map(SteppingStoneProcExpr)
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
    use parser::space::{linespace, space};
    use parser::{identifier, stepping_stone_proc_expr};

    parser(identifier)
        .and(
            linespace()
                .with(char('→'))
                .with(space())
                .with(parser(stepping_stone_proc_expr)),
        )
        .map(|(ident, expr)| FunctionDefinition(ident, Box::new(expr)))
        .parse_stream(input)
}

fn querydef(input: &str) -> ParseResult<QueryDefinition, &str> {
    use ast::QueryDefinition;
    use combine::{parser, Parser};
    use parser::keywords::Keyword;
    use parser::space::space;
    use parser::stepping_stone_proc_expr;

    Keyword::Query
        .parser()
        .with(space())
        .with(parser(stepping_stone_proc_expr))
        .map(|x| QueryDefinition(Box::new(x)))
        .parse_stream(input)
}
