use ast::Identifier;

#[derive(Clone, Debug)]
pub enum Pattern {
    Bind(Identifier),
}
