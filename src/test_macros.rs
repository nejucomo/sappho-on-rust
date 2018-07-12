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
