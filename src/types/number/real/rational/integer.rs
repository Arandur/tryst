use crate::dispatchable;

use num::{BigInt, Zero, One, Num, FromPrimitive, ToPrimitive, Integer as TInteger, bigint::ToBigInt};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Integer {
    I64(i64),
    BigInt(BigInt)
}

dispatchable!(Integer { I64, BigInt });

impl From<i64> for Integer {
    fn from(i: i64) -> Integer { Integer::I64(i) }
}

impl From<BigInt> for Integer {
    fn from(i: BigInt) -> Integer { 
        // Try to store it as an i64 first; otherwise store it as a BigInt
        i.to_i64().map_or_else(|| Integer::BigInt(i), |i| i.into())
    }
}

impl ToBigInt for Integer {
    dispatch!(fn to_bigint(&self) -> Option<BigInt>);
}

impl FromPrimitive for Integer {
    fn from_i64(i: i64) -> Option<Self> { Some(i.into()) }
    fn from_u64(u: u64) -> Option<Self> { 
        i64::from_u64(u).map_or_else(
            || BigInt::from_u64(u).map(Integer::BigInt),
            Self::from_i64
        )
    }
}

impl ToPrimitive for Integer {
    dispatch!(fn to_i64(&self) -> Option<i64>);
    dispatch!(fn to_u64(&self) -> Option<u64>);
}

macro_rules! impl_num_op {
    ( $trait:ident { $func:ident, $checked_func:ident } ) => {
        impl ::core::ops::$trait<Self> for Integer {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Integer::I64(lhs), Integer::I64(rhs)) => lhs.$checked_func(rhs).map_or_else(
                        || {
                            let lhs = BigInt::from_i64(lhs).unwrap();
                            let rhs = BigInt::from_i64(rhs).unwrap();
                            Integer::BigInt(::core::ops::$trait::$func(lhs, rhs))
                        },
                        Integer::I64),
                    (Integer::I64(lhs), Integer::BigInt(rhs)) => Integer::BigInt(::core::ops::$trait::$func(lhs, rhs)),
                    (Integer::BigInt(lhs), Integer::I64(rhs)) => Integer::BigInt(::core::ops::$trait::$func(lhs, rhs)),
                    (Integer::BigInt(lhs), Integer::BigInt(rhs)) => Integer::BigInt(::core::ops::$trait::$func(lhs, rhs))
                }
            }
        }
    }
}

impl_num_op! { Add { add, checked_add } }
impl_num_op! { Sub { sub, checked_sub } }
impl_num_op! { Mul { mul, checked_mul } }
impl_num_op! { Div { div, checked_div } }

impl ::core::ops::Rem<Self> for Integer {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::I64(lhs), Integer::I64(rhs)) => Integer::I64(lhs.rem(rhs)),
            (Integer::I64(lhs), Integer::BigInt(rhs)) => Integer::BigInt(lhs.rem(rhs)),
            (Integer::BigInt(lhs), Integer::I64(rhs)) => Integer::BigInt(lhs.rem(rhs)),
            (Integer::BigInt(lhs), Integer::BigInt(rhs)) => Integer::BigInt(lhs.rem(rhs))
        }
    }
}

impl Zero for Integer { 
    fn zero() -> Self { Integer::I64(0) }

    dispatch!(fn is_zero(&self) -> bool);
}

impl One for Integer {
    fn one() -> Self { Integer::I64(1) }

    dispatch!(fn is_one(&self) -> bool);
}

impl Num for Integer {
    type FromStrRadixErr = <BigInt as Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        i64::from_str_radix(s, radix).map(Integer::from)
            .or_else(|_| BigInt::from_str_radix(s, radix).map(Integer::from))
    }
}

macro_rules! impl_fn {
    ($func:ident ( self, $rhs:ident ) -> $ret:ty) => {
        fn $func (&self, $rhs: &Self) -> $ret {
            match self {
                Integer::I64(lhs) => $rhs.to_i64().map_or_else(
                    || BigInt::from_i64(*lhs).unwrap().$func(&$rhs.to_bigint().unwrap()).into(),
                    |rhs| lhs.$func(&rhs).into()
                ),
                Integer::BigInt(lhs) => lhs.$func(&$rhs.to_bigint().unwrap()).into()
            }
        }
    };
}

impl TInteger for Integer {
    impl_fn!(div_floor(self, rhs) -> Self);
    impl_fn!(mod_floor(self, rhs) -> Self);
    impl_fn!(gcd(self, rhs) -> Self);
    impl_fn!(lcm(self, rhs) -> Self);
    impl_fn!(is_multiple_of(self, rhs) -> bool);

    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        fn cvt<T: Into<Integer>>((a, b): (T, T)) -> (Integer, Integer) { (a.into(), b.into()) }

        match self {
            Integer::I64(lhs) => rhs.to_i64().map_or_else(
                || cvt(BigInt::from_i64(*lhs).unwrap().div_rem(&rhs.to_bigint().unwrap())),
                |rhs| cvt(lhs.div_rem(&rhs))
            ),
            Integer::BigInt(lhs) => cvt(lhs.div_rem(&rhs.to_bigint().unwrap()))
        }
    }

    fn divides(&self, rhs: &Self) -> bool { self.is_multiple_of(rhs) }

    dispatch!(fn is_even(&self) -> bool);

    dispatch!(fn is_odd(&self) -> bool);
}