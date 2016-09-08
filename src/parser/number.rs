use combine::{ParseResult};
use value::Number;
use num::BigRational;


pub fn number(input: &str) -> ParseResult<Number, &str>
{
    use combine::{Parser, ParserExt, char, parser};

    char('+')
        .with(parser(signless_number))
        .or(char('-')
            .with(parser(signless_number))
            .map(|n| -n))
        .or(parser(signless_number))
        .map(Number::from_bigrational)
        .parse_state(input)
}


pub fn signless_number(input: &str) -> ParseResult<BigRational, &str>
{
    use combine::{Parser, ParserExt, try, parser};

    try(parser(zero_or_hexbin_number))
        .or(parser(decimal_number))
        .parse_state(input)
}

pub fn zero_or_hexbin_number(input: &str) -> ParseResult<BigRational, &str>
{
    use combine::{
        Parser,
        ParserExt,
        char,
        hex_digit,
        many1,
        satisfy,
    };
    use num::{BigInt, BigRational, Num};

    char('0').with(
        char('x')
            .with(many1(hex_digit())
                  .and_then(
                      |s: String| BigInt::from_str_radix(s.as_str(), 16)))
            .or(
                char('b')
                    .with(many1(satisfy(|c| c == '0' || c == '1'))
                          .and_then(
                              |s: String| BigInt::from_str_radix(s.as_str(), 2)))))
        .map(BigRational::from_integer)
        .parse_state(input)
}

pub fn decimal_number(input: &str) -> ParseResult<BigRational, &str>
{
    use combine::{Parser, ParserExt, char, digit, optional, many1};
    many1(digit())
        .and(optional(char('.').with(many1(digit()))))
        .and(optional(char('e').or(char('E')).with(many1(digit()))))
        .and_then(
            |((mut digs, optdec), optexp): ((String, Option<String>), Option<String>)| {
                use num::{BigInt, FromPrimitive, Num, pow};

                let decplaces = match optdec {
                    None => 0,

                    Some(dec) => {
                        digs.push_str(dec.as_str());
                        dec.len()
                    }
                };
                let exp = optexp.map(|s| s.parse::<usize>().unwrap()).unwrap_or(1);
                let places = decplaces + exp;
                let ten = BigInt::from_u64(10).unwrap();
                let denom = pow(BigRational::from_integer(ten), places);

                BigInt::from_str_radix(digs.as_str(), 10)
                    .map(BigRational::from_integer)
                    .map(|num| num / denom)
            })
        .parse_state(input)
}


#[cfg(test)]
mod tests {
    use super::number;
    use value::Number;

    #[test]
    fn reject() {
        use combine::{Parser, ParserExt, eof, parser};

        for input in include_cases!("test-vectors/number/reject") {
            let res = parser(number).skip(eof()).parse(input);
            assert!(res.is_err(), "Incorrectly parsed as number: {:?}", input);
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

    test_cases_number_parser!(
        zero,
        one);

    fn test_parse_number(input: &str) -> Number {
        use combine::{Parser, ParserExt, eof, parser};

        let res = parser(number).skip(eof()).parse(input);
        assert!(res.is_ok(), "Failed to parse: {:?}", input);
        let (x, rem) = res.unwrap();
        assert_eq!(rem, "");

        x
    }
}
