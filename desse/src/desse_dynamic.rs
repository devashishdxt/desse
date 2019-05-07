#![cfg(feature = "dynamic")]

use alloc::string::String;
use alloc::vec::Vec;

use crate::{DesseSized, Result};

/// Marker trait for all types whose size is not known at compile time
pub trait DesseUnsized {
    /// Returns the size of bytes after serialization
    fn serialized_size(&self) -> usize;
}

/// Any type must implement this trait for serialization and deserialization
pub trait DesseDynamic: Sized {
    /// Serializes current object
    fn serialize(&self) -> Result<Vec<u8>>;

    /// Serializes current object into bytes
    fn serialize_into(&self, bytes: &mut [u8]) -> Result<usize>;

    /// Deserializes an object
    fn deserialize_from(bytes: &[u8]) -> Result<(Self, usize)>;
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
