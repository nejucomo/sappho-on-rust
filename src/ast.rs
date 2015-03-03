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
    pub func: Function,
    pub props: Properties,
}

impl Object {
    pub fn empty() -> Object {
        Object::from_func(Function::empty())
    }

    pub fn from_func(f: Function) -> Object {
        Object { func: f, props: Properties::empty() }
    }

    pub fn from_properties(p: Properties) -> Object {
        Object { func: Function::empty(), props: p }
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
    pub map: HashMap<Identifier, Box<Expression>>,
    pub varprop: Option<(Identifier, Box<Expression>)>,
}

impl Properties {
    pub fn empty() -> Properties {
        Properties {
            map: HashMap::new(),
            varprop: None,
        }
    }

    pub fn from_items(concretes: Vec<(Identifier, Box<Expression>)>,
                      vp: Option<(Identifier, Box<Expression>)>)
                      -> Properties
    {
        let mut m = HashMap::with_capacity(concretes.len());

        for (id, expr) in concretes.into_iter() {
            m.insert(id, expr);
        }

        Properties { map: m, varprop: vp }
    }
}


pub type Identifier = String;
