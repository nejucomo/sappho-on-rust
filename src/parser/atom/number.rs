use combine::Parser;
use num::{BigInt, Num};
use std::ops::Neg;
use value::Number;

pub fn number<'a>() -> impl Parser<Output = Number, Input = &'a str> {
    signed(signless_number())
}

fn signed<'a, P, O>(p: P) -> impl Clone + Parser<Output = O, Input = &'a str>
where
    P: Clone + Parser<Output = O, Input = &'a str>,
    O: Neg<Output = O>,
{
    use combine::char::char;

    char('+')
        .with(p.clone())
        .or(char('-').with(p.clone()).map(|n| -n))
        .or(p)
}

fn signless_number<'a>() -> impl Clone + Parser<Output = Number, Input = &'a str> {
    use combine::{try, Parser};

    try(zero_or_hexbin_number()).or(decimal_number())
}

fn zero_or_hexbin_number<'a>() -> impl Clone + Parser<Output = Number, Input = &'a str> {
    use combine::char::{char, hex_digit};
    use combine::{many1, satisfy, Parser};

    char('0')
        .with(
            char('x')
                .with(from_radix(16, many1(hex_digit())))
                .or(char('b').with(from_radix(2, many1(satisfy(|c| c == '0' || c == '1'))))),
        )
        .map(Number::from_bigint)
}

fn decimal_number<'a>() -> impl Clone + Parser<Output = Number, Input = &'a str> {
    use combine::char::{char, digit};
    use combine::{many1, optional, try, Parser};

    many1(digit())
        .and(optional(try(char('.').with(many1(digit())))))
        .and(optional(
            char('e')
                .or(char('E'))
                .with(signed(from_radix(10, many1(digit())))),
        ))
        .and_then(
            |((mut digs, optdec), optexp): ((String, Option<String>), Option<BigInt>)| {
                use num::{BigInt, Num, ToPrimitive};

                let decplaces = match optdec {
                    None => 0i32,

                    Some(dec) => {
                        digs.push_str(dec.as_str());
                        -(dec.len() as i32)
                    }
                };
                let exp = optexp.and_then(|biref| biref.to_i32()).unwrap_or(0);
                let places = decplaces + exp;

                BigInt::from_str_radix(digs.as_str(), 10).map(|i| Number::new(i, places))
            },
        )
}

fn from_radix<'a, P>(radix: u32, p: P) -> impl Clone + Parser<Output = BigInt, Input = &'a str>
where
    P: Clone + Parser<Output = String, Input = &'a str>,
{
    p.and_then(move |s: String| <BigInt>::from_str_radix(s.as_str(), radix))
}

#[cfg(tests)]
parser_tests_mod!(
    tests,
    number,
    include_dir!("src/parser/test-vectors/number/")
);
