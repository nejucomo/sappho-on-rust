use combine::{ParseResult};
use value::Number;


pub fn number(input: &str) -> ParseResult<Number, &str>
{
    use combine::{Parser, ParserExt, char, parser};

    char('+')
        .with(parser(signless_number))
        .or(char('-')
            .with(parser(signless_number))
            .map(|n| -n))
        .or(parser(signless_number))
        .parse_state(input)
}


pub fn signless_number(input: &str) -> ParseResult<Number, &str>
{
    use combine::{Parser, ParserExt, digit, many1};

    many1(digit())
        .and_then(
            |s: String| {
                use num::{BigInt, BigRational, Num};

                BigInt::from_str_radix(s.as_str(), 10)
                    .map(BigRational::from_integer)
                    .map(Number::from_bigrational)
            })
        .parse_state(input)
}


#[cfg(test)]
mod tests {
    use super::number;
    use value::Number;

    macro_rules! include_number {
        ($case_name:expr) => {
            include_without_newline!(
                concat!(
                    "test-vectors/number/case",
                    $case_name,
                    ".",
                    $ab));
        }
    }

    macro_rules! test_cases_number_parser {
        ( $( $case_name:ident ),* ) => {
            $(
                #[test]
                fn $case_name() {
                    use std::iter::FromIterator;

                    let inputs: Vec<&str> = Vec::from_iter(
                        include_cases!(
                            concat!(
                                "test-vectors/number/case.",
                                stringify!($case_name))));

                    for i in 0..inputs.len() {
                        for j in (i+1)..inputs.len() {
                            let a = test_parse_number(inputs[i]);
                            let b = test_parse_number(inputs[j]);
                            assert_eq!(a, b);
                        }
                    }
                }
            )*
        }
    }

    test_cases_number_parser!(zero);

    fn test_parse_number(input: &str) -> Number {
        use combine::{Parser, ParserExt, eof, parser};

        let res = parser(number).skip(eof()).parse(input);
        assert!(res.is_ok(), "Failed to parse: {:?}", input);
        let (x, rem) = res.unwrap();
        assert_eq!(rem, "");

        x
    }
}
