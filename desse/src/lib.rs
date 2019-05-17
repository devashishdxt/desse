#![deny(missing_docs)]
//! Ultra fast binary serialization and deserialization for types with a constant size (known at compile time). This
//! crate cannot be used to serialize or deserialize dynamically allocated types, such as,
//! [`String`](alloc::string::String), [`Vec`](alloc::vec::Vec), [`BTreeMap`](alloc::collections::BTreeMap), etc., and
//! types with unknown size at compile time such as [`slice`](core::slice), [`&str`](core::str), etc.
//!
//! ## Binary Encoding Scheme
//!
//! This crate uses a minimal binary encoding scheme such that the size of encoded object will be smaller than (in cases
//! where Rust adds padding bytes for alignment) or equal to it's size in a running Rust program. For example, consider
//! the following `struct`:
//!
//! ```
//! struct MyStruct {
//!     a: u8,
//!     b: u16,
//! }
//! ```
//!
//! `DesseStatic::serialize` will serialize this struct in `[u8; 3]` where `3` is the sum of sizes of `u8` and `u16`.
//!
//! ## Usage
//!
//! `DesseStatic` trait can be implemented for any struct or enum (whose size is known at compile time) using `derive`
//! macro. This crate also provides a `derive` macro for implementing `DesseSized` trait which is necessary for
//! implementing `DesseStatic` trait.
//! ```
//! use desse::{DesseStatic, DesseSized};
//!
//! #[derive(Debug, PartialEq, DesseStatic, DesseSized)]
//! struct MyStruct {
//!     a: u8,
//!     b: u16,
//! }
//! ```
//!
//! Now, you can use `DesseStatic::serialize` and `DesseStatic::deserialize_from` for serialization and deserialization
//! of this struct.
//!
//! ```
//! # use desse::{DesseStatic, DesseSized};
//! #
//! # #[derive(Debug, PartialEq, DesseStatic, DesseSized)]
//! # struct MyStruct {
//! #     a: u8,
//! #     b: u16,
//! # }
//! #
//! let my_struct = MyStruct { a: 5, b: 1005 };
//! let serialized: [u8; 3] = my_struct.serialize();
//! let new_struct = MyStruct::deserialize_from(&serialized).unwrap();
//!
//! assert_eq!(my_struct, new_struct);
//! ```
//!
//! Note that `DesseStatic::serialize` returns an array of fixed length (`3` in above case) and
//! `DesseStatic::deserialize` takes reference to an array of fixed length as argument.

#![no_std]

#[cfg(feature = "dynamic")]
extern crate alloc;

#[cfg(feature = "dynamic")]
mod desse_dynamic;
mod desse_static;
mod error;
#[cfg(feature = "dynamic")]
mod reader;
#[cfg(feature = "dynamic")]
mod writer;

#[cfg(feature = "dynamic")]
pub use crate::desse_dynamic::DesseDynamic;
pub use crate::desse_static::{DesseSized, DesseStatic};
pub use crate::error::{Error, ErrorKind, Result};
#[cfg(feature = "dynamic")]
pub use crate::reader::Reader;
#[cfg(feature = "dynamic")]
pub use crate::writer::Writer;

#[cfg(feature = "derive")]
pub use desse_derive::*;

/// Compares and returns maximum of two values.
///
/// # Warning
///
/// This function internally uses bitwise operations to find maximum value, which, in some cases, may not be best
/// approach. But, Rust's [`max()`](https://doc.rust-lang.org/nightly/std/cmp/fn.max.html) is not a `const fn` which is
/// needed for compile time evaluation of `max` value.
///
/// The need for this function will go away once [`const fn`](https://github.com/rust-lang/rust/issues/57563) fully
/// lands in stable and `max()` becomes a `const fn`.
pub const fn max(x: usize, y: usize) -> usize {
    let x = x as i128;
    let y = y as i128;

    (x ^ ((x ^ y) & -((x < y) as i128))) as usize
}
