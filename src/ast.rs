/** A parse-layer AST
 *
 * "parse-layer" means the parser is able to produce this AST structure
 * with minimal contortions, and thus this AST resembles the surfact
 * syntax closely.
 *
 * Someday there may be a different structure for representing the result
 * of translations and optimizations.
 *
 **/

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
pub enum DGrammar {
    Expr(Expression<DGrammar>),
}


/* The Query expression grammar (QGrammar) allows queries of mutable
 * state.
 */
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum QGrammar {
    Expr(Expression<QGrammar>),
    QueryApp(Box<QGrammar>),
}


/* PGrammar, the Proc grammar, allows mutations and i/o. */
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum PGrammar {
    Expr(Expression<PGrammar>),
    QueryApp(Box<PGrammar>),
    ProcApp(Box<PGrammar>),
}


/* All grammars share a common expression syntax: */
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expression<T> {
    Leaf(LeafExpression),
    List(List<T>),
    Let(Let<T>),
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
    Return(Box<PGrammar>),
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Query(pub Box<QGrammar>);


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Function(pub Vec<PatternItem<DGrammar>>);

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
    pub map: HashMap<Identifier, Box<DGrammar>>,
    pub varprop: Option<PropItem>,
}

pub type PropItem = (Identifier, Box<DGrammar>);


impl Properties {
    pub fn empty() -> Properties {
        Properties {
            map: HashMap::new(),
            varprop: None,
        }
    }

    pub fn from_varprop(id: Identifier, expr: DGrammar) -> Properties {
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

    pub fn plus_item(mut self, id: Identifier, expr: DGrammar) -> Properties {
        self.map.insert(id, Box::new(expr));

        self
    }
}


/** Common Compound Expressions **/

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


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Let<T> {
    pub bindings: Vec<PatternItem<T>>,
    pub expr: Box<T>,
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct PatternItem<T> {
    pub pattern: Pattern,
    pub expr: Box<T>
}
