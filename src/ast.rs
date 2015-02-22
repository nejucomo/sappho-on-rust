#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expression {
    Literal(Literal)
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Literal {
    Bool(bool)
}


