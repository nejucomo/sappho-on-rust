use num::BigRational;


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum Number {
    RBig(BigRational),
}


impl Number {
    pub fn from_bigrational(r: BigRational) -> Number { Number::RBig(r) }
}
