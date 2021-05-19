mod rational;
pub use rational::*;

use num::{BigInt, Zero, One, Num, FromPrimitive, ToPrimitive};

use crate::dispatchable;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Real {
    F64(f64),
    Rational(Rational),
}

dispatchable!(Real { F64, Rational });

impl From<f64> for Real {
    fn from(f: f64) -> Real { 
        if f.fract() == 0.0 {
            Real::Rational(BigInt::from_f64(f.trunc()).unwrap().into())
        } else {
            Real::F64(f)
        }
    }
}

impl From<Rational> for Real {
    fn from(r: Rational) -> Real {
        Real::Rational(r)
    }
}

impl FromPrimitive for Real {
    fn from_i64(i: i64) -> Option<Self> { Rational::from_i64(i).map(Real::from) }
    fn from_u64(u: u64) -> Option<Self> { Rational::from_u64(u).map(Real::from) }
    fn from_f64(f: f64) -> Option<Self> { Some(Real::F64(f)) }
}

impl ToPrimitive for Real {
    dispatch!(fn to_i64(&self) -> Option<i64>);
    dispatch!(fn to_u64(&self) -> Option<u64>);
    dispatch!(fn to_f64(&self) -> Option<f64>);
}

macro_rules! impl_num_op {
    ( $trait:ident { $func:ident } ) => {
        impl ::core::ops::$trait<Self> for Real {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Real::F64(lhs), Real::F64(rhs)) => ::core::ops::$trait::$func(lhs, rhs).into(),
                    (Real::F64(lhs), Real::Rational(rhs)) => ::core::ops::$trait::$func(lhs, rhs.to_f64().unwrap()).into(),
                    (Real::Rational(lhs), Real::F64(rhs)) => ::core::ops::$trait::$func(lhs.to_f64().unwrap(), rhs).into(),
                    (Real::Rational(lhs), Real::Rational(rhs)) => ::core::ops::$trait::$func(lhs, rhs).into()
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

impl Zero for Real {
    fn zero() -> Self { Rational::from(0).into() }
    dispatch!(fn is_zero(&self) -> bool);
}

impl One for Real {
    fn one() -> Self { Rational::from(1).into() }
    dispatch!(fn is_one(&self) -> bool);
}

impl Num for Real {
    type FromStrRadixErr = <Rational as Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        f64::from_str_radix(s, radix).map(Real::from)
            .or_else(|_| Rational::from_str_radix(s, radix).map(Real::from))
    }
}