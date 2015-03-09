use std::iter::FromIterator;
use std::collections::HashMap;

use super::{
    Application,
    Callable,
    Expression,
    Function,
    Identifier,
    Let,
    List,
    Object,
    ParseResult,
    PatternItem,
    Proc,
    PropItem,
    Properties,
    Query,
    StatementBlock,
    Uncallable,
};


pub fn verify_expression(x: Expression) -> ParseResult {
    x.verify(&Context::D).map_or(Ok(x), |errmsg| Err(errmsg))
}


// "Internal Verify Result":
type IVR = Option<String>;


#[derive(Eq)]
#[derive(PartialEq)]
enum Context {
    D, // Deterministic
    Q, // Query
    P, // Proc
}
use self::Context::{D,Q,P};


trait Verifiable {
    fn verify(&self, &Context) -> IVR;
}


// Short-circuit sequencing of verifications:
macro_rules! vseq {
    [ $( $ctx:expr => $x:expr ),+ ] => {{
        $({
            let ivr = $x.verify($ctx);
            if ivr.is_some() {
                return Some(ivr.unwrap())
            }
        })+

        None
    }}
}


impl Verifiable for Expression {
    fn verify(&self, ctx: &Context) -> IVR {
        match *self {
            Expression::Apps(ref a, ref b) => { vseq! [ ctx => a, ctx => b ] },
            Expression::Uncallable(ref a)  => { vseq! [ ctx => a           ] },
        }
    }
}

impl Verifiable for Callable {
    fn verify(&self, ctx: &Context) -> IVR {
        match *self {
            Callable::List(ref a)     => { vseq! [ ctx => a ] },
            Callable::Parens(ref a)   => { vseq! [ ctx => a ] },
            Callable::ProcApp(ref a)  => {
                match *ctx {
                    D => Some("! in deterministic context.".to_string()),
                    Q => Some("! in query context.".to_string()),
                    P => vseq! [ ctx => a ],
                }
            },
            Callable::QueryApp(ref a) => {
                match *ctx {
                    D => Some("$ in deterministic context.".to_string()),
                    _  => vseq! [ ctx => a ],
                }
            },

            // Dereference and Literal ok in any context:
            _ => None
        }
    }
}

impl Verifiable for Application {
    fn verify(&self, ctx: &Context) -> IVR {
        match *self {
            Application::Lookup(_)       => None,
            Application::Dispatch(ref a) => { vseq! [ ctx => a ] },
            Application::ListApp(ref a)  => { vseq! [ ctx => a ] },
        }
    }
}

impl Verifiable for Uncallable {
    fn verify(&self, ctx: &Context) -> IVR {
        match *self {
            Uncallable::Object(ref a) => { vseq! [ ctx => a ] },
            Uncallable::Let(ref a)    => { vseq! [ ctx => a ] },
        }
    }
}

impl Verifiable for Object {
    fn verify(&self, _: &Context) -> IVR {
        vseq! [
            &P => self.proc_,
            &Q => self.query,
            &D => self.func,
            &D => self.props
        ]
    }
}

impl Verifiable for Proc {
    fn verify(&self, ctx: &Context) -> IVR {
        assert!(ctx == &P);
        match *self {
            Proc(ref a) => a.verify(&P)
        }
    }
}

impl Verifiable for StatementBlock {
    fn verify(&self, ctx: &Context) -> IVR {
        assert!(ctx == &P);
        match *self {
            StatementBlock::Return(ref x) => x.verify(&P)
        }
    }
}

impl Verifiable for Query {
    fn verify(&self, ctx: &Context) -> IVR {
        assert!(ctx == &Q);
        match *self {
            Query(ref x) => x.verify(&Q)
        }
    }
}

impl Verifiable for Function {
    fn verify(&self, ctx: &Context) -> IVR {
        assert!(ctx == &D);
        match *self {
            Function(ref x) => x.verify(&D)
        }
    }
}

impl Verifiable for Properties {
    fn verify(&self, ctx: &Context) -> IVR {
        assert!(ctx == &D);
        vseq! [ &D => self.map, &D => self.varprop ]
    }
}

impl Verifiable for HashMap<Identifier, Box<Expression>> {
    fn verify(&self, ctx: &Context) -> IVR {
        assert!(ctx == &D);
        let values : Vec<&Box<Expression>> =
            FromIterator::from_iter(self.values());

        values.verify(&D)
    }
}

impl Verifiable for PropItem {
    fn verify(&self, ctx: &Context) -> IVR {
        assert!(ctx == &D);
        match *self {
            (_, ref a) => a.verify(&D)
        }
    }
}

impl Verifiable for List {
    fn verify(&self, ctx: &Context) -> IVR {
        match *self {
            List(ref a) => a.verify(ctx)
        }
    }
}

impl Verifiable for Let {
    fn verify(&self, ctx: &Context) -> IVR {
        vseq! [ ctx => self.bindings, ctx => self.expr ]
    }
}

impl Verifiable for PatternItem {
    fn verify(&self, ctx: &Context) -> IVR {
        self.expr.verify(ctx)
    }
}


// Higher-order impls:
impl<'a, T: Verifiable> Verifiable for &'a T {
    fn verify(&self, ctx: &Context) -> IVR {
        (*self).verify(ctx)
    }
}

impl<T: Verifiable> Verifiable for Option<T> {
    fn verify(&self, ctx: &Context) -> IVR {
        match *self {
            None => None,
            Some(ref x) => x.verify(ctx),
        }
    }
}

impl<T: Verifiable> Verifiable for Box<T> {
    fn verify(&self, ctx: &Context) -> IVR {
        (**self).verify(ctx)
    }
}

impl<T: Verifiable> Verifiable for Vec<T> {
    fn verify(&self, ctx: &Context) -> IVR {
        for x in self {
            match x.verify(ctx) {
                Some(errmsg) => { return Some(errmsg) }
                None         => {}
            }
        }
        return None
    }
}
