use value::{Atom, Symbol};

#[derive(Clone, Debug)]
pub struct Identifier(pub String);

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

#[derive(Clone, Debug)]
pub enum FuncUnOp {
    Invert,
}

#[derive(Clone, Debug)]
pub enum QueryUnOp {
    FUO(FuncUnOp),
    Resolve,
}

#[derive(Clone, Debug)]
pub enum ProcUnOp {
    QUO(QueryUnOp),
    Mutate,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Times,
}

#[derive(Clone, Debug)]
pub struct LambdaDefinition {
    pub func: Option<FunctionDefinition>,
    pub query: Option<QueryDefinition>,
    pub proc: Option<ProcDefinition>,
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition(pub Identifier, pub Box<FuncExpr>);

impl From<FunctionDefinition> for LambdaDefinition {
    fn from(fd: FunctionDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: Some(fd),
            query: None,
            proc: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueryDefinition(pub Box<QueryExpr>);

impl From<QueryDefinition> for LambdaDefinition {
    fn from(qd: QueryDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: None,
            query: Some(qd),
            proc: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ProcDefinition {
    Return(Option<Box<ProcExpr>>),
}

impl From<ProcDefinition> for LambdaDefinition {
    fn from(pd: ProcDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: None,
            query: None,
            proc: Some(pd),
        }
    }
}
