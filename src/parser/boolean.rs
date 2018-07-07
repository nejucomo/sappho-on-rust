use combine::ParseResult;

pub fn boolean(input: &str) -> ParseResult<bool, &str> {
    use combine::char::string;
    use combine::{value, Parser};

    string("true")
        .with(value(true))
        .or(string("false").with(value(false)))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::boolean;

    test_case_simple_parser!(boolean, "boolean", test_boolean, |s: &str| match s {
        "true" => true,
        "false" => false,
        s => unreachable!("invalid boolean.accept test-vector: {:?}", s),
    });
}
