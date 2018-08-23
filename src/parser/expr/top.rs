use ast::{CompoundExpr, FuncExpr, ProcExpr, QueryExpr};
use combine::{ParseResult, Parser};
use parser::expr::tepi::TopExprParseInfo;
use std::marker::PhantomData;

macro_rules! define_top_parser {
    ($maker:ident, $toptype:ty, $wrap:expr, $parsestruct:ident, $parser:expr) => {
        pub fn $maker<'a>() -> $parsestruct<'a> {
            $parsestruct(PhantomData)
        }

        #[derive(Clone)]
        pub struct $parsestruct<'a>(PhantomData<&'a ()>);

        impl<'a> Parser for $parsestruct<'a> {
            type Input = &'a str;
            type Output = $toptype;

            fn parse_stream(
                &mut self,
                input: Self::Input,
            ) -> ParseResult<Self::Output, Self::Input> {
                $parser.parse_stream(input)
            }
        }

        impl<'a> TopExprParseInfo<'a> for $toptype {
            type TopExprParser = $parsestruct<'a>;

            fn parser() -> $parsestruct<'a> {
                $maker()
            }

            fn wrap_compound(comp: CompoundExpr<Self>) -> Self {
                ($wrap)(comp)
            }
        }
    };
}

define_top_parser!(func_expr, FuncExpr, FuncExpr, FuncExprParser, {
    use parser::expr::compound::compound_expr;

    compound_expr::<FuncExpr>()
});

define_top_parser!(
    query_expr,
    QueryExpr,
    QueryExpr::Compound,
    QueryExprParser,
    {
        use combine::char::char;
        use parser::expr::compound::compound_expr;

        char('$')
            .with(query_expr())
            .map(|x| QueryExpr::Resolve(Box::new(x)))
            .or(compound_expr())
    }
);

define_top_parser!(proc_expr, ProcExpr, ProcExpr::Compound, ProcExprParser, {
    use combine::char::char;
    use parser::expr::compound::compound_expr;

    char('!')
        .with(proc_expr())
        .map(|x| ProcExpr::Mutate(Box::new(x)))
        .or(char('$')
            .with(proc_expr())
            .map(|x| ProcExpr::Resolve(Box::new(x))))
        .or(compound_expr())
});
