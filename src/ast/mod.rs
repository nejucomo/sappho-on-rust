#[cfg(test)]
#[macro_use]
mod test; // This must occur first for test macros use in other sub-mods.


mod parser;
mod types;


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


pub fn parse_expression(source: &str) -> ParseResult {
    parser::parse_expression(source)
}

