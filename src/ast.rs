#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Object,
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Literal {
    Bool(bool),
}


