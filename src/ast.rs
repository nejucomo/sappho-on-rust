use value::{Atom, Symbol};

#[derive(Clone, Debug)]
pub struct Identifier(pub String);

#[derive(Clone, Debug)]
pub struct FuncExpr(pub GenExpr<FuncExpr>);

/*
#[derive(Clone, Debug)]
pub enum QueryExpr {
    GenExpr(GenExpr<QueryExpr>),
    Query(Box<QueryExpr>),
}
*/

#[derive(Clone, Debug)]
pub enum ProcExpr {
    GenExpr(GenExpr<SteppingStoneProcExpr>),
    Query(Box<SteppingStoneProcExpr>),
    Mutate(Box<SteppingStoneProcExpr>),
}

/* SteppingStoneProcExpr is a placeholder type we will remove later.
 *
 * We haven't updated the parser with the refinement, so every expression
 * parses into this type. We introduce this to distinguish where we *should*
 * parse GenExpr versus where we really should parse ProcExpr.
 */
#[derive(Clone, Debug)]
pub struct SteppingStoneProcExpr(pub ProcExpr);

#[derive(Clone, Debug)]
pub enum GenExpr<X>
where
    X: Clone,
{
    Atom(Atom),
    Deref(Identifier),
    List(Vec<X>),
    LookupApp(Box<X>, Symbol),
    FuncApp(Box<X>, Box<X>),
    // UnApp(UnaryOperator, Box<X>),
    BinOp(BinaryOperator, Box<X>, Box<X>),
    Lambda(LambdaDefinition),
}

/*
#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Invert,
}
*/

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Times,
}

#[derive(Clone, Debug)]
pub struct LambdaDefinition {
    func: Option<FunctionDefinition>,
    query: Option<QueryDefinition>,
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition(pub Identifier, pub Box<SteppingStoneProcExpr>);

impl From<FunctionDefinition> for LambdaDefinition {
    fn from(fd: FunctionDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: Some(fd),
            query: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueryDefinition(pub Box<SteppingStoneProcExpr>);

impl From<QueryDefinition> for LambdaDefinition {
    fn from(qd: QueryDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: None,
            query: Some(qd),
        }
    }
}
