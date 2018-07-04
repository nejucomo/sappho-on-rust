use value::Atom;

#[derive(Debug)]
pub enum Expr {
    Atom(Atom),
    Deref(String),
    List(Vec<Expr>),
    UnApp(UnaryApplication),
}

#[derive(Debug)]
pub struct UnaryApplication(pub UnaryOperator, pub Box<Expr>);

#[derive(Debug)]
pub enum UnaryOperator {
    Query,
    Mutate,
}
