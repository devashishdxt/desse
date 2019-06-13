#![cfg(feature = "dynamic")]
use alloc::vec::Vec;

use crate::private::Sealed;
use crate::{ErrorKind, Result};

/// Trait for doing `write` operations.
pub trait Writer: Sealed {
    /// Writes the `buf` in current object.
    fn write(&mut self, buf: &[u8]) -> Result<()>;

    /// Writes `buf` in current object.
    ///
    /// # Panic
    ///
    /// Panics if there's not enough space in writer for storing `buf` (only when writer is `&mut [u8]`).
    fn write_unchecked(&mut self, buf: &[u8]) -> Result<()>;
}

impl Writer for &mut [u8] {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        if self.len() < buf.len() {
            return Err(ErrorKind::InvalidSliceLength.into());
        }

        self.write_unchecked(buf)
    }

    #[inline]
    fn write_unchecked(&mut self, buf: &[u8]) -> Result<()> {
        let (a, b) = core::mem::replace(self, &mut []).split_at_mut(buf.len());
        a.copy_from_slice(buf);
        *self = b;
        Ok(())
    }
}

impl Writer for Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.write_unchecked(buf)
    }

    #[inline]
    fn write_unchecked(&mut self, buf: &[u8]) -> Result<()> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

impl<W> Writer for &mut W
where
    W: Writer,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        (**self).write(buf)
    }

    #[inline]
    fn write_unchecked(&mut self, buf: &[u8]) -> Result<()> {
        (**self).write_unchecked(buf)
    }
}
