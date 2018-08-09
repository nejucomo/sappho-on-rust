use value::{Atom, Symbol};

#[derive(Clone, Debug)]
pub struct Identifier(pub String);

#[derive(Clone, Debug)]
pub enum GenExpr<T> {
    Atom(Atom),
    Deref(Identifier),
    List(Vec<GenExpr<T>>),
    LookupApp(Box<GenExpr<T>>, Symbol),
    FuncApp(Box<GenExpr<T>>, Box<GenExpr<T>>),
    UnApp(T, Box<GenExpr<T>>),
    BinOp(BinaryOperator, Box<GenExpr<T>>, Box<GenExpr<T>>),
    Lambda(LambdaDefinition),
}

// Concrete Expression types:
pub type FuncExpr = GenExpr<FuncUnOp>;
pub type QueryExpr = GenExpr<QueryUnOp>;
pub type ProcExpr = GenExpr<ProcUnOp>;

#[derive(Clone, Debug)]
pub enum FuncUnOp {
    NotValid,
}

#[derive(Clone, Debug)]
pub enum QueryUnOp {
    NotValid,
    Query,
}

#[derive(Clone, Debug)]
pub enum ProcUnOp {
    Query,
    Mutate,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Times,
}

#[derive(Clone, Debug)]
pub struct LambdaDefinition {
    func: Option<FunctionDefinition>,
    query: Option<QueryDefinition>,
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition(pub Identifier, pub Box<FuncExpr>);

impl From<FunctionDefinition> for LambdaDefinition {
    fn from(fd: FunctionDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: Some(fd),
            query: None,
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
        }
    }
}
