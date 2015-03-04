use std::iter::FromIterator;
use std::collections::HashMap;


pub type Identifier = String;


/** Top-Level Expressions **/

/* The top level expression grammar is deterministic and excludes query
 * and proc applications (which are not deterministic).
 */
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expression {
    PLE(PureLeafExpression),
    LE(List<Expression>),
}


/* A QueryExpression tree is like a (deterministic) Expression tree,
 * except it may contain QueryApp productions.
 */
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum QueryExpression {
    PLE(PureLeafExpression),
    LE(List<QueryExpression>),
    QueryApp(Box<QueryExpression>),
}


/* A ProcExpression tree is like a QueryExpression tree,
 * except it may also contain ProcApp productions.
 */
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum ProcExpression {
    PLE(PureLeafExpression),
    LE(List<ProcExpression>),
    QueryApp(Box<ProcExpression>),
    ProcApp(Box<ProcExpression>),
}



/** PureLeafExpressions and subgrammars **/

/* A PureLeafExpression does not contain subexpressions which are
 * evaluated prerequisite to the PureLeafExpression itself.
 */
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum PureLeafExpression {
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
    Return(Box<ProcExpression>),
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Query(pub Box<QueryExpression>);


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


/** List Expressions **/
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct List<T>(pub Vec<Box<T>>);


impl<T> List<T> {
    pub fn from_unboxed_vec(xs: Vec<T>) -> List<T> {
        List(
            FromIterator::from_iter(
                xs.into_iter().map(
                    |x| Box::new(x))))
    }
}
