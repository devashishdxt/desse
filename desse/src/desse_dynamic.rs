#![cfg(feature = "dynamic")]

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::{DesseSized, DesseStatic, ErrorKind, Result};

/// Marker trait for all types whose size is not known at compile time
pub trait DesseUnsized: Sized {
    /// Returns the size of bytes after serialization
    fn serialized_size(&self) -> usize;
}

/// Any type must implement this trait for serialization and deserialization
pub trait DesseDynamic: DesseUnsized {
    /// Serializes current object into a vector
    fn serialize(&self) -> Result<Vec<u8>>;

    /// Serializes current object into provided byte slice
    ///
    /// # Error
    ///
    /// This function returns error (`ErrorKind::InvalidSliceLength`) when length of input slice is not equal to
    /// `serialized_size()`.
    fn serialize_into(&self, bytes: &mut [u8]) -> Result<()>;

    /// Serializes current object into bytes
    ///
    /// # Panic
    ///
    /// This function panics when length of input slice is not equal to `serialized_size()`.
    unsafe fn serialize_into_unchecked(&self, bytes: &mut [u8]) -> Result<()>;

    /// Deserializes byte slice into an object
    ///
    /// # Error
    ///
    /// This function returns error (`ErrorKind::InvalidSliceLength`) when length of input slice is less than expected.
    fn deserialize_from(bytes: &[u8]) -> Result<Self>;

    /// Deserializes an object
    ///
    /// # Panic
    ///
    /// This function panics when length of input slice is not equal to `serialized_size()`.
    unsafe fn deserialize_from_unchecked(bytes: &[u8]) -> Result<Self>;
}

impl DesseUnsized for String {
    #[inline]
    fn serialized_size(&self) -> usize {
        <u64>::SIZE + self.len()
    }
}

impl DesseUnsized for &str {
    #[inline]
    fn serialized_size(&self) -> usize {
        <u64>::SIZE + self.len()
    }
}

impl<T> DesseUnsized for T
where
    T: DesseSized,
{
    #[inline]
    fn serialized_size(&self) -> usize {
        <T>::SIZE
    }
}

impl<T> DesseUnsized for Vec<T>
where
    T: DesseUnsized,
{
    #[inline]
    fn serialized_size(&self) -> usize {
        let sum: usize = self.iter().map(DesseUnsized::serialized_size).sum();
        <u64>::SIZE + sum
    }
}

impl<T> DesseUnsized for &[T]
where
    T: DesseUnsized,
{
    #[inline]
    fn serialized_size(&self) -> usize {
        let sum: usize = self.iter().map(DesseUnsized::serialized_size).sum();
        <u64>::SIZE + sum
    }
}

macro_rules! impl_desse_dynamic {
    ($type: ty) => {
        impl DesseDynamic for $type {
            #[inline]
            fn serialize(&self) -> Result<Vec<u8>> {
                Ok(<$type as DesseStatic>::serialize(self).to_vec())
            }

            #[inline]
            fn serialize_into(&self, bytes: &mut [u8]) -> Result<()> {
                if <$type>::SIZE != bytes.len() {
                    Err(ErrorKind::InvalidSliceLength.into())
                } else {
                    unsafe { self.serialize_into_unchecked(bytes) }
                }
            }

            #[inline]
            unsafe fn serialize_into_unchecked(&self, bytes: &mut [u8]) -> Result<()> {
                <$type as DesseStatic>::serialize_into(
                    self,
                    &mut *(bytes.as_mut_ptr() as *mut [u8; <$type>::SIZE]),
                );

                Ok(())
            }

            #[inline]
            fn deserialize_from(bytes: &[u8]) -> Result<Self> {
                if bytes.len() < <$type>::SIZE {
                    Err(ErrorKind::InvalidSliceLength.into())
                } else {
                    unsafe { Self::deserialize_from_unchecked(bytes) }
                }
            }

            #[inline]
            unsafe fn deserialize_from_unchecked(bytes: &[u8]) -> Result<Self> {
                <$type as DesseStatic>::deserialize_from(
                    &*(bytes[0..<$type>::SIZE].as_ptr() as *const [u8; <$type>::SIZE]),
                )
            }
        }
    };
}

impl_desse_dynamic!(bool);
impl_desse_dynamic!(char);

impl_desse_dynamic!(u8);
impl_desse_dynamic!(u16);
impl_desse_dynamic!(u32);
impl_desse_dynamic!(u64);
impl_desse_dynamic!(u128);

impl_desse_dynamic!(i8);
impl_desse_dynamic!(i16);
impl_desse_dynamic!(i32);
impl_desse_dynamic!(i64);
impl_desse_dynamic!(i128);

impl DesseDynamic for String {
    #[inline]
    fn serialize(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![0; self.serialized_size()];
        self.serialize_into(&mut bytes)?;
        Ok(bytes)
    }

    #[inline]
    fn serialize_into(&self, bytes: &mut [u8]) -> Result<()> {
        if self.serialized_size() != bytes.len() {
            Err(ErrorKind::InvalidSliceLength.into())
        } else {
            unsafe { self.serialize_into_unchecked(bytes) }
        }
    }

    #[inline]
    unsafe fn serialize_into_unchecked(&self, bytes: &mut [u8]) -> Result<()> {
        let len = self.len() as u64;
        len.serialize_into_unchecked(&mut bytes[0..<u64>::SIZE])?;

        bytes[<u64>::SIZE..].copy_from_slice(self.as_bytes());

        Ok(())
    }

    #[inline]
    fn deserialize_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < <u64>::SIZE {
            Err(ErrorKind::InvalidSliceLength.into())
        } else {
            let len = unsafe {
                <u64 as DesseDynamic>::deserialize_from_unchecked(&bytes[0..<u64>::SIZE])?
            };

            if bytes.len() < <u64>::SIZE + (len as usize) {
                Err(ErrorKind::InvalidSliceLength.into())
            } else {
                unsafe { Self::deserialize_from_unchecked(bytes) }
            }
        }
    }

    #[inline]
    unsafe fn deserialize_from_unchecked(bytes: &[u8]) -> Result<Self> {
        let len =
            <u64 as DesseDynamic>::deserialize_from_unchecked(&bytes[0..<u64>::SIZE])? as usize;

        Ok(String::from_utf8(
            bytes[<u64>::SIZE..(<u64>::SIZE + len)].to_vec(),
        )?)
    }
}

#[cfg(test)]
mod tests {
    // For initializing global memory allocator
    extern crate std;

    use super::*;

    use alloc::string::ToString;

    macro_rules! impl_desse_dynamic_test {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let num: $type = rand::random::<$type>();
                let serialized = DesseDynamic::serialize(&num).unwrap();
                let new_num = <$type as DesseDynamic>::deserialize_from(&serialized).unwrap();
                assert_eq!(num, new_num, "Invalid serialization / deserialization")
            }
        };
    }

    impl_desse_dynamic_test!(bool, check_primitive_bool);
    impl_desse_dynamic_test!(char, check_primitive_char);

    impl_desse_dynamic_test!(u8, check_primitive_u8);
    impl_desse_dynamic_test!(u16, check_primitive_u16);
    impl_desse_dynamic_test!(u32, check_primitive_u32);
    impl_desse_dynamic_test!(u64, check_primitive_u64);
    impl_desse_dynamic_test!(u128, check_primitive_u128);

    impl_desse_dynamic_test!(i8, check_primitive_i8);
    impl_desse_dynamic_test!(i16, check_primitive_i16);
    impl_desse_dynamic_test!(i32, check_primitive_i32);
    impl_desse_dynamic_test!(i64, check_primitive_i64);
    impl_desse_dynamic_test!(i128, check_primitive_i128);

    #[test]
    fn check_string() {
        let s = "hello".to_string();
        let serialized = DesseDynamic::serialize(&s).unwrap();
        let new_s = String::deserialize_from(&serialized).unwrap();
        assert_eq!(s, new_s, "Invalid serialization / deserialization")
    }
}
