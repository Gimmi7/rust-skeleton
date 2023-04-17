use crate::arithmetic::errors::ParseBigIntError;

pub trait WrappedBigInt: InnerAccess + zeroize::Zeroize + Converter + num_traits::Num {}

pub trait Wrap {
    type WrappedType: WrappedBigInt;
    fn wrap(self) -> Self::WrappedType;
}

/// Access the innerType as reference, mutable reference and acquire ownership
pub trait InnerAccess {
    type InnerType: Wrap;
    fn inner_ref(&self) -> &Self::InnerType;
    fn inner_mut(&mut self) -> &mut Self::InnerType;
    fn into_inner(self) -> Self::InnerType;
}

/// Converts
pub trait Converter: Sized {
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &[u8]) -> Self;

    fn to_str_radix(&self, radix: u8) -> String;

    fn from_str_radix(s: &str, radix: u8) -> Result<Self, ParseBigIntError>;

    fn to_hex(&self) -> String { self.to_str_radix(16) }

    fn from_hex(hex_str: &str) -> Result<Self, ParseBigIntError> { Self::from_str_radix(hex_str, 16) }

    /// Returns bytes representation of the number in an array with length specify by the user
    /// If the array is larger that than the bytes it pads the array with zeros in the most significant bytes
    /// If the array is too small for the inter it returns None
    fn to_bytes_array<const N: usize>(&self) -> Option<[u8; N]> {
        let bytes = self.to_bytes();
        if bytes.len() > N {
            return None;
        }

        let mut array = [0u8; N];
        array[N - bytes.len()..].copy_from_slice(&bytes);
        Some(array)
    }
}