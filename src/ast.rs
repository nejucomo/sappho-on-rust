use value::{Atom, Symbol};

#[derive(Clone, Debug)]
pub enum Expr {
    Atom(Atom),
    Deref(String),
    List(Vec<Expr>),
    LookupApp(Box<Expr>, Symbol),
    FuncApp(Box<Expr>, Box<Expr>),
    UnApp(UnaryOperator, Box<Expr>),
    BinOp(BinaryOperator, Box<Expr>, Box<Expr>),
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
