use ast::CompoundExpr;
use combine::Parser;

pub trait TopExprParseInfo<'a>
where
    Self: Sized + Clone,
{
    type TopExprParser: Clone + Parser<Output = Self, Input = &'a str>;

    fn parser() -> Self::TopExprParser;

    fn wrap_compound(CompoundExpr<Self>) -> Self;

    fn box_compound(comp: CompoundExpr<Self>) -> Box<Self> {
        Box::new(Self::wrap_compound(comp))
    }
}
