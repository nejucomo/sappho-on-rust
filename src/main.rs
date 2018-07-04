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
    use std::io::BufRead;

    let stdin = std::io::stdin();
    for lineres in stdin.lock().lines() {
        let line = lineres.unwrap();
        println!("input: {:?}", &line);
        let result = parser(expr).skip(eof()).parse(&line);
        println!("result: {:?}", &result);
    }
}
