#![cfg(feature = "dynamic")]
use alloc::vec::Vec;

pub trait Sealed {}

impl Sealed for &[u8] {}
impl Sealed for &mut [u8] {}
impl Sealed for Vec<u8> {}

impl<S> Sealed for &mut S where S: Sealed {}
