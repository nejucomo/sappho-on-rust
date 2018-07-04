use value::Atom;

#[derive(Debug)]
pub enum Expr {
    Atom(Atom),
    Deref(String),
    List(Vec<Expr>),
}
