#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Object(Object),
    Dereference(String),
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Literal {
    Bool(bool),
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Object;


