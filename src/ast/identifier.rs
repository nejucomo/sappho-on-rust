use std::ops::Deref;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Identifier(pub String); // FIXME: Remove field pub.

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &str {
        self.0.deref()
    }
}

impl<'a> From<&'a Identifier> for Identifier {
    fn from(src: &'a Identifier) -> Identifier {
        src.clone()
    }
}

impl<T> From<T> for Identifier
where
    String: From<T>,
{
    fn from(src: T) -> Identifier {
        Identifier(String::from(src))
    }
}
