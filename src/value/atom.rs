use value::number::Number;

#[derive(Debug)]
pub enum Atom {
    Bool(bool),
    Number(Number),
    Char(char),
    Text(String),
    Symbol(String),
}
