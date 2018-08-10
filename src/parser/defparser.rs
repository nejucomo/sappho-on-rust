macro_rules! def_ge_parser {
    ($makername:ident, $structname:ident, $parse:expr) => {
        pub fn $makername<'a, F>(f: &'static F) -> $structname<'a, F> {
            $structname {
                f: f,
                _phantom: PhantomData,
            }
        }

        pub struct $structname<'a, F: 'static> {
            f: &'static F,
            _phantom: PhantomData<&'a str>,
        }

        impl<'a, F, T> Parser for $structname<'a, F>
        where
            F: 'static + Fn(&str) -> ParseResult<T, &str>,
        {
            type Input = &'a str;
            type Output = T;

            fn parse_stream(
                &mut self,
                input: Self::Input,
            ) -> ParseResult<Self::Output, Self::Input> {
                ($parse)(self.f).parse_stream(input)
            }
        }
    };
}
