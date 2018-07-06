use combine::{ParseError, Parser};
use include_dir::Dir;
use std::fmt::Debug;

pub fn run_parser_repr_tests<'a, F, P, O>(makeparser: F, vecdir: Dir<'a>)
where
    F: Fn() -> P,
    P: Parser<Input = &'a str, Output = O>,
    O: Debug,
{
    use std::fmt::Write;
    let mut failures = String::new();

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
                let actualres = parse_input(makeparser(), input);

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
                    "*** Case {:?} {:?} ***\n{}\n",
                    casedir.path().file_name().unwrap(),
                    inentry.path().file_name().unwrap(),
                    casefailures
                ).unwrap();
            }
        }
    }

    assert_eq!(0, failures.len(), "\n\n{}", failures);
}

pub fn run_parser_reject_tests<'a, F, P, O>(makeparser: F, input: &'a str)
where
    F: Fn() -> P,
    P: Parser<Input = &'a str, Output = O>,
    O: Debug,
{
    for line in input.trim_right().split("\n") {
        let res = parse_input(makeparser(), line);
        assert!(res.is_err(), "Invalidly parsed {:?} to {:?}", line, res);
    }
}

fn parse_input<'a, P, O>(p: P, input: &'a str) -> Result<(O, &'a str), ParseError<&'a str>>
where
    P: Parser<Input = &'a str, Output = O>,
{
    use combine::char::spaces;
    use combine::{eof, Parser};

    p.skip(spaces()).skip(eof()).parse(input)
}
