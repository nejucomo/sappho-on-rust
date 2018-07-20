use value::{Atom, Symbol};

#[derive(Clone, Debug)]
pub enum Expr {
    Atom(Atom),
    Deref(String),
    List(Vec<Expr>),
    UnApp(UnaryOperator, Box<Expr>),
    LookupApp(Box<Expr>, Symbol),
    FuncApp(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Query,
    Mutate,
}
