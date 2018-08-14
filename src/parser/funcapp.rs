use ast::GenExpr;
use combine::{ParseResult, Parser};
use std::marker::PhantomData;

def_ge_parser!(funcapp, FuncAppParser, |f| {
    use ast::GenExpr::{FuncApp, LookupApp};
    use combine::Parser;
    use parser::applicand::applicand;
    use parser::leftassoc::left_associative;
    use parser::postapp::app_postfix;
    use parser::postapp::ApplicationPostFix::{FuncAPF, LookupAPF};
    use parser::space::optspace;

    left_associative(
        applicand(f).skip(optspace()),
        optspace().with(app_postfix(f)),
        |x, apf| match apf {
            LookupAPF(sym) => LookupApp(Box::new(x), sym),
            FuncAPF(apf) => FuncApp(Box::new(x), Box::new(apf)),
        },
    )
});
