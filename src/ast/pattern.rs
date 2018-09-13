use ast::Identifier;

#[derive(Clone, Debug)]
pub enum Pattern {
    Bind(Identifier),
}

impl<'a> IntoIterator for &'a Pattern {
    type Item = &'a Identifier;
    type IntoIter = PatternIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PatternIterator(Some(self))
    }
}

pub struct PatternIterator<'a>(Option<&'a Pattern>);

impl<'a> Iterator for PatternIterator<'a> {
    type Item = &'a Identifier;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.0 {
            Some(&Pattern::Bind(ref id)) => {
                self.0 = None;
                Some(id)
            }

            None => None,
        };

        result
    }
}
