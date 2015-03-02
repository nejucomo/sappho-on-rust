use std::collections::HashMap;


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expression {
    Dereference(Identifier),
    Literal(Literal),
    Object(Object),
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
    props: Properties,
}

impl Object {
    pub fn empty() -> Object {
        Object::from_func(Function::empty())
    }

    pub fn from_func(f: Function) -> Object {
        Object { func: f, props: Properties::empty() }
    }
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Function(pub Vec<FuncRule>);

impl Function {
    pub fn empty() -> Function {
        Function(vec![])
    }
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct FuncRule {
    pub pattern: Pattern,
    pub body: Expression,
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Pattern {
    Bind(Identifier),
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Properties {
    map: HashMap<Identifier, Box<Expression>>,
    varprop: Option<(Identifier, Box<Expression>)>,
}

impl Properties {
    pub fn empty() -> Properties {
        Properties {
            map: HashMap::new(),
            varprop: None,
        }
    }
}


pub type Identifier = String;
