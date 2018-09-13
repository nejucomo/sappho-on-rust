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
    use parser::common::keywords::Keyword;
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
        use ast::ProcExpr;
        use parser::proc_expr;

        let line = lineres.unwrap();
        println!("input: {:?}", &line);
        let implicitbindings: Vec<&'static str> = vec![];
        let result: Result<(ProcExpr, _), _> = proc_expr(implicitbindings).skip(eof()).parse(&line);
        println!("result: {:?}", &result);
        prompt();
    }
}
