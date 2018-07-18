use value::{Atom, Symbol};

#[derive(Clone, Debug)]
pub enum Expr {
    Atom(Atom),
    Deref(String),
    List(Vec<Expr>),
    UnApp(UnaryApplication),
    LookupApp(LookupApplication),
    // FuncApp(FunctionalApplication),
}

#[derive(Clone, Debug)]
pub struct UnaryApplication(pub UnaryOperator, pub Box<Expr>);

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Query,
    Mutate,
}

#[derive(Clone, Debug)]
pub struct LookupApplication(pub Box<Expr>, pub Symbol);

/*
#[derive(Debug)]
pub struct FunctionalApplication(pub Box<Expr>, pub Box<Expr>);
*/
