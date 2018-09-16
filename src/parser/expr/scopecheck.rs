use ast::{Expr, Identifier};
use combine::{ParseError, Parser, StreamOnce};
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn deref<'a, OP>(sc: ScopeCheck) -> impl Clone + Parser<Output = Expr<OP>, Input = &'a str> {
    use combine::position;
    use parser::atom::identifier;

    position()
        .and(identifier())
        .flat_map(move |(pos, id)| {
            if id.starts_with("_") {
                Err(make_parse_error_builder(pos)(format!(
                    "Disallowed Underscore Derefence: {:?}",
                    id
                )))
            } else {
                sc.mark_deref(&id)
                    .map_err(make_parse_error_builder(pos))
                    .map(|_| id)
            }
        })
        .map(Expr::Deref)
}

#[derive(Clone)]
pub struct ScopeCheck(Rc<Layer>);

#[derive(Clone)]
enum Layer {
    Empty,
    MapLink(Rc<Layer>, HashMap<Identifier, Cell<bool>>),
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
            map.insert(Identifier::from(id), Cell::new(false));
        }

        ScopeCheck(Rc::new(Layer::MapLink(self.0.clone(), map)))
    }

    pub fn check_unused<'a, T>(
        self,
        pos: <(&str) as StreamOnce>::Position,
        thing: T,
    ) -> Result<T, ParseError<&'a str>> {
        self.0
            .check_unused()
            .map(|()| thing)
            .map_err(make_parse_error_builder(pos))
    }

    // FIXME: This does not mark the binding as referenced yet; requires mut.
    fn mark_deref(&self, id: &Identifier) -> Result<(), String> {
        self.0.mark_deref(id)
    }
}

impl Layer {
    fn mark_deref(&self, id: &Identifier) -> Result<(), String> {
        match self {
            &Layer::Empty => Err(format!("Undefined reference: {:?}", id)),
            &Layer::MapLink(ref supref, ref map) => match map.get(id) {
                None => supref.mark_deref(id),
                Some(cell) => {
                    cell.set(true);
                    Ok(())
                }
            },
        }
    }

    fn check_unused(&self) -> Result<(), String> {
        if let &Layer::MapLink(_, ref map) = self {
            let mut prefix = "Unused bindings: ";
            let mut errstr = String::new();

            for (id, usecell) in map {
                if !(id.starts_with("_") || usecell.get()) {
                    errstr.push_str(prefix);
                    errstr.push_str(&id);
                    prefix = ", ";
                }
            }

            if errstr.len() > 0 {
                return Err(errstr);
            }
        }

        return Ok(());
    }
}

fn make_parse_error_builder<S>(pos: S::Position) -> impl FnOnce(String) -> ParseError<S>
where
    S: StreamOnce,
{
    use combine::primitives::{Error, Info};
    use combine::ParseError;

    |e| ParseError::new(pos, Error::Message(Info::Owned(e)))
}
