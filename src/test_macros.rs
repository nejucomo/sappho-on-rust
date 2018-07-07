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
    ($test_dir:expr, $name:expr,accept) => {
        include_cases!(concat!(
            "test-vectors/",
            $test_dir,
            "/",
            stringify!($name),
            ".accept"
        ));
    };

    ($test_dir:expr, $name:expr,reject) => {
        include_cases!(concat!(
            "test-vectors/",
            $test_dir,
            "/",
            stringify!($name),
            ".reject"
        ));
    };
}

macro_rules! test_case_simple_parser {
    ($name:ident, $test_dir:expr, $test_name:ident, $make_result:expr) => {
        #[test]
        fn $test_name() {
            use combine::{eof, parser, Parser};

            for s in include_parser_test_vector!($test_dir, $name, accept) {
                assert_eq!(
                    parser($name).skip(eof()).parse(s),
                    Ok(($make_result(s), ""))
                );
            }

            for s in include_parser_test_vector!($test_dir, $name, reject) {
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
