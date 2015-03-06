use collections::string::String;

use super::types::{Expression};


pub type ParseResult = Result<Expression, String>;


pub fn parse_expression(source: &str) -> ParseResult {
    peg::expr(source)
}


// Private implementation innards below:
peg_file! peg("sappho.rustpeg");


#[cfg(test)]
mod tests;

