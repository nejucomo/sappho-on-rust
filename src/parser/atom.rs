use combine::{ParseResult, Parser};
use value::Atom;

pub fn atom(input: &str) -> ParseResult<Atom, &str> {
    use combine::parser;
    use parser::{boolean, character, number, symbol, text};

    (parser(boolean).map(Atom::Bool))
        .or(parser(number).map(Atom::Number))
        .or(parser(character).map(Atom::Char))
        .or(parser(text).map(Atom::Text))
        .or(parser(symbol).map(Atom::Symbol))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::atom;

    #[test]
    fn accepts() {
        use std::fmt::Write;
        let mut failures = String::new();

        let vecdir = include_dir!("src/parser/test-vectors/atom/");

        for casedir in vecdir.dirs() {
            let mut reprbuf = casedir.path().to_path_buf();
            reprbuf.push("repr");

            let reprf = casedir.get_file(reprbuf).unwrap();
            let rawexp = reprf.contents_utf8().unwrap().to_string();
            let expected = rawexp.trim_right();

            for inentry in casedir.files() {
                let mut casefailures = String::new();

                let stem = {
                    use std::os::unix::ffi::OsStrExt;
                    use std::str::from_utf8;

                    from_utf8(inentry.path().file_name().unwrap().as_bytes())
                        .unwrap()
                        .split(|cp| cp == '.')
                        .next()
                        .unwrap()
                };

                match stem {
                    "input" => {}
                    "repr" => continue,
                    _ => assert!(false, "Bad filename: {:?}", inentry),
                }

                let input = inentry.contents_utf8().unwrap();

                {
                    use combine::char::spaces;
                    use combine::{eof, parser, Parser};

                    let actualres = parser(atom).skip(spaces()).skip(eof()).parse(input);

                    if actualres.is_ok() {
                        let (actualobj, rem) = actualres.unwrap();
                        if rem != "" {
                            write!(&mut casefailures, "Unparsed input: {:?}\n", rem).unwrap();
                        }
                        let actual = format!("{:?}", actualobj);
                        if actual != expected {
                            write!(
                                casefailures,
                                "mismatch:\nexpected: {:?}\nactual  : {:?}\n",
                                expected, actual
                            ).unwrap();
                        }
                    } else {
                        write!(casefailures, "Parse failure: {:?}\n", actualres).unwrap();
                    }
                }

                if casefailures.len() > 0 {
                    write!(
                        failures,
                        "*** Case {:?} ***\n{}\n",
                        casedir.path().file_name().unwrap(),
                        casefailures
                    ).unwrap();
                }
            }
        }

        assert_eq!(0, failures.len(), "\n\n{}", failures);
    }
}
