mod real;
pub use real::*;

use num::{Complex, Zero, One, Num, FromPrimitive, ToPrimitive};

use crate::dispatchable;

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Complex(Complex<Real>),
    Real(Real)
}

dispatchable!(Number { Complex, Real });

impl From<Complex<Real>> for Number {
    fn from(n: Complex<Real>) -> Number {
        if n.im.is_zero() { Number::Real(n.re) } else { Number::Complex(n) }
    }
}

impl From<Real> for Number {
    fn from(n: Real) -> Number {
        Number::Real(n)
    }
}

impl FromPrimitive for Number {
    fn from_i64(i: i64) -> Option<Self> { Real::from_i64(i).map(Number::from) }
    fn from_u64(u: u64) -> Option<Self> { Real::from_u64(u).map(Number::from) }
    fn from_f64(f: f64) -> Option<Self> { Real::from_f64(f).map(Number::from) }
}

impl ToPrimitive for Number {
    dispatch!(fn to_i64(&self) -> Option<i64>);
    dispatch!(fn to_u64(&self) -> Option<u64>);
    dispatch!(fn to_f64(&self) -> Option<f64>);
}

macro_rules! impl_num_op {
    ( $trait:ident { $func:ident } ) => {
        impl ::core::ops::$trait<Self> for Number {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Number::Complex(lhs), Number::Complex(rhs)) => ::core::ops::$trait::$func(lhs, rhs).into(),
                    (Number::Complex(lhs), Number::Real(rhs)) => ::core::ops::$trait::$func(lhs, rhs).into(),
                    (Number::Real(lhs), Number::Complex(rhs)) => ::core::ops::$trait::$func(Complex::from(lhs), rhs).into(),
                    (Number::Real(lhs), Number::Real(rhs)) => ::core::ops::$trait::$func(lhs, rhs).into()
                }
            }
        }
    }
}

impl_num_op!{ Add { add } }
impl_num_op!{ Sub { sub } }
impl_num_op!{ Mul { mul } }
impl_num_op!{ Div { div } }
impl_num_op!{ Rem { rem } }

impl Zero for Number {
    fn zero() -> Self { Real::from(0.0).into() }
    dispatch!(fn is_zero(&self) -> bool);
}

impl One for Number {
    fn one() -> Self { Real::from(1.0).into() }
    dispatch!(fn is_one(&self) -> bool);
}

impl Num for Number {
    type FromStrRadixErr = <Complex<Real> as Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Real::from_str_radix(s, radix).map(Number::from)
            .or_else(|_| Complex::from_str_radix(s, radix).map(Number::from))
    }
}