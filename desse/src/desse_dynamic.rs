#![cfg(feature = "dynamic")]
use alloc::string::String;
use alloc::vec::Vec;
use core::time::Duration;

use crate::{DesseSized, DesseStatic, Reader, Result, Writer};

/// Any type must implement this trait for serialization and deserialization
pub trait DesseDynamic {
    /// Type of deserialized object
    type Output: DesseDynamic;

    /// Returns the size of bytes after serialization
    fn serialized_size(&self) -> usize;

    /// Serializes current object into a vector
    fn serialize(&self) -> Result<Vec<u8>>;

    /// Serializes current object into provided byte slice
    ///
    /// # Error
    ///
    /// This function returns error (`ErrorKind::InvalidSliceLength`) when length of input slice is less than
    /// `serialized_size()`.
    fn serialize_into<W: Writer>(&self, writer: W) -> Result<()>;

    /// Serializes current object into bytes
    ///
    /// # Panic
    ///
    /// This function panics when length of input slice is not equal to `serialized_size()`.
    fn serialize_into_unchecked<W: Writer>(&self, writer: W) -> Result<()>;

    /// Deserializes byte slice into an object
    ///
    /// # Error
    ///
    /// This function returns error (`ErrorKind::InvalidSliceLength`) when length of input slice is less than expected.
    fn deserialize_from<R: Reader>(reader: R) -> Result<Self::Output>;

    /// Deserializes an object
    ///
    /// # Panic
    ///
    /// This function panics when length of input slice is less than expected.
    fn deserialize_from_unchecked<R: Reader>(reader: R) -> Result<Self::Output>;
}

macro_rules! impl_desse_dynamic_for_static {
    ($type: ty) => {
        impl DesseDynamic for $type {
            type Output = Self;

            #[inline]
            fn serialized_size(&self) -> usize {
                <$type>::SIZE
            }

            #[inline]
            fn serialize(&self) -> Result<Vec<u8>> {
                Ok(<$type as DesseStatic>::serialize(self).to_vec())
            }

            #[inline]
            fn serialize_into<W: Writer>(&self, mut writer: W) -> Result<()> {
                writer.write(&<$type as DesseStatic>::serialize(self))
            }

            #[inline]
            fn serialize_into_unchecked<W: Writer>(&self, mut writer: W) -> Result<()> {
                writer.write_unchecked(&<$type as DesseStatic>::serialize(self))
            }

            #[inline]
            fn deserialize_from<R: Reader>(mut reader: R) -> Result<Self::Output> {
                let bytes = reader.read(<$type>::SIZE)?;
                unsafe {
                    <$type as DesseStatic>::deserialize_from(
                        &*(bytes.as_ptr() as *const [u8; <$type>::SIZE]),
                    )
                }
            }

            #[inline]
            fn deserialize_from_unchecked<R: Reader>(mut reader: R) -> Result<Self::Output> {
                let bytes = reader.read_unchecked(<$type>::SIZE)?;
                unsafe {
                    <$type as DesseStatic>::deserialize_from(
                        &*(bytes.as_ptr() as *const [u8; <$type>::SIZE]),
                    )
                }
            }
        }
    };
}

impl_desse_dynamic_for_static!(bool);
impl_desse_dynamic_for_static!(char);

impl_desse_dynamic_for_static!(u8);
impl_desse_dynamic_for_static!(u16);
impl_desse_dynamic_for_static!(u32);
impl_desse_dynamic_for_static!(u64);
impl_desse_dynamic_for_static!(u128);

impl_desse_dynamic_for_static!(i8);
impl_desse_dynamic_for_static!(i16);
impl_desse_dynamic_for_static!(i32);
impl_desse_dynamic_for_static!(i64);
impl_desse_dynamic_for_static!(i128);

impl_desse_dynamic_for_static!(Duration);

macro_rules! impl_desse_dynamic_str {
    ($type: ty) => {
        impl DesseDynamic for $type {
            type Output = String;

            #[inline]
            fn serialized_size(&self) -> usize {
                <u64>::SIZE + self.len()
            }

            #[inline]
            fn serialize(&self) -> Result<Vec<u8>> {
                let mut bytes = Vec::with_capacity(DesseDynamic::serialized_size(self));
                DesseDynamic::serialize_into_unchecked(self, &mut bytes)?;
                Ok(bytes)
            }

            #[inline]
            fn serialize_into<W: Writer>(&self, mut writer: W) -> Result<()> {
                let len = self.len() as u64;
                DesseDynamic::serialize_into(&len, &mut writer)?;
                writer.write(&self.as_bytes())
            }

            #[inline]
            fn serialize_into_unchecked<W: Writer>(&self, mut writer: W) -> Result<()> {
                let len = self.len() as u64;
                DesseDynamic::serialize_into_unchecked(&len, &mut writer)?;
                writer.write_unchecked(&self.as_bytes())
            }

            #[inline]
            fn deserialize_from<R: Reader>(mut reader: R) -> Result<Self::Output> {
                let len = <u64 as DesseDynamic>::deserialize_from(&mut reader)?;
                Ok(String::from_utf8(reader.read(len as usize)?.to_vec())?)
            }

            #[inline]
            fn deserialize_from_unchecked<R: Reader>(mut reader: R) -> Result<Self::Output> {
                let len = <u64 as DesseDynamic>::deserialize_from_unchecked(&mut reader)?;
                Ok(String::from_utf8(reader.read(len as usize)?.to_vec())?)
            }
        }
    };
}

impl_desse_dynamic_str!(&str);
impl_desse_dynamic_str!(String);

impl<T> DesseDynamic for Vec<T>
where
    T: DesseDynamic,
{
    type Output = Vec<T::Output>;

    #[inline]
    fn serialized_size(&self) -> usize {
        let sum = self
            .iter()
            .map(DesseDynamic::serialized_size)
            .sum::<usize>();

        <u64>::SIZE + sum
    }

    #[inline]
    fn serialize(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(DesseDynamic::serialized_size(self));
        DesseDynamic::serialize_into_unchecked(self, &mut bytes)?;
        Ok(bytes)
    }

    #[inline]
    fn serialize_into<W: Writer>(&self, mut writer: W) -> Result<()> {
        let len = self.len() as u64;
        DesseDynamic::serialize_into(&len, &mut writer)?;

        for item in self.iter() {
            DesseDynamic::serialize_into(item, &mut writer)?;
        }

        Ok(())
    }

    #[inline]
    fn serialize_into_unchecked<W: Writer>(&self, mut writer: W) -> Result<()> {
        let len = self.len() as u64;
        DesseDynamic::serialize_into_unchecked(&len, &mut writer)?;

        for item in self.iter() {
            DesseDynamic::serialize_into_unchecked(item, &mut writer)?;
        }

        Ok(())
    }

    #[inline]
    fn deserialize_from<R: Reader>(mut reader: R) -> Result<Self::Output> {
        let len = <u64 as DesseDynamic>::deserialize_from(&mut reader)?;
        let mut output = Vec::with_capacity(len as usize);

        let mut i = 0;
        while i < len {
            output.push(<T as DesseDynamic>::deserialize_from(&mut reader)?);
            i += 1;
        }

        Ok(output)
    }

    #[inline]
    fn deserialize_from_unchecked<R: Reader>(mut reader: R) -> Result<Self::Output> {
        let len = <u64 as DesseDynamic>::deserialize_from_unchecked(&mut reader)?;
        let mut output = Vec::with_capacity(len as usize);

        let mut i = 0;
        while i < len {
            output.push(<T as DesseDynamic>::deserialize_from_unchecked(
                &mut reader,
            )?);
            i += 1;
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    // For initializing global memory allocator
    extern crate std;

    use super::*;

    use alloc::string::ToString;
    use alloc::vec;

    macro_rules! impl_desse_dynamic_test {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let num: $type = rand::random::<$type>();
                let serialized = DesseDynamic::serialize(&num).unwrap();
                let new_num = <$type as DesseDynamic>::deserialize_from(&*serialized).unwrap();
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
    fn check_duration() {
        let duration = Duration::new(rand::random(), rand::random());
        let serialized = DesseDynamic::serialize(&duration).unwrap();
        let new_duration = <Duration as DesseDynamic>::deserialize_from(&*serialized).unwrap();
        assert_eq!(
            duration, new_duration,
            "Invalid serialization / deserialization"
        )
    }

    #[test]
    fn check_string() {
        let s = "hello".to_string();
        let serialized = DesseDynamic::serialize(&s).unwrap();
        let new_s = String::deserialize_from(&*serialized).unwrap();
        assert_eq!(s, new_s, "Invalid serialization / deserialization")
    }

    #[test]
    fn check_str() {
        let s = "hello";
        let serialized = DesseDynamic::serialize(&s).unwrap();
        let new_s = String::deserialize_from(&*serialized).unwrap();
        assert_eq!(s, new_s, "Invalid serialization / deserialization")
    }

    #[test]
    fn check_vec_str() {
        let v = vec!["hello".to_string(), "world".to_string()];
        let serialized = DesseDynamic::serialize(&vec!["hello", "world"]).unwrap();
        let new_v = Vec::<String>::deserialize_from(&*serialized).unwrap();
        assert_eq!(v, new_v, "Invalid serialization / deserialization")
    }
}
