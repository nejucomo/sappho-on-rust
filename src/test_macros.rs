macro_rules! include_without_newline {
    ($p:expr) => {
        {
            let src = include_str!($p);
            if src.len() == 0 {
                &""
            } else {
                assert_eq!('\n', src.chars().rev().next().unwrap());
                &src[0..src.len()-1]
            }
        }
    }
}

macro_rules! include_cases {
    ($p:expr) => {
        {
            let src = include_without_newline!($p);
            src.split("\n")
        }
    }
}

macro_rules! include_parser_test_vector {
    ($name:expr, accept) => {
        include_cases!(concat!("test-vectors/", stringify!($name), ".accept"));
    };

    ($name:expr, reject) => {
        include_cases!(concat!("test-vectors/", stringify!($name), ".reject"));
    }
}

macro_rules! test_case_simple_parser {
    ($test_name:ident, $name:ident, $make_result:expr) => {
        #[test]
        fn $test_name() {
            use combine::{Parser, ParserExt, eof, parser};

            for s in include_parser_test_vector!($name, accept) {
                assert_eq!(
                    parser($name).skip(eof()).parse(s),
                    Ok(($make_result(s), "")));
            }

            for s in include_parser_test_vector!($name, reject) {
                assert!(
                    parser($name).skip(eof()).parse(s).is_err(),
                    "invalidly parsed {:?} as {}",
                    s,
                    stringify!($name));
            }
        }
    }
}

macro_rules! include_io {
    ($name:ident, $case_name:expr, $io:expr) => {
        include_without_newline!(
            concat!(
                "test-vectors/",
                stringify!($name),
                "/case.",
                $case_name,
                ".",
                $io));
    }
}

macro_rules! test_case_string_parser {
    ($test_name:ident, $name:ident, $case_name:expr) => {
        #[test]
        fn $test_name() {
            use combine::{Parser, ParserExt, eof, parser};

            let input  = include_io!($name, $case_name, "input");
            let output = include_io!($name, $case_name, "output");

            assert_eq!(
                parser($name)
                    .skip(eof())
                    .parse(input)
                    .map(|(res, rem)| (res.to_string(), rem)),
                Ok((output.to_string(), "")));
        }
    }
}

