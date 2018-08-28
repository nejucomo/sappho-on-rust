use value::{Atom, Symbol};

use ast::{BinaryOperator, FuncUnOp, Identifier, LambdaDefinition, ProcUnOp, QueryUnOp};

pub type FuncExpr = Expr<FuncUnOp>;
pub type QueryExpr = Expr<QueryUnOp>;
pub type ProcExpr = Expr<ProcUnOp>;

#[derive(Clone, Debug)]
pub enum Expr<OP> {
    Atom(Atom),
    Deref(Identifier),
    List(Vec<Expr<OP>>),
    LookupApp(Box<Expr<OP>>, Symbol),
    FuncApp(Box<Expr<OP>>, Box<Expr<OP>>),
    UnApp(OP, Box<Expr<OP>>),
    BinOp(BinaryOperator, Box<Expr<OP>>, Box<Expr<OP>>),
    Lambda(LambdaDefinition),
}
