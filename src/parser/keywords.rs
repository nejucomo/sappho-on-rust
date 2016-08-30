use combine::Parser;


pub const KEYWORDS: [&'static str; 8] = [
    "𝜆",
    "\\",
    "proc",
    "query",
    "let",
    "in",
    "from",
    "bind",
    ];


pub fn keyword(kw: &'static str) -> Box<Parser<Input=&str, Output=()>>
{
    use combine::{ParserExt, string, value};

    assert!(KEYWORDS.contains(&kw));

    Box::new(string(kw).with(value(())))
}


#[cfg(test)]
mod tests {
    use super::keyword;

    #[test]
    fn test_identifier() {
        use combine::{Parser, ParserExt, eof};

        macro_rules! include_cases {
            ($p:expr) => {
                {
                    let src = include_str!($p);
                    assert_eq!('\n', src.chars().rev().next().unwrap());
                    src[0..src.len()-1].split("\n")
                }
            }
        }

        // let parse_only = |p, s| p.skip(eof()).parse(s);

        for s in include_cases!("test-vectors/keyword.accept") {
            assert_eq!(
                keyword(s).skip(eof()).parse(s),
                Ok(((), "")));
        }

        for s in include_cases!("test-vectors/keyword.reject") {
            assert!(
                // parse_only(keyword("proc"), s).is_err(),
                keyword("proc").skip(eof()).parse(s).is_err(),
                "invalidly parsed {:?} as keyword",
                s);
        }
    }
}
