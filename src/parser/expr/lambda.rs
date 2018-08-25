use ast::{Expr, FunctionDefinition, LambdaDefinition, ProcDefinition, QueryDefinition};
use combine::Parser;

pub fn lambda_expr<'a, OP>() -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str> {
    use combine::Parser;

    kw_lambda_expr()
        .or(querydef().map(LambdaDefinition::from))
        .or(procdef().map(LambdaDefinition::from))
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
    use combine::{optional, value, Parser};
    use parser::expr::brackets::bracketed;
    use parser::terminal::space::{linespace, optspace};
    use std::fmt::Debug;

    fn merge_options<T: Debug>(left: Option<T>, right: Option<T>) -> Option<T> {
        match (left, right) {
            (Some(l), Some(r)) => panic!(
                "invalid LambdaDefinition merge state in parser: {:?} {:?}",
                l, r
            ),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }

    fn merge_ldef<T>((left, right): (T, Option<LambdaDefinition>)) -> LambdaDefinition
    where
        LambdaDefinition: From<T>,
    {
        let ldef = LambdaDefinition::from(left);

        right.map_or(ldef.clone(), |rdef| LambdaDefinition {
            func: merge_options(ldef.func, rdef.func),
            query: merge_options(ldef.query, rdef.query),
            proc: merge_options(ldef.proc, rdef.proc),
        })
    }

    fn func_or_nothing<'a>() -> impl Clone + Parser<Output = LambdaDefinition, Input = &'a str> {
        funcdef()
            .map(LambdaDefinition::from)
            .or(value(LambdaDefinition {
                func: None,
                query: None,
                proc: None,
            }))
    }

    fn query_or_rest<'a>() -> impl Clone + Parser<Output = LambdaDefinition, Input = &'a str> {
        querydef()
            .skip(optspace())
            .and(optional(
                char(';').skip(linespace()).with(func_or_nothing()),
            ))
            .map(merge_ldef)
            .or(func_or_nothing())
    }

    fn proc_or_rest<'a>() -> impl Clone + Parser<Output = LambdaDefinition, Input = &'a str> {
        procdef()
            .skip(optspace())
            .and(optional(char(';').skip(linespace()).with(query_or_rest())))
            .map(merge_ldef)
            .or(query_or_rest())
    }

    bracketed('{', '}', proc_or_rest())
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
    use parser::expr;
    use parser::terminal::keywords::Keyword;
    use parser::terminal::space::space;

    Keyword::Query
        .parser()
        .with(space())
        .with(expr())
        .map(|x| QueryDefinition(Box::new(x)))
}

fn procdef<'a>() -> impl Clone + Parser<Output = ProcDefinition, Input = &'a str> {
    use ast::ProcDefinition;
    use combine::{optional, value, Parser};
    use parser::expr::brackets::bracketed;
    use parser::proc_expr;
    use parser::terminal::keywords::Keyword;
    use parser::terminal::space::space;

    Keyword::Proc.parser().skip(space()).with(bracketed(
        '{',
        '}',
        Keyword::Return
            .parser()
            .skip(space())
            .with(optional(proc_expr()))
            .map(|ox| ox.map(Box::new))
            .map(ProcDefinition::Return)
            .or(value(ProcDefinition::Return(None))),
    ))
}
