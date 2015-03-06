/** A parse-layer AST
 *
 * "parse-layer" means the parser is able to produce this AST structure
 * with minimal contortions, and thus this AST resembles the surfact
 * syntax closely.
 *
 * Someday there may be a different structure for representing the result
 * of translations and optimizations.
 *
 * Note: This grammar does not distinguish between, D/Q/P (deterministic,
 * query, proc) subgrammars.  This simplifies the AST and parsing
 * productions, but requires a subsequent validity check on the resulting
 * AST (and subtly also changes violations from a syntax error to a
 * different static verification error).
 *
 **/

use std::iter::FromIterator;
use std::collections::HashMap;


pub type Identifier = String;


/** Top-Level Expressions **/
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expression {
    Leaf(LeafExpression),
    ProcApp(Box<Expression>),
    QueryApp(Box<Expression>),
    PropApp(PropApplication),
    List(List),
    Let(Let),
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum LeafExpression {
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
    pub proc_:  Option<Proc>,
    pub query: Option<Query>,
    pub func:  Function,
    pub props: Properties,
}

impl Object {
    pub fn empty() -> Object {
        Object::from_func(Function::empty())
    }

    pub fn from_proc(p: Proc) -> Object {
        Object {
            proc_: Some(p),
            query: None,
            func: Function::empty(),
            props: Properties::empty(),
        }
    }

    pub fn from_query(q: Query) -> Object {
        Object {
            proc_: None,
            query: Some(q),
            func: Function::empty(),
            props: Properties::empty(),
        }
    }

    pub fn from_func(f: Function) -> Object {
        Object {
            proc_: None,
            query: None,
            func: f,
            props: Properties::empty(),
        }
    }

    pub fn from_properties(p: Properties) -> Object {
        Object {
            proc_: None,
            query: None,
            func: Function::empty(),
            props: p,
        }
    }
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Proc(pub StatementBlock);


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum StatementBlock {
    Return(Box<Expression>),
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Query(pub Box<Expression>);


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Function(pub Vec<PatternItem>);

impl Function {
    pub fn empty() -> Function {
        Function(vec![])
    }
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
    pub varprop: Option<PropItem>,
}

pub type PropItem = (Identifier, Box<Expression>);


impl Properties {
    pub fn empty() -> Properties {
        Properties {
            map: HashMap::new(),
            varprop: None,
        }
    }

    pub fn from_varprop(id: Identifier, expr: Expression) -> Properties {
        Properties::from_items(vec![], Some((id, Box::new(expr))))
    }

    pub fn from_items(concretes: Vec<PropItem>,
                      vp: Option<PropItem>)
                      -> Properties
    {
        let mut m = HashMap::with_capacity(concretes.len());

        for (id, expr) in concretes.into_iter() {
            m.insert(id, expr);
        }

        Properties { map: m, varprop: vp }
    }

    pub fn plus_item(mut self, id: Identifier, expr: Expression) -> Properties {
        self.map.insert(id, Box::new(expr));

        self
    }
}


/** Common Compound Expressions **/
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum PropApplication {
    Lookup(Box<Expression>, Identifier),
    Dispatch(Box<Expression>, Box<Expression>),
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct List(pub Vec<Box<Expression>>);


impl List {
    pub fn from_unboxed_vec(xs: Vec<Expression>) -> List {
        List(
            FromIterator::from_iter(
                xs.into_iter().map(
                    |x| Box::new(x))))
    }
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Let {
    pub bindings: Vec<PatternItem>,
    pub expr: Box<Expression>,
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct PatternItem {
    pub pattern: Pattern,
    pub expr: Box<Expression>,
}
