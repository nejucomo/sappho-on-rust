macro_rules! def_parser {
    ($maker:ident, $parsestruct:ident, $result:ident, $parse:expr) => {
        pub fn $maker<'a, F, T>(f: &'static F) -> $parsestruct<'a, F, T>
        where
            F: Clone + Fn(&'a str) -> ParseResult<T, &'a str>,
        {
            $parsestruct {
                f: f,
                _marker: PhantomData,
            }
        }

        #[derive(Debug, Clone)]
        struct $parsestruct<'a, F: 'static, T>
        where
            F: Clone + Fn(&'a str) -> ParseResult<T, &'a str>,
        {
            f: &'static F,
            _marker: PhantomData<&'a ()>,
        }

        impl<'a, F, T> Parser for $parsestruct<'a, F, T>
        where
            F: Clone + Fn(&'a str) -> ParseResult<T, &'a str>,
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

macro_rules! def_top_parser {
    ($maker:ident, $parsestruct:ident, $parse:expr) => {
        pub fn $maker<'a, F, W, T>(f: &'static F, wrap: &'static W) -> $parsestruct<'a, F, W, T>
        where
            F: Clone + Fn(&'a str) -> ParseResult<T, &'a str>,
            W: Clone + Fn(GenExpr<T>) -> T,
            T: Clone,
        {
            $parsestruct {
                f: f,
                wrap: wrap,
                _marker: PhantomData,
            }
        }

        #[derive(Debug, Clone)]
        struct $parsestruct<'a, F: 'static, W: 'static, T>
        where
            F: Clone + Fn(&'a str) -> ParseResult<T, &'a str>,
            W: Clone + Fn(GenExpr<T>) -> T,
            T: Clone,
        {
            f: &'static F,
            wrap: &'static W,
            _marker: PhantomData<&'a ()>,
        }

        impl<'a, F, W, T> Parser for $parsestruct<'a, F, W, T>
        where
            F: Clone + Fn(&'a str) -> ParseResult<T, &'a str>,
            W: Clone + Fn(GenExpr<T>) -> T,
            T: Clone,
        {
            type Input = &'a str;
            type Output = T;

            fn parse_stream(
                &mut self,
                input: Self::Input,
            ) -> ParseResult<Self::Output, Self::Input> {
                ($parse)(self.f, self.wrap).parse_stream(input)
            }
        }
    };
}
