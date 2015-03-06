#[cfg(test)]
#[macro_use]
mod test; // This must occur first for test macros use in other sub-mods.


mod parser;
mod types;
mod verifier;


pub use self::parser::ParseResult;

pub use self::types::{
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
    PropItem,
    Properties,
    Query,
    StatementBlock,
    Uncallable,
};


pub fn parse_verified_expression(source: &str) -> ParseResult {
    parser::parse_expression(source).and_then(verifier::verify_expression)
}

