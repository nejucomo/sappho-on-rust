#[derive(Clone, Debug)]
pub enum FuncUnOp {
    Invert,
}

#[derive(Clone, Debug)]
pub enum QueryUnOp {
    FUO(FuncUnOp),
    Resolve,
}

#[derive(Clone, Debug)]
pub enum ProcUnOp {
    QUO(QueryUnOp),
    Mutate,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Times,
}
