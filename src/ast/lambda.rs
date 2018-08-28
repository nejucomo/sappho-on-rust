use ast::{FuncExpr, Pattern, ProcExpr, QueryExpr};

#[derive(Clone, Debug)]
pub struct LambdaDefinition {
    pub func: Option<FunctionDefinition>,
    pub query: Option<QueryDefinition>,
    pub proc: Option<ProcDefinition>,
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition(pub Pattern, pub Box<FuncExpr>);

#[derive(Clone, Debug)]
pub struct QueryDefinition(pub Box<QueryExpr>);

#[derive(Clone, Debug)]
pub enum ProcDefinition {
    Return(Option<Box<ProcExpr>>),
}

impl From<FunctionDefinition> for LambdaDefinition {
    fn from(fd: FunctionDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: Some(fd),
            query: None,
            proc: None,
        }
    }
}

impl From<QueryDefinition> for LambdaDefinition {
    fn from(qd: QueryDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: None,
            query: Some(qd),
            proc: None,
        }
    }
}

impl From<ProcDefinition> for LambdaDefinition {
    fn from(pd: ProcDefinition) -> LambdaDefinition {
        LambdaDefinition {
            func: None,
            query: None,
            proc: Some(pd),
        }
    }
}
