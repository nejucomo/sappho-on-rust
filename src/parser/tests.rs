use super::super::ast::{
    Expression,
    Function,
    FuncRule,
    Identifier,
    Literal,
    Object,
    Pattern,
    Properties,
};
use super::{
    parse_expression,
};


fn check_parse_expectation(inputs: &[&str], expectation: Option<Expression>) {
    for input in inputs.iter() {
        let result = parse_expression(input).ok();
        assert!(result == expectation,
                "Parse expectation failure:\nInput: {:?}\nExpectation: {:?}\nResult: {:?}\n",
                input, expectation, result);
    }
}


macro_rules! test_parse_expectations {
    ( $( $name:ident : $inputs:expr => $expectation:expr );* ) => {
        $(
            #[test]
            fn $name () { check_parse_expectation( $inputs, $expectation ) }
        )*
    }
}


// A helper trait for concisely specifying tests:
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
impl IntoExpr for Properties {
    fn into_expr(self) -> Expression {
        iexpr(Object::from_properties(self))
    }
}

fn iexpr<T: IntoExpr>(x: T) -> Expression {
    IntoExpr::into_expr(x)
}


// Test cases:
test_parse_expectations! {
    literal_true
        : &["true"]
        => Some(iexpr(true));

    literal_false
        : &["false"]
        => Some(iexpr(false));

    dereference
        : &["x"]
        => Some(iexpr("x"));

    dangling_keyword
        : &["object"]
        => None;

    empty_object
        : &["object {}",
            "object { }",
            "object {\n}"]
        => Some(iexpr(Object::empty()));

    object_braces_malformed
        : &["object\n{}",
            "object\t{}",
            "object{}"]
        => None;

    empty_func
        : &["object { func { } }",
            "object {func {}}",
            "object { func {} }",
            "object {func {}}",
            "object {\n  func {\n  }\n}",

            // func-only-object shorthand:
            "func { }",
            "func {}",
            "func {\n}"]
        => Some(iexpr(Function::empty()));

    identity_func
        : &["object { func { x -> x } }",
            "object {func { x -> x }}",
            "object { func {x -> x} }",
            "object {func {x -> x}}",
            "object {\n  func {\n    x -> x\n  }\n}",
            "object {\n  func x -> x\n}",

            // func-only-object shorthand:
            "func { x -> x }",
            "func {x -> x}",
            "func {\n  x -> x\n}",
            "func x -> x"]
        => Some(
            iexpr(
                Function(
                    vec![
                        FuncRule {
                            pattern: Pattern::Bind("x".to_string()),
                            body: Expression::Dereference("x".to_string()),
                        }])));

    func_braces_malformed
        : &["object {func{}}",
            "object {func\n{}}",
            "func{ }",
            "func\n{}"]
        => None;

    properties
        : &["object { prop .t -> true; prop .f -> false; prop (x) -> x }"]
        => Some(
            iexpr(
                Properties::from_items(
                    vec![
                        ("t".to_string(),
                         Box::new(Expression::Literal(Literal::Bool(true)))),
                        ("f".to_string(),
                         Box::new(Expression::Literal(Literal::Bool(false))))],
                    Some(
                        ("x".to_string(),
                         Box::new(Expression::Dereference("x".to_string())))))));

    concrete_properties
        : &["object { prop .t -> true; prop .f -> false }"]
        => Some(
            iexpr(
                Properties::from_items(
                    vec![
                        ("t".to_string(),
                         Box::new(Expression::Literal(Literal::Bool(true)))),
                        ("f".to_string(),
                         Box::new(Expression::Literal(Literal::Bool(false))))],
                    None)))
}
