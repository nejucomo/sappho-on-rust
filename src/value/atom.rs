use value::number::Number;
use value::symbol::Symbol;

#[derive(Clone, Debug)]
pub enum Atom {
    Bool(bool),
    Number(Number),
    Char(char),
    Text(String),
    Symbol(Symbol),
}
