use std::ops::Neg;
use num::BigRational;


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub struct Number(BigRational);


impl Number {
    pub fn from_bigrational(r: BigRational) -> Number { Number(r) }
}


impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Number { Number(-self.0) }
}
