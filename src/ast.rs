use value::Atom;

#[derive(Debug)]
pub enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}
