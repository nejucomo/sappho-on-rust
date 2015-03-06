mod parser;
mod types;


pub use self::parser::ParseResult;

pub use self::types::{
    Expression,
    Callable,
    Literal,
};


pub fn parse_expression(source: &str) -> ParseResult {
    parser::parse_expression(source)
}


#[cfg(test)]
mod tests;

