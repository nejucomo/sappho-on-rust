use combine::ParseResult;
use value::Number;

macro_rules! from_radix {
    ($t:ty, $radix:expr) => {
        |s: String| <$t>::from_str_radix(s.as_str(), $radix)
    };
}

macro_rules! signed {
    ($p:expr) => {{
        use combine::char::char;

        char('+').with($p).or(char('-').with($p).map(|n| -n)).or($p)
    }};
}

pub fn number(input: &str) -> ParseResult<Number, &str> {
    use combine::{parser, Parser};

    signed!(parser(signless_number)).parse_stream(input)
}

pub fn signless_number(input: &str) -> ParseResult<Number, &str> {
    use combine::{parser, try, Parser};

    try(parser(zero_or_hexbin_number))
        .or(parser(decimal_number))
        .parse_stream(input)
}

pub fn zero_or_hexbin_number(input: &str) -> ParseResult<Number, &str> {
    use combine::char::{char, hex_digit};
    use combine::{many1, satisfy, Parser};
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
        .parse_stream(input)
}

pub fn decimal_number(input: &str) -> ParseResult<Number, &str> {
    use combine::char::{char, digit};
    use combine::{many1, optional, try, Parser};

    many1(digit())
        .and(optional(try(char('.').with(many1(digit())))))
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
        .parse_stream(input)
}

#[cfg(tests)]
parser_tests_mod!(
    tests,
    number,
    include_dir!("src/parser/test-vectors/number/")
);
