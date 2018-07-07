macro_rules! include_without_newline {
    ($p:expr) => {{
        let src = include_str!($p);
        if src.len() == 0 {
            &""
        } else {
            assert_eq!('\n', src.chars().rev().next().unwrap());
            &src[0..src.len() - 1]
        }
    }};
}

macro_rules! include_cases {
    ($p:expr) => {{
        let src = include_without_newline!($p);
        src.split("\n")
    }};
}

macro_rules! include_parser_test_vector {
    ($name:expr,accept) => {
        include_cases!(concat!("test-vectors/", stringify!($name), ".accept"));
    };

    ($name:expr,reject) => {
        include_cases!(concat!("test-vectors/", stringify!($name), ".reject"));
    };
}

macro_rules! test_case_simple_parser {
    ($name:ident, $test_name:ident, $make_result:expr) => {
        #[test]
        fn $test_name() {
            use combine::{eof, parser, Parser};

            for s in include_parser_test_vector!($name, accept) {
                assert_eq!(
                    parser($name).skip(eof()).parse(s),
                    Ok(($make_result(s), ""))
                );
            }

            for s in include_parser_test_vector!($name, reject) {
                assert!(
                    parser($name).skip(eof()).parse(s).is_err(),
                    "invalidly parsed {:?} as {}",
                    s,
                    stringify!($name)
                );
            }
        }
    };
}

macro_rules! include_io {
    ($name:ident, $case_name:expr, $io:expr) => {
        include_without_newline!(concat!(
            "test-vectors/",
            stringify!($name),
            "/case.",
            $case_name,
            ".",
            $io
        ));
    };
}

macro_rules! test_case_text_parser {
    ($name:ident, $test_name:ident, $case_name:expr) => {
        #[test]
        fn $test_name() {
            use combine::{eof, parser, Parser};

            let inputsq = include_io!($name, $case_name, "input.sq");
            let inputdq = include_io!($name, $case_name, "input.dq");
            let output = include_io!($name, $case_name, "output");

            for input in &[inputsq, inputdq] {
                assert_eq!(
                    parser($name)
                        .skip(eof())
                        .parse(input)
                        .map(|(res, rem)| (res.to_string(), rem)),
                    Ok((output.to_string(), ""))
                );
            }
        }
    };
}

macro_rules! test_cases_text_parser {
    ($name:ident, [ $( ( $test_name:ident, $case_name:expr ) ),* ] ) => {
        $(
            test_case_text_parser!($name, $test_name, $case_name);
        )*
    }
}
