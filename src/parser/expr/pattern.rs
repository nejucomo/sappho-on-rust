use ast::Pattern;
use combine::Parser;

pub fn pattern<'a>() -> impl Clone + Parser<Output = Pattern, Input = &'a str> {
    use parser::atom::identifier;

    identifier().map(Pattern::Bind)
}
