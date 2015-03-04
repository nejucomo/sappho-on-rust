use super::super::super::ast::{
    Expression,
    FuncRule,
    Function,
    Identifier,
    Literal,
    Object,
    Properties,
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


// A helper fn & trait for concisely specifying tests:
pub fn iexpr<T: IntoExpr>(x: T) -> Expression {
    IntoExpr::into_expr(x)
}


trait IntoExpr {
    fn into_expr(self) -> Expression;
}

impl IntoExpr for Identifier {
    fn into_expr(self) -> Expression {
        Expression::Dereference(self)
    }
}
impl IntoExpr for &'static str {
    fn into_expr(self) -> Expression {
        iexpr(self.to_string())
    }
}
impl IntoExpr for bool {
    fn into_expr(self) -> Expression {
        Expression::Literal(Literal::Bool(self))
    }
}
impl IntoExpr for Object {
    fn into_expr(self) -> Expression {
        Expression::Object(self)
    }
}
impl IntoExpr for Function {
    fn into_expr(self) -> Expression {
        iexpr(Object::from_func(self))
    }
}
impl IntoExpr for FuncRule {
    fn into_expr(self) -> Expression {
        iexpr(Function(vec![self]))
    }
}
impl IntoExpr for Properties {
    fn into_expr(self) -> Expression {
        iexpr(Object::from_properties(self))
    }
}
