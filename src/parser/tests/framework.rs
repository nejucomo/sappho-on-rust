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
    Proc,
    PropApplication,
    Properties,
    PropItem,
    Query,
};
use super::super::{
    parse_expression,
};


pub fn check_parse_expectation(inputs: &[&str], expectation: Option<Expression>) {
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
pub fn expr<T: IntoExpression>(x: T) -> Expression {
    x.into_expr()
}

pub fn qapp<T: IntoExpression>(x: T) -> Expression {
    Expression::QueryApp(Box::new(x.into_expr()))
}

pub fn papp<T: IntoExpression>(x: T) -> Expression {
    Expression::ProcApp(Box::new(x.into_expr()))
}

pub fn query<T: IntoExpression>(x: T) -> Query {
    Query(Box::new(x.into_expr()))
}

pub fn patitem<T: IntoExpression>(p: Pattern, x: T) -> PatternItem {
    PatternItem { pattern: p, expr: Box::new(x.into_expr()) }
}

pub fn propitem(id: &str, expr: Expression) -> PropItem {
    (id.to_string(), Box::new(expr))
}

pub fn lookup<T: IntoExpression>(target: T, propname: &str) -> Expression {
    PropApplication::Lookup(
        Box::new(target.into_expr()),
        propname.to_string(),
        ).into_expr()
}

pub fn dispatch<T: IntoExpression, U: IntoExpression>
    (target: T, proparg: U)
     -> Expression
{
    PropApplication::Dispatch(
        Box::new(target.into_expr()),
        Box::new(proparg.into_expr()),
        ).into_expr()
}


/* Private plumbing below */
trait IntoExpression {
    fn into_expr(self) -> Expression;
}

impl IntoExpression for Expression {
    fn into_expr(self) -> Expression { self }
}
impl IntoExpression for LeafExpression {
    fn into_expr(self) -> Expression {
        Expression::Leaf(self)
    }
}
impl IntoExpression for PropApplication {
    fn into_expr(self) -> Expression {
        Expression::PropApp(self)
    }
}
impl<T: IntoExpression> IntoExpression for Vec<T> {
    fn into_expr(self) -> Expression {
        Expression::List(
            List(
                FromIterator::from_iter(
                    self.into_iter().map(
                        |x| Box::new(x.into_expr())))))
    }
}
impl IntoExpression for Let {
    fn into_expr(self) -> Expression {
        Expression::Let(self)
    }
}


macro_rules! define_into_impls_for_leafs {
    ( $( $source:ty ),* ) => {
        $(
            impl IntoExpression for $source {
                fn into_expr(self) -> Expression {
                    self.into_leaf().into_expr()
                }
            }
        )*
    }
}


define_into_impls_for_leafs! {
    bool,
    PatternItem,
    Function,
    Identifier,
    Object,
    Proc,
    Properties,
    Query,
    &'static str
}


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
impl IntoLeaf for PatternItem {
    fn into_leaf(self) -> LeafExpression {
        Function(vec![self]).into_leaf()
    }
}
impl IntoLeaf for Properties {
    fn into_leaf(self) -> LeafExpression {
        Object::from_properties(self).into_leaf()
    }
}
