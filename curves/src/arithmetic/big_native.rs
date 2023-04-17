use std::ops;

use num_bigint::{BigInt, Sign};
use num_traits::{Num, One, Zero};
use zeroize::Zeroize;

use crate::arithmetic::errors::{ParseBigIntError, ParseBigIntErrorReason};
use crate::arithmetic::traits::{Converter, InnerAccess, Wrap, WrappedBigInt};

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone)]
pub struct NativeBigInt {
    inner: BigInt,
}

impl Wrap for BigInt {
    type WrappedType = NativeBigInt;

    fn wrap(self) -> NativeBigInt {
        NativeBigInt {
            inner: self
        }
    }
}


impl InnerAccess for NativeBigInt {
    type InnerType = BigInt;

    fn inner_ref(&self) -> &Self::InnerType {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::InnerType {
        &mut self.inner
    }

    fn into_inner(self) -> Self::InnerType {
        self.inner
    }
}

/// In Rust, it's a good practice to zeroize a value when it contains sensitive or confidential information,
/// such as passwords, encryption keys, or private data.
impl Zeroize for NativeBigInt {
    fn zeroize(&mut self) {
        use std::{ptr, sync::atomic};
        unsafe { ptr::write_volatile(&mut self.inner, Zero::zero()) };
        atomic::fence(atomic::Ordering::SeqCst);
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }
}

impl Converter for NativeBigInt {
    fn to_bytes(&self) -> Vec<u8> {
        let (_sign, bytes) = self.inner.to_bytes_be();
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        BigInt::from_bytes_le(Sign::Plus, bytes).wrap()
    }

    fn to_str_radix(&self, radix: u8) -> String {
        self.inner.to_str_radix(radix.into())
    }

    fn from_str_radix(s: &str, radix: u8) -> Result<Self, ParseBigIntError> {
        BigInt::parse_bytes(s.as_bytes(), radix.into())
            .map(Wrap::wrap)
            .ok_or(ParseBigIntError {
                reason: ParseBigIntErrorReason::NumBigint,
                radix: radix.into(),
            })
    }
}

// implement from method for NativeBigInt
crate::wrapped_bigint_impl_from! {NativeBigInt, BigInt, u32, i32, u64}

impl Num for NativeBigInt {
    type FromStrRadixErr = ParseBigIntError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        <Self as Converter>::from_str_radix(str, radix.try_into().unwrap())
    }
}


impl Zero for NativeBigInt {
    fn zero() -> Self {
        BigInt::zero().wrap()
    }

    fn is_zero(&self) -> bool {
        matches!(self.inner.sign(), Sign::NoSign)
    }
}

// implement num ops for NativeBigInt
crate::wrapped_bigint_impl_ops! {
    NativeBigInt,
    Add add,
    Sub sub,
    Mul mul,
    Div div,
    Rem rem,
    BitAnd bitand,
    BitXor bitxor
}



impl One for NativeBigInt {
    fn one() -> Self {
        BigInt::one().wrap()
    }
}


// impl NumOps for NativeBigInt {}


impl WrappedBigInt for NativeBigInt {}


#[cfg(test)]
mod test {
    use crate::arithmetic::big_native::NativeBigInt;
    use crate::arithmetic::traits::InnerAccess;

    #[test]
    fn test_impl_from_macro() {
        let bn = NativeBigInt::from(32u32);
        println!("{:?}", bn.inner_ref());

        let i32bn = NativeBigInt::from(66i32);
        println!("{:?}", i32bn.inner_ref());

        let u64bn = NativeBigInt::from(77u64);
        println!("{:?}", u64bn.inner_ref());
    }

    #[test]
    fn test_impl_ops_macro() {
        // add
        let owner_bn = NativeBigInt::from(1) + NativeBigInt::from(2);
        println!("+owner_bn:{:?}", owner_bn.inner_ref());

        let ref_bn = &NativeBigInt::from(1) + &NativeBigInt::from(2);
        println!("+ref_bn:{:?}", ref_bn.inner_ref());

        let owner_ref_bn = NativeBigInt::from(1) + &NativeBigInt::from(2);
        println!("+owner_ref_bn:{:?}", owner_ref_bn.inner_ref());

        let ref_owner_bn = &NativeBigInt::from(1) + NativeBigInt::from(2);
        println!("+ref_owner_bn:{:?}", ref_owner_bn.inner_ref());

        // sub
        let owner_sub = NativeBigInt::from(3) - NativeBigInt::from(5);
        println!("owner_sub:{:?}", owner_sub.inner_ref());

        // mul
        let owner_mul = NativeBigInt::from(5) * NativeBigInt::from(6);
        println!("owner_mul:{:?}", owner_mul.inner_ref());

        // div
        let owner_div = NativeBigInt::from(5) / NativeBigInt::from(2);
        println!("owner_div:{:?}", owner_div.inner_ref());

        // rem
        let owner_rem = NativeBigInt::from(15) % NativeBigInt::from(7);
        println!("owner_rem:{:?}", owner_rem.inner_ref());
    }
}