use combine::{ParseError, Parser};
use include_dir::Dir;
use std::fmt::{Debug, Error, Write};

macro_rules! log_failure {
    ( $caselog:expr, $( $args:expr ),* ) => {
        write!($caselog, $( $args ),* ).unwrap();
    }
}

pub fn run_parser_repr_tests<'a, F, P, O>(makeparser: F, vecdir: Dir<'a>)
where
    F: Fn() -> P,
    P: Parser<Input = &'a str, Output = O>,
    O: Debug,
{
    use std::fmt::Write;
    let mut flog = FailureLog::new();

    for casedir in vecdir.dirs() {
        let mut reprbuf = casedir.path().to_path_buf();
        reprbuf.push("repr");

        let reprf = casedir.get_file(reprbuf).unwrap();
        let rawexp = reprf.contents_utf8().unwrap().to_string();
        let expected = rawexp.trim_right();

        for inentry in casedir.files() {
            let mut caselog = flog.subcase_log(&inentry.path().file_name().unwrap());

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
                        log_failure!(&mut caselog, "Unparsed input: {:?}\n", rem);
                    }
                    let actual = format!("{:?}", actualobj);
                    if actual != expected {
                        log_failure!(
                            caselog,
                            "mismatch:\nexpected: {:?}\nactual  : {:?}\n",
                            expected,
                            actual
                        );
                    }
                } else {
                    log_failure!(caselog, "Parse failure: {:?}\n", actualres);
                }
            }

            caselog.finish();
        }
    }

    assert_eq!(0, flog.0.len(), "\n\n{}", flog.0);
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

struct FailureLog(String);

impl FailureLog {
    fn new() -> FailureLog {
        FailureLog(String::new())
    }

    fn subcase_log<'a, D: Debug>(&'a mut self, casename: &D) -> SubcaseLog<'a> {
        SubcaseLog(self, false, format!("*** Case {:?} ***\n", casename))
    }
}

struct SubcaseLog<'a>(&'a mut FailureLog, bool, String);

impl<'a> SubcaseLog<'a> {
    fn finish(self) {
        let SubcaseLog(flref, dirty, body) = self;

        if dirty {
            write!(flref.0, "{}", body).unwrap();
        }
    }
}

impl<'a> Write for SubcaseLog<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.1 = true;
        self.2.write_str(s)
    }
}
