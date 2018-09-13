use ast::{Expr, Identifier};
use combine::Parser;
use std::collections::HashMap;
use std::rc::Rc;

pub fn deref<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str> {
    use combine::primitives::{Error, Info};
    use combine::{position, ParseError};
    use parser::atom::identifier;

    position()
        .and(identifier())
        .flat_map(move |(pos, id)| {
            sc.check_deref(&id)
                .map_err(|e| ParseError::new(pos, Error::Message(Info::Owned(e))))
                .map(|_| id)
        })
        .map(Expr::Deref)
}

#[derive(Clone)]
pub struct ScopeCheck(Rc<Layer>);

#[derive(Clone)]
enum Layer {
    Empty,
    MapLink(Rc<Layer>, HashMap<Identifier, bool>),
}

impl ScopeCheck {
    pub fn empty() -> ScopeCheck {
        ScopeCheck(Rc::new(Layer::Empty))
    }

    pub fn with_implicit_bindings<'a, I, S>(bindings: I) -> ScopeCheck
    where
        I: IntoIterator<Item = S>,
        Identifier: From<S>,
    {
        ScopeCheck::empty().push(bindings)
    }

    pub fn push<'a, I, S>(self, bindings: I) -> ScopeCheck
    where
        I: IntoIterator<Item = S>,
        Identifier: From<S>,
    {
        let mut map = HashMap::new();

        for id in bindings {
            map.insert(Identifier::from(id), false);
        }

        ScopeCheck(Rc::new(Layer::MapLink(self.0.clone(), map)))
    }

    // FIXME: This does not mark the binding as referenced yet; requires mut.
    fn check_deref(&self, id: &Identifier) -> Result<(), String> {
        self.0.check_deref(id)
    }
}

impl Layer {
    fn check_deref(&self, id: &Identifier) -> Result<(), String> {
        match self {
            &Layer::Empty => Err(format!("Undefined reference: {:?}", id)),
            &Layer::MapLink(ref supref, ref map) => match map.get(id) {
                None => supref.check_deref(id),
                Some(_) => Ok(()),
            },
        }
    }
}
