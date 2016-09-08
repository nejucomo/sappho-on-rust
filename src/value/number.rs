use std::ops::Neg;
use std::fmt::{Debug, Formatter, Error};
use num::BigRational;


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


impl Debug for Number {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Number {}", self.0)
    }
}
