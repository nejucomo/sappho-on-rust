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

    fn check_nonempty<'a, T>(vd: Dir<'a>, xs: &'a [T], label: &'static str) -> &'a [T] {
        assert!(xs.len() > 0, "Missing {} in {:?}.", label, vd.path());
        xs
    }

    for casedir in check_nonempty(vecdir, vecdir.dirs(), "cases") {
        let mut reprbuf = casedir.path().to_path_buf();
        reprbuf.push("repr");

        let reprf = casedir.get_file(reprbuf).unwrap();
        let rawexp = reprf.contents_utf8().unwrap().to_string();
        let expected = rawexp.trim_right();

        for inentry in check_nonempty(vecdir, casedir.files(), "case inputs") {
            let mut caselog = flog.subcase_log(&format!(
                "{:?} {:?}",
                casedir.path().file_name().unwrap(),
                &inentry.path().file_name().unwrap()
            ));

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

                match actualres {
                    Ok((actualobj, rem)) => {
                        if rem != "" {
                            log_failure!(caselog, "Unparsed input: {:?}\n", rem);
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
                    }
                    Err(e) => {
                        log_failure!(caselog, "{}\nFor input {:?}\n", e, input);
                    }
                }
            }
        }
    }
}

pub fn run_parser_reject_tests<'a, F, P, O>(makeparser: F, input: &'a str)
where
    F: Fn() -> P,
    P: Parser<Input = &'a str, Output = O>,
    O: Debug,
{
    let mut flog = FailureLog::new();

    for line in input.trim_right().split("\n") {
        let mut caselog = flog.subcase_log(&line);
        let res = parse_input(makeparser(), line);
        if res.is_ok() {
            log_failure!(caselog, "Invalidly parsed to {:?}\n", res);
        }
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

#[derive(Debug)]
struct FailureLog(String);

impl FailureLog {
    fn new() -> FailureLog {
        FailureLog(String::new())
    }

    fn subcase_log<'a>(&'a mut self, casename: &str) -> SubcaseLog<'a> {
        SubcaseLog(self, false, format!("*** Case {} ***\n", casename))
    }
}

impl Drop for FailureLog {
    fn drop(&mut self) {
        assert_eq!(0, self.0.len(), "\n\n{}\n", self.0);
    }
}

#[derive(Debug)]
struct SubcaseLog<'a>(&'a mut FailureLog, bool, String);

impl<'a> Drop for SubcaseLog<'a> {
    fn drop(&mut self) {
        let SubcaseLog(ref mut flref, ref dirty, ref body) = *self;

        if *dirty {
            write!(flref.0, "{}\n", body).unwrap();
        }
    }
}

impl<'a> Write for SubcaseLog<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.1 = true;
        self.2.write_str(s)
    }
}
