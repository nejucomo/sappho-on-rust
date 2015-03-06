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
    Application,
    Callable,
    Expression,
    Function,
    Identifier,
    Let,
    List,
    Literal,
    Object,
    Pattern,
    PatternItem,
    Proc,
    Properties,
    PropItem,
    Query,
    Uncallable,
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

pub fn qapp<T: IntoCallable>(x: T) -> Callable {
    Callable::QueryApp(Box::new(x.into_callable()))
}

pub fn papp<T: IntoCallable>(x: T) -> Callable {
    Callable::ProcApp(Box::new(x.into_callable()))
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

pub fn lookup<T: IntoCallable>(target: T, propname: &str) -> Expression {
    Expression::Apps(
        target.into_callable(),
        vec![
            Application::Lookup(propname.to_string()),
            ])
}

pub fn dispatch<T: IntoCallable, U: IntoExpression>
    (target: T, proparg: U)
     -> Expression
{
    Expression::Apps(
        target.into_callable(),
        vec![
            Application::Dispatch(Box::new(proparg.into_expr())),
            ])
}


/* Private plumbing below */
trait IntoExpression {
    fn into_expr(self) -> Expression;
}

impl IntoExpression for Expression {
    fn into_expr(self) -> Expression { self }
}
impl IntoExpression for Callable {
    fn into_expr(self) -> Expression { Expression::Apps(self, vec![]) }
}

impl IntoExpression for Uncallable {
    fn into_expr(self) -> Expression {
        Expression::Uncallable(self)
    }
}
impl IntoExpression for Object {
    fn into_expr(self) -> Expression {
        Uncallable::Object(self).into_expr()
    }
}
impl IntoExpression for Let {
    fn into_expr(self) -> Expression {
        Uncallable::Let(self).into_expr()
    }
}

impl IntoExpression for Proc {
    fn into_expr(self) -> Expression {
        Object::from_proc(self).into_expr()
    }
}
impl IntoExpression for Query {
    fn into_expr(self) -> Expression {
        Object::from_query(self).into_expr()
    }
}
impl IntoExpression for Function {
    fn into_expr(self) -> Expression {
        Object::from_func(self).into_expr()
    }
}
impl IntoExpression for PatternItem {
    fn into_expr(self) -> Expression {
        Function(vec![self]).into_expr()
    }
}
impl IntoExpression for Properties {
    fn into_expr(self) -> Expression {
        Object::from_properties(self).into_expr()
    }
}


trait IntoCallable {
    fn into_callable(self) -> Callable;
}
impl IntoCallable for Callable {
    fn into_callable(self) -> Callable { self }
}
impl IntoCallable for Expression {
    fn into_callable(self) -> Callable {
        Callable::Parens(Box::new(self))
    }
}

impl<T: IntoExpression> IntoCallable for Vec<T> {
    fn into_callable(self) -> Callable {
        Callable::List(
            List(
                FromIterator::from_iter(
                    self.into_iter().map(
                        |x| Box::new(
                            x.into_expr())))))
    }
}
impl<T: IntoExpression> IntoExpression for Vec<T> {
    fn into_expr(self) -> Expression {
        self.into_callable().into_expr()
    }
}


macro_rules! into_expression_via_callable {
    ( $t:ty ) => {
        impl IntoExpression for $t {
            fn into_expr(self) -> Expression {
                self.into_callable().into_expr()
            }
        }
    }
}

into_expression_via_callable! { () }
impl IntoCallable for () {
    fn into_callable(self) -> Callable {
        Callable::List(List(vec![]))
    }
}

into_expression_via_callable! { Identifier }
impl IntoCallable for Identifier {
    fn into_callable(self) -> Callable {
        Callable::Dereference(self)
    }
}

into_expression_via_callable! { &'static str }
impl IntoCallable for &'static str {
    fn into_callable(self) -> Callable {
        self.to_string().into_callable()
    }
}

into_expression_via_callable! { bool }
impl IntoCallable for bool {
    fn into_callable(self) -> Callable {
        Callable::Literal(Literal::Bool(self))
    }
}
