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

#[test]
fn parse_expectations() {
    let cases = vec![
        ("true", Ok(true)),
        ("false", Ok(false)),
        ];

    for (input, expectation) in cases {
        let result = parser::expression(input);
        assert!(result == expectation,
                "Parse expectation failure:\nInput: {}\nExpectation: {}\nResult: {}\n",
                input, expectation, result);
    }
}
