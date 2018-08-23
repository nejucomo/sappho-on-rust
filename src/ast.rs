use value::{Atom, Symbol};

#[derive(Clone, Debug)]
pub struct Identifier(pub String);

#[derive(Clone, Debug)]
pub struct FuncExpr(pub CompoundExpr<FuncExpr>);

#[derive(Clone, Debug)]
pub enum QueryExpr {
    Compound(CompoundExpr<QueryExpr>),
    Resolve(Box<QueryExpr>),
}

#[derive(Clone, Debug)]
pub enum ProcExpr {
    Compound(CompoundExpr<ProcExpr>),
    Resolve(Box<ProcExpr>),
    Mutate(Box<ProcExpr>),
}

#[derive(Clone, Debug)]
pub enum CompoundExpr<X> {
    Atom(Atom),
    Deref(Identifier),
    List(Vec<X>),
    LookupApp(Box<X>, Symbol),
    FuncApp(Box<X>, Box<X>),
    UnApp(UnaryOperator, Box<X>),
    BinOp(BinaryOperator, Box<X>, Box<X>),
    Lambda(LambdaDefinition),
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Negate,
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
