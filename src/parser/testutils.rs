use combine::{ParseError, Parser};
use include_dir::{Dir, File};
use std::fmt::{Debug, Error, Write};

#[macro_export]
macro_rules! parser_tests_mod {
    ($modname:ident, $parserfn:expr, $incdir:expr) => {
        #[cfg(test)]
        mod $modname {
            use parser;

            parser_accept_reject_tests!($parserfn, $incdir);
        }
    };
}

#[macro_export]
macro_rules! parser_accept_reject_tests {
    ($parserfn:expr, $incdir:expr) => {
        #[test]
        fn accepts() {
            use parser::testutils::run_parser_repr_tests;

            run_parser_repr_tests($parserfn, $incdir);
        }

        #[test]
        fn rejects() {
            use parser::testutils::run_parser_reject_tests;

            run_parser_reject_tests($parserfn, $incdir);
        }
    };
}

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

    for casedir in check_nonempty(&vecdir, vecdir.dirs(), "cases") {
        if casedir.path().file_name().unwrap() == "rejects" {
            continue;
        }

        let mut reprbuf = casedir.path().to_path_buf();
        reprbuf.push("repr");

        let reprf = casedir.get_file(reprbuf).unwrap();
        let rawexp = reprf.contents_utf8().unwrap().to_string();
        let expected = rawexp.trim_right();

        for inentry in check_nonempty(&vecdir, casedir.files(), "case inputs") {
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
                let mut caselog = flog.subcase_log(inentry, input);
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

pub fn run_parser_reject_tests<'a, F, P, O>(makeparser: F, vecdir: Dir<'a>)
where
    F: Fn() -> P,
    P: Parser<Input = &'a str, Output = O>,
    O: Debug,
{
    let mut flog = FailureLog::new();

    let rejectsdir = vecdir
        .dirs()
        .into_iter()
        .find(|d| d.path().file_name().unwrap() == "rejects")
        .expect(&format!(
            "Could not find \"rejects\" dir within {:?}",
            vecdir
        ));

    for inentry in check_nonempty(rejectsdir, rejectsdir.files(), "reject inputs") {
        let input = inentry.contents_utf8().unwrap();
        let mut caselog = flog.subcase_log(inentry, input);
        let res = parse_input(makeparser(), input);
        if res.is_ok() {
            log_failure!(caselog, "Invalidly parsed to {:?}\n", res);
        }
    }
}

fn check_nonempty<'a, T>(vd: &'a Dir<'a>, xs: &'a [T], label: &'static str) -> &'a [T] {
    assert!(xs.len() > 0, "Missing {} in {:?}.", label, vd.path());
    xs
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

    fn subcase_log<'a>(&'a mut self, file: &'a File<'a>, casename: &str) -> SubcaseLog<'a> {
        SubcaseLog(
            self,
            false,
            format!("*** Case {:?} {:?}\n", file.path(), casename),
        )
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
