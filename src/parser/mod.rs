use collections::string::String;

use super::ast;


// The main top-level interface to the parser:
pub fn parse_expression(source: &str) -> ParseResult {
    peg::expression(source)
}

type ParseResult = Result<ast::Expression, String>;


// Private implementation innards below:
peg_file! peg("sappho.rustpeg");


mod tests;

