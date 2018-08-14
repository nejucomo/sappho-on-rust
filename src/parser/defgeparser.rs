macro_rules! def_parser {
    ($maker:ident, $parsestruct:ident, $result:ident, $parse:expr) => {
        pub fn $maker<'a, F, T>(f: &'static F) -> $parsestruct<'a, F, T>
        where
            F: Fn(&'a str) -> ParseResult<T, &'a str>,
        {
            $parsestruct {
                f: f,
                _marker: PhantomData,
            }
        }

        #[derive(Debug, Clone)]
        struct $parsestruct<'a, F: 'static, T>
        where
            F: Fn(&'a str) -> ParseResult<T, &'a str>,
        {
            f: &'static F,
            _marker: PhantomData<&'a ()>,
        }

        impl<'a, F, T> Parser for $parsestruct<'a, F, T>
        where
            F: Fn(&'a str) -> ParseResult<T, &'a str>,
            T: Clone,
        {
            type Input = &'a str;
            type Output = $result<T>;

            fn parse_stream(
                &mut self,
                input: Self::Input,
            ) -> ParseResult<Self::Output, Self::Input> {
                ($parse)(self.f).parse_stream(input)
            }
        }
    };
}

macro_rules! def_ge_parser {
    ($maker:ident, $parsestruct:ident, $parse:expr) => {
        def_parser!($maker, $parsestruct, GenExpr, $parse);
    };
}
