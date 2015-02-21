#![feature(plugin)]
#![plugin(peg_syntax_ext)]

fn main() {
    print_parse("foo");
}

fn print_parse(src: &str) {
    println!("Source: {}", src);
    match parser::expression(src) {
        Ok(v) => println!("Ok({})", v),
        Err(e) => println!("Err({})", e),
    }
}

peg_file! parser("sappho.rustpeg");
