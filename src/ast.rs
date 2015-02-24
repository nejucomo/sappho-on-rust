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
pub struct Object {
    func: Function,
}

impl Object {
    pub fn empty() -> Object {
        Object::from_func(vec![])
    }

    pub fn from_func(f: Function) -> Object {
        Object { func: f }
    }
}


type Function = Vec<FuncRule>;


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct FuncRule {
    pattern: Pattern,
    body: Expression,
}


type Pattern = ();


