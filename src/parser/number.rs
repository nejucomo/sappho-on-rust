use combine::ParseResult;
use value::Number;

macro_rules! from_radix {
    ($t:ty, $radix:expr) => {
        |s: String| <$t>::from_str_radix(s.as_str(), $radix)
    };
}

macro_rules! signed {
    ($p:expr) => {{
        use combine::{char, ParserExt};

        char('+').with($p).or(char('-').with($p).map(|n| -n)).or($p)
    }};
}

pub fn number(input: &str) -> ParseResult<Number, &str> {
    use combine::{parser, Parser};

    signed!(parser(signless_number)).parse_state(input)
}

pub fn signless_number(input: &str) -> ParseResult<Number, &str> {
    use combine::{parser, try, Parser, ParserExt};

    try(parser(zero_or_hexbin_number))
        .or(parser(decimal_number))
        .parse_state(input)
}

pub fn zero_or_hexbin_number(input: &str) -> ParseResult<Number, &str> {
    use combine::{char, hex_digit, many1, satisfy, Parser, ParserExt};
    use num::{BigInt, Num};

    char('0')
        .with(
            char('x')
                .with(many1(hex_digit()).and_then(from_radix!(BigInt, 16)))
                .or(char('b').with(
                    many1(satisfy(|c| c == '0' || c == '1')).and_then(from_radix!(BigInt, 2)),
                )),
        )
        .map(Number::from_bigint)
        .parse_state(input)
}

pub fn decimal_number(input: &str) -> ParseResult<Number, &str> {
    use combine::{char, digit, many1, optional, Parser, ParserExt};
    many1(digit())
        .and(optional(char('.').with(many1(digit()))))
        .and(optional(char('e').or(char('E')).with(signed!(
            many1(digit()).and_then(from_radix!(i32, 10))
        ))))
        .and_then(
            |((mut digs, optdec), optexp): ((String, Option<String>), Option<i32>)| {
                use num::{BigInt, Num};

                let decplaces = match optdec {
                    None => 0i32,

                    Some(dec) => {
                        digs.push_str(dec.as_str());
                        -(dec.len() as i32)
                    }
                };
                let exp = optexp.unwrap_or(0);
                let places = decplaces + exp;

                BigInt::from_str_radix(digs.as_str(), 10).map(|i| Number::new(i, places))
            },
        )
        .parse_state(input)
}

#[cfg(test)]
mod tests {
    use super::number;
    use value::Number;

    #[test]
    fn reject() {
        use combine::{eof, parser, Parser, ParserExt};

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
                            let ina = inputs[i];
                            let inb = inputs[j];
                            let a = test_parse_number(ina);
                            let b = test_parse_number(inb);
                            assert!(
                                a == b,
                                "Equality failed for: {:?} parsed from {:?} \
                                 != {:?} parsed from {:?}",
                                a, ina, b, inb);
                        }
                    }
                }
            )*
        }
    }

    test_cases_number_parser!(zero, one);

    fn test_parse_number(input: &str) -> Number {
        use combine::{eof, parser, Parser, ParserExt};

        let res = parser(number).skip(eof()).parse(input);
        assert!(res.is_ok(), "Failed to parse: {:?}", input);
        let (x, rem) = res.unwrap();
        assert_eq!(rem, "");

        x
    }
}
