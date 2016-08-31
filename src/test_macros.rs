macro_rules! include_cases {
    ($p:expr) => {
        {
            let src = include_str!($p);
            assert_eq!('\n', src.chars().rev().next().unwrap());
            src[0..src.len()-1].split("\n")
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

