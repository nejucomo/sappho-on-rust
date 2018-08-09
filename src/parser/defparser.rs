macro_rules! def_ge_parser {
    ($parsename:ident, $parse:expr) => {
        pub struct $parsename<P>(pub P);

        impl<'a, P, T> Parser for $parsename<P>
        where
            P: Parser<Input = &'a str, Output = T>,
            T: Clone,
        {
            type Input = &'a str;
            type Output = GenExpr<T>;

            fn parse_stream(
                &mut self,
                input: Self::Input,
            ) -> ParseResult<Self::Output, Self::Input> {
                ($parse)(self.0).parse_stream(input)
            }
        }
    };
}
