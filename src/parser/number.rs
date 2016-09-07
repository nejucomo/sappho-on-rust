use combine::{ParseResult};
use value::Number;


pub fn number(input: &str) -> ParseResult<Number, &str>
{
    use combine::{Parser, ParserExt, digit, many1};
    use num::{BigRational, Num};

    many1(digit())
        .and_then(
            |s: String| {
                BigRational::from_str_radix(s.as_str(), 10)
                    .map(Number::from_bigrational)
            })
        .parse_state(input)
}


#[cfg(test)]
mod tests {
    use super::number;

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
                    use combine::{Parser, ParserExt, eof, parser};

                    let inputs: Vec<&str> = Vec::from_iter(
                        include_cases!(
                            concat!(
                                "test-vectors/number/case.",
                                stringify!($case_name))));

                    for i in 0..inputs.len() {
                        for j in (i+1)..inputs.len() {
                            let ares = parser(number).skip(eof()).parse(inputs[i]);
                            let bres = parser(number).skip(eof()).parse(inputs[j]);

                            assert!(ares.is_ok());
                            assert!(bres.is_ok());

                            let (a, arem) = ares.unwrap();
                            let (b, brem) = bres.unwrap();

                            assert_eq!(arem, "");
                            assert_eq!(brem, "");

                            assert_eq!(a, b);
                        }
                    }
                }
            )*
        }
    }

    test_cases_number_parser!(zero);
}
