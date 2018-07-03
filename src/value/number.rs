use num::{BigInt, FromPrimitive, Zero};
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::Neg;

pub struct Number {
    i: BigInt,
    point: i32,
}

impl Number {
    pub fn new(mut i: BigInt, mut point: i32) -> Number {
        let ten: BigInt = BigInt::from_i64(10).unwrap();
        let zero: BigInt = BigInt::zero();

        // "Normalize point" to be the value closest to zero that
        // preserves full information. For every trailing 0 digit in i,
        // divide it by ten and increment point by 1.
        //
        // Examples:
        //
        // a: 1230000 pt -1 -> 123 pt 3
        // b: 1230000 pt 2 -> 123 pt 6
        // c: 1230000 pt -8 -> 123 pt -4
        // d: -1230000 pt 2 -> -123 pt 6
        //
        if &i == &zero {
            point = 0;
        } else {
            // FIXME: divmod operation may be an improvement.
            while &i % &ten == zero {
                i = &i / &ten;
                point += 1;
            }
        }
        Number { i: i, point: point }
    }

    pub fn from_bigint(i: BigInt) -> Number {
        Number::new(i, 0)
    }
}

impl PartialEq<Number> for Number {
    fn eq(&self, other: &Number) -> bool {
        self.point == other.point && self.i == other.i
    }
}

// impl Add<Number> for Number {
// }
//
//
// impl Zero for Number {
// fn zero()
// }
//

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Number {
        Number::new(-self.i, self.point)
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Number {}", self)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use num::Zero;

        if self.i.is_zero() {
            write!(f, "0")
        } else {
            let digs = format!("{}", self.i);
            if self.point >= 0 {
                try!(write!(f, "{}", digs));
                for _ in 0..self.point {
                    try!(write!(f, "0"));
                }
                Ok(())
            } else {
                let c = (digs.len() as i32) + self.point;
                if c < 0 {
                    use num::bigint::Sign;

                    let signprefix = match self.i.sign() {
                        Sign::NoSign => "",
                        Sign::Minus => "-",
                        Sign::Plus => "",
                    };
                    let signadj = signprefix.len();

                    try!(write!(f, "{}0.", signprefix));
                    for _ in 0..(-c + (signadj as i32)) {
                        try!(write!(f, "0"));
                    }
                    write!(f, "{}", &digs[signadj..])
                } else {
                    let i = c as usize;
                    write!(f, "{}.{}", &digs[..i], &digs[i..])
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Number;

    macro_rules! display_tests {
        ( $( ( $name:ident, $i:expr, $pt:expr, $expected:expr ) ),* ) => {
            $(
                #[test]
                fn $name() {
                    use num::{BigInt, FromPrimitive};

                    let s = format!("{}",
                                    Number::new(
                                        BigInt::from_i64($i).unwrap(),
                                        $pt));
                    assert_eq!(s, $expected);
                }
            )*
        }
    }

    display_tests!(
        (test_display_pos_pt_0, 3, 0, "3"),
        (test_display_pos_pt_pos, 3, 2, "300"),
        (test_display_pos_pt_negmid, 54321, -2, "543.21"),
        (test_display_pos_pt_negbig, 54321, -7, "0.0054321"),
        (test_display_neg_pt_0, -3, 0, "-3"),
        (test_display_neg_pt_pos, -3, 2, "-300"),
        (test_display_neg_pt_negmid, -54321, -2, "-543.21"),
        (test_display_neg_pt_negbig, -54321, -7, "-0.0054321"),
        (test_display_0_pt_0, 0, 0, "0"),
        (test_display_0_pt_pos, 0, 2, "0"),
        (test_display_0_pt_negmid, 0, -2, "0"),
        (test_display_0_pt_negbig, 0, -7, "0")
    );
}
