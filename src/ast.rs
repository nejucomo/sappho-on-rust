use value::{Atom, Symbol};

#[derive(Clone, Debug)]
pub struct Identifier(pub String);

#[derive(Clone, Debug)]
pub enum Expr {
    Atom(Atom),
    Deref(Identifier),
    List(Vec<Expr>),
    LookupApp(Box<Expr>, Symbol),
    FuncApp(Box<Expr>, Box<Expr>),
    UnApp(UnaryOperator, Box<Expr>),
    BinOp(BinaryOperator, Box<Expr>, Box<Expr>),
    Lambda(LambdaDefinition),
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
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
pub struct FunctionDefinition(pub Identifier, pub Box<Expr>);

impl From<FunctionDefinition> for LambdaDefinition {
    fn from(fd: FunctionDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: Some(fd),
            query: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueryDefinition(pub Box<Expr>);

impl From<QueryDefinition> for LambdaDefinition {
    fn from(qd: QueryDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: None,
            query: Some(qd),
        }
    }
}
