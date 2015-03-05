/* This mod provides a framework for concisely expression parser tests.
 * There are two utilities:
 *
 * - The test_parse_expectations! macro expands to separate test functions
 *   for each compactly specified case.
 * - The [dqp]gram*, query, proc, and propitem functions are a DSL for
 *   concisely specifying ast graphs which are otherwise textually large.
 */

use std::iter::FromIterator;
use super::super::super::ast::{
    DGrammar,
    Expression,
    Function,
    Identifier,
    LeafExpression,
    Let,
    List,
    Literal,
    Object,
    Pattern,
    PatternItem,
    PGrammar,
    Proc,
    Properties,
    PropItem,
    QGrammar,
    Query,
};
use super::super::{
    parse_expression,
};


pub fn check_parse_expectation(inputs: &[&str], expectation: Option<DGrammar>) {
    for input in inputs.iter() {
        let result = parse_expression(input);
        assert!(result.as_ref().ok() == expectation.as_ref(),
                "Parse expectation failure:\nInput: {:?}\nExpectation: {:?}\nResult: {:?}\n",
                input, expectation, result);
    }
}


#[macro_export]
macro_rules! test_parse_expectations {
    ( $( $name:ident : $inputs:expr => $expectation:expr );* ) => {
        $(
            #[test]
            fn $name () {
                $crate::parser::tests::framework::check_parse_expectation( $inputs, $expectation )
            }
        )*
    }
}


// helper fns & a trait for concisely specifying tests:
pub fn dgram<T: IntoDGrammar>(x: T) -> DGrammar {
    x.into_dgram()
}

pub fn qgram<T: IntoQGrammar>(x: T) -> QGrammar {
    x.into_qgram()
}

pub fn qgram_qapp<T: IntoQGrammar>(x: T) -> QGrammar {
    QGrammar::QueryApp(Box::new(x.into_qgram()))
}

pub fn pgram<T: IntoPGrammar>(x: T) -> PGrammar {
    x.into_pgram()
}

pub fn pgram_qapp<T: IntoPGrammar>(x: T) -> PGrammar {
    PGrammar::QueryApp(Box::new(x.into_pgram()))
}

pub fn pgram_papp<T: IntoPGrammar>(x: T) -> PGrammar {
    PGrammar::ProcApp(Box::new(x.into_pgram()))
}

pub fn query<T: IntoQGrammar>(x: T) -> Query {
    Query(Box::new(qgram(x)))
}

pub fn patitem<T>(p: Pattern, x: T) -> PatternItem<T> {
    PatternItem { pattern: p, expr: Box::new(x) }
}

pub fn propitem(id: &str, expr: DGrammar) -> PropItem {
    (id.to_string(), Box::new(expr))
}


/* Private and convoluted plumbing below */
macro_rules! define_into_impls_for_leafs {
    ( ( $target:ident, $traitname:ident, $methodname:ident ) : [ $( $source:ty ),* ] ) => {
        $(
            impl $traitname for $source {
                fn $methodname(self) -> $target {
                    self.into_leaf().$methodname()
                }
            }
        )*
    }
}


macro_rules! define_into_trait_and_impls {
    ( $target:ident, $traitname:ident, $methodname:ident ) => {

        trait $traitname {
            fn $methodname(self) -> $target;
        }
        impl $traitname for $target {
            fn $methodname(self) -> $target { self }
        }
        impl $traitname for LeafExpression {
            fn $methodname(self) -> $target {
                /* Note: This macro requires all consistent
                 * $target::Expr(Expression<$target>) structure.
                 */
                $target::Expr(Expression::Leaf(self))
            }
        }
        impl<T: $traitname> $traitname for Vec<T> {
            fn $methodname(self) -> $target {
                $target::Expr(
                    Expression::List(
                        List(
                            FromIterator::from_iter(
                                self.into_iter().map(
                                    |x| Box::new(x.$methodname()))))))
            }
        }
        impl $traitname for Let<$target> {
            fn $methodname(self) -> $target {
                $target::Expr(Expression::Let(self))
            }
        }

        define_into_impls_for_leafs! {
            ( $target, $traitname, $methodname )
                : [bool,
                   PatternItem<DGrammar>,
                   Function,
                   Identifier,
                   Object,
                   Proc,
                   Properties,
                   Query,
                   &'static str]
        }
    }
}


define_into_trait_and_impls! ( DGrammar, IntoDGrammar, into_dgram );
define_into_trait_and_impls! ( QGrammar, IntoQGrammar, into_qgram );
define_into_trait_and_impls! ( PGrammar, IntoPGrammar, into_pgram );



// Conversion plumbing for leaf expressions:
trait IntoLeaf {
    fn into_leaf(self) -> LeafExpression;
}
impl IntoLeaf for LeafExpression {
    fn into_leaf(self) -> LeafExpression { self }
}
impl IntoLeaf for Identifier {
    fn into_leaf(self) -> LeafExpression {
        LeafExpression::Dereference(self)
    }
}
impl IntoLeaf for &'static str {
    fn into_leaf(self) -> LeafExpression {
        self.to_string().into_leaf()
    }
}
impl IntoLeaf for bool {
    fn into_leaf(self) -> LeafExpression {
        LeafExpression::Literal(Literal::Bool(self))
    }
}
impl IntoLeaf for Object {
    fn into_leaf(self) -> LeafExpression {
        LeafExpression::Object(self)
    }
}
impl IntoLeaf for Proc {
    fn into_leaf(self) -> LeafExpression {
        Object::from_proc(self).into_leaf()
    }
}
impl IntoLeaf for Query {
    fn into_leaf(self) -> LeafExpression {
        Object::from_query(self).into_leaf()
    }
}
impl IntoLeaf for Function {
    fn into_leaf(self) -> LeafExpression {
        Object::from_func(self).into_leaf()
    }
}
impl IntoLeaf for PatternItem<DGrammar> {
    fn into_leaf(self) -> LeafExpression {
        Function(vec![self]).into_leaf()
    }
}
impl IntoLeaf for Properties {
    fn into_leaf(self) -> LeafExpression {
        Object::from_properties(self).into_leaf()
    }
}
