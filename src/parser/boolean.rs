use combine::ParseResult;


pub fn boolean(input: &str) -> ParseResult<bool, &str>
{
    use combine::{Parser, ParserExt, string, value};

    string("true").with(value(true))
        .or(string("false").with(value(false)))
        .parse_state(input)
}


#[cfg(test)]
mod tests {
    use super::boolean;

    test_case_simple_parser!(
        boolean, test_boolean,
        |s: &str| {
            match s {
                "true" => true,
                "false" => false,
                s => unreachable!("invalid boolean.accept test-vector: {:?}", s),
            }
        });
}
