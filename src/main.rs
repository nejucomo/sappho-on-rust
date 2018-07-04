#![deny(warnings)]

extern crate combine;
extern crate num;

#[cfg(test)]
#[macro_use]
mod test_macros;

mod ast;
mod parser;
mod value;

fn main() {
    use combine::{eof, parser, Parser};
    use parser::expr;
    use std::io::{stdin, Read};

    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    println!("input: {:?}", &s);
    let result = parser(expr).skip(eof()).parse(&s);
    println!("result: {:?}", &result);
}
