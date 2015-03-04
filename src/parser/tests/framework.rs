use super::super::super::ast::{
    Expression,
    FuncRule,
    Function,
    Identifier,
    Literal,
    Object,
    Properties,
    PropItem,
    PureLeafExpression,
    Query,
    QueryExpression,
};
use super::super::{
    parse_expression,
};


pub fn check_parse_expectation(inputs: &[&str], expectation: Option<Expression>) {
    for input in inputs.iter() {
        let result = parse_expression(input).ok();
        assert!(result == expectation,
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
pub fn expr<T: IntoExpr>(x: T) -> Expression {
    IntoExpr::into_expr(x)
}

pub fn qexpr<T: IntoExpr>(x: T) -> QueryExpression {
    IntoExpr::into_qexpr(x)
}

pub fn qapp<T: IntoExpr>(x: T) -> QueryExpression {
    QueryExpression::QueryApp(Box::new(qexpr(x)))
}

pub fn query<T: IntoExpr>(x: T) -> Query {
    Query(Box::new(qexpr(x)))
}


trait IntoExpr {
    fn into_expr(self) -> Expression;
    fn into_qexpr(self) -> QueryExpression;
}

impl IntoExpr for PureLeafExpression {
    fn into_expr(self) -> Expression {
        Expression::PLE(self)
    }
    fn into_qexpr(self) -> QueryExpression {
        QueryExpression::PLE(self)
    }
}
impl IntoExpr for QueryExpression {
    fn into_expr(self) -> Expression {
        expr(Query(Box::new(self)))
    }
    fn into_qexpr(self) -> QueryExpression {
        self
    }
}
impl IntoExpr for Identifier {
    fn into_expr(self) -> Expression {
        expr(PureLeafExpression::Dereference(self))
    }
    fn into_qexpr(self) -> QueryExpression {
        qexpr(PureLeafExpression::Dereference(self))
    }
}
impl IntoExpr for &'static str {
    fn into_expr(self) -> Expression {
        expr(self.to_string())
    }
    fn into_qexpr(self) -> QueryExpression {
        qexpr(self.to_string())
    }
}
impl IntoExpr for bool {
    fn into_expr(self) -> Expression {
        expr(PureLeafExpression::Literal(Literal::Bool(self)))
    }
    fn into_qexpr(self) -> QueryExpression {
        qexpr(PureLeafExpression::Literal(Literal::Bool(self)))
    }
}
impl IntoExpr for Object {
    fn into_expr(self) -> Expression {
        expr(PureLeafExpression::Object(self))
    }
    fn into_qexpr(self) -> QueryExpression {
        qexpr(PureLeafExpression::Object(self))
    }
}
impl IntoExpr for Query {
    fn into_expr(self) -> Expression {
        expr(Object::from_query(self))
    }
    fn into_qexpr(self) -> QueryExpression {
        qexpr(Object::from_query(self))
    }
}
impl IntoExpr for Function {
    fn into_expr(self) -> Expression {
        expr(Object::from_func(self))
    }
    fn into_qexpr(self) -> QueryExpression {
        qexpr(Object::from_func(self))
    }
}
impl IntoExpr for FuncRule {
    fn into_expr(self) -> Expression {
        expr(Function(vec![self]))
    }
    fn into_qexpr(self) -> QueryExpression {
        qexpr(Function(vec![self]))
    }
}
impl IntoExpr for Properties {
    fn into_expr(self) -> Expression {
        expr(Object::from_properties(self))
    }
    fn into_qexpr(self) -> QueryExpression {
        qexpr(Object::from_properties(self))
    }
}


// Helpers for PropItems:
pub fn propitem(id: &str, expr: Expression) -> PropItem {
    (id.to_string(), Box::new(expr))
}
