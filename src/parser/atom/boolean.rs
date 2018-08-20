use combine::Parser;

pub fn boolean<'a>() -> impl Parser<Input = &'a str, Output = bool> {
    use combine::char::string;
    use combine::{value, Parser};

    string("true")
        .with(value(true))
        .or(string("false").with(value(false)))
}

#[cfg(tests)]
parser_tests_mod!(
    tests,
    boolean,
    include_dir!("src/parser/test-vectors/boolean/")
);
