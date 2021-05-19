mod integer;
pub use integer::*;

use num::{rational::Ratio, Zero, One, Num, FromPrimitive, ToPrimitive};

use crate::dispatchable;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rational {
    Integer(Integer),
    Fraction(Ratio<Integer>),
}

dispatchable!(Rational { Integer, Fraction });

impl From<Ratio<Integer>> for Rational {
    fn from(r: Ratio<Integer>) -> Self {
        if r.is_integer() {
            r.numer().clone().into()
        } else {
            Rational::Fraction(r)
        }
    }
}

impl <T> From<T> for Rational where Integer: From<T> {
    fn from(r: T) -> Self { Rational::Integer(Integer::from(r)) }
}

impl FromPrimitive for Rational {
    fn from_i64(n: i64) -> Option<Self> { Integer::from_i64(n).map(Rational::from) }
    fn from_u64(n: u64) -> Option<Self> { Integer::from_u64(n).map(Rational::from) }
}

impl ToPrimitive for Rational {
    dispatch!(fn to_u64(&self) -> Option<u64>);
    dispatch!(fn to_i64(&self) -> Option<i64>);
}

macro_rules! impl_num_op {
    ($trait:ident { $func:ident }) => {
        impl ::core::ops::$trait<Self> for Rational {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Rational::Integer(lhs), Rational::Integer(rhs)) => (::core::ops::$trait::$func(lhs, rhs)).into(),
                    (Rational::Integer(lhs), Rational::Fraction(rhs)) => (::core::ops::$trait::$func(Ratio::from(lhs), rhs)).into(),
                    (Rational::Fraction(lhs), Rational::Integer(rhs)) => (::core::ops::$trait::$func(lhs, Ratio::from(rhs))).into(),
                    (Rational::Fraction(lhs), Rational::Fraction(rhs)) => (::core::ops::$trait::$func(lhs, rhs)).into()
                }
            }
        }
    };
}

impl_num_op!{ Add { add } }
impl_num_op!{ Sub { sub } }
impl_num_op!{ Mul { mul } }
impl_num_op!{ Div { div } }
impl_num_op!{ Rem { rem } }

impl Zero for Rational {
    fn zero() -> Self { Integer::from(0).into() }
    dispatch!(fn is_zero(&self) -> bool);
}

impl One for Rational {
    fn one() -> Self { Integer::from(1).into() }
    dispatch!(fn is_one(&self) -> bool);
}

impl Num for Rational {
    type FromStrRadixErr = <Integer as Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if let Some((num, den)) = s.split_once('/') {
            Integer::from_str_radix(num, radix)
                .and_then(|num| Integer::from_str_radix(den, radix).map(|den| (num, den)))
                .map(|(num, den)| Ratio::new(num, den).into())
        } else {
            Integer::from_str_radix(s, radix).map(Rational::from)
        }
    }
}