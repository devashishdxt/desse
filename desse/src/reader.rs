#![cfg(feature = "dynamic")]
use crate::private::Sealed;
use crate::{ErrorKind, Result};

/// Trait for doing `read` operations.
pub trait Reader: Sealed + Sized {
    /// Reads `len` bytes from current reader
    fn read(&mut self, len: usize) -> Result<&[u8]>;

    /// Reads `len` bytes from current reader
    ///
    /// # Panic
    ///
    /// Panics if there are not enough bytes in reader
    fn read_unchecked(&mut self, len: usize) -> Result<&[u8]>;
}

impl Reader for &[u8] {
    #[inline]
    fn read(&mut self, len: usize) -> Result<&[u8]> {
        if self.len() < len {
            Err(ErrorKind::InvalidSliceLength.into())
        } else {
            self.read_unchecked(len)
        }
    }

    #[inline]
    fn read_unchecked(&mut self, len: usize) -> Result<&[u8]> {
        let (a, b) = self.split_at(len);
        *self = b;
        Ok(a)
    }
}

impl<R> Reader for &mut R
where
    R: Reader,
{
    #[inline]
    fn read(&mut self, len: usize) -> Result<&[u8]> {
        (**self).read(len)
    }

    #[inline]
    fn read_unchecked(&mut self, len: usize) -> Result<&[u8]> {
        (**self).read_unchecked(len)
    }
}
