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
    use parser::common::keywords::Keyword;
    use parser::common::space::sp;

    sp(Keyword::Lambda.parser()).with(funcdef().map(LambdaDefinition::from).or(squigglydef()))
}

fn squigglydef<'a>() -> impl Clone + Parser<Output = LambdaDefinition, Input = &'a str> {
    use combine::char::char;
    use combine::{optional, value, Parser};
    use parser::common::brackets::bracketed;
    use parser::common::space::{lsp, osp};
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
        osp(querydef())
            .and(optional(lsp(char(';')).with(func_or_nothing())))
            .map(merge_ldef)
            .or(func_or_nothing())
    }

    fn proc_or_rest<'a>() -> impl Clone + Parser<Output = LambdaDefinition, Input = &'a str> {
        osp(procdef())
            .and(optional(lsp(char(';')).with(query_or_rest())))
            .map(merge_ldef)
            .or(query_or_rest())
    }

    bracketed('{', '}', proc_or_rest())
}

fn funcdef<'a>() -> impl Clone + Parser<Output = FunctionDefinition, Input = &'a str> {
    use ast::FunctionDefinition;
    use combine::char::char;
    use combine::Parser;
    use parser::common::space::{lsp, sp};
    use parser::expr::expr;
    use parser::expr::pattern::pattern;

    lsp(pattern())
        .and(sp(char('→')).with(expr()))
        .map(|(ident, expr)| FunctionDefinition(ident, Box::new(expr)))
}

fn querydef<'a>() -> impl Clone + Parser<Output = QueryDefinition, Input = &'a str> {
    use ast::QueryDefinition;
    use combine::Parser;
    use parser::common::keywords::Keyword;
    use parser::common::space::sp;
    use parser::expr;

    sp(Keyword::Query.parser())
        .with(expr())
        .map(|x| QueryDefinition(Box::new(x)))
}

fn procdef<'a>() -> impl Clone + Parser<Output = ProcDefinition, Input = &'a str> {
    use ast::ProcDefinition;
    use combine::{optional, value, Parser};
    use parser::common::brackets::bracketed;
    use parser::common::keywords::Keyword;
    use parser::common::space::sp;
    use parser::proc_expr;

    sp(Keyword::Proc.parser()).with(bracketed(
        '{',
        '}',
        sp(Keyword::Return.parser())
            .with(optional(proc_expr()))
            .map(|ox| ox.map(Box::new))
            .map(ProcDefinition::Return)
            .or(value(ProcDefinition::Return(None))),
    ))
}
