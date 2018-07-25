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
    pub func: Option<FunctionDefinition>,
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition(pub Identifier, pub Box<Expr>);
