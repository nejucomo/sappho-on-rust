#![deny(warnings)]

extern crate combine;
extern crate num;

#[cfg(test)]
#[macro_use]
extern crate include_dir;

mod ast;
mod parser;
mod value;

fn main() {
    use combine::{eof, Parser};
    use parser::func_expr;
    use parser::terminal::keywords::Keyword;
    use std::io::BufRead;

    // Keyword coverage in main:
    let keywords = Keyword::all();
    println!("Known keywords: {:?}", keywords);
    for kw in keywords {
        let kwstr = kw.as_str();
        assert_eq!(Ok(((), "")), kw.parser().parse(kwstr));
    }

    fn prompt() {
        use std::io::Write;
        let mut stdout = std::io::stdout();
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();
    }

    prompt();

    let stdin = std::io::stdin();
    for lineres in stdin.lock().lines() {
        let line = lineres.unwrap();
        println!("input: {:?}", &line);
        let result = func_expr().skip(eof()).parse(&line);
        println!("result: {:?}", &result);
        prompt();
    }
}
