#![deny(missing_docs)]
//! Ultra fast binary serialization and deserialization for types with a constant size (known at compile time). This
//! crate cannot be used to serialize or deserialize dynamically allocated types, such as,
//! [`String`](std::string::String), [`Vec`](std::vec::Vec), [`HashMap`](std::collections::HashMap), etc.
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
//! `Desse::serialize` will serialize this struct in `[u8; 3]` where `3` is the sum of sizes of `u8` and `u16`.
//!
//! ## Performance
//!
//! This crate values performance more than anything. We don't shy away from using tested and verified **unsafe** code
//! if it improves performance.
//!
//! ## Future Improvements
//!
//! Once [`const_generics`](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md) is implemented
//! in Rust, we can provide default implementations for many types such as, `impl Desse for [T; n] where T: Desse`, and
//! other variable size statically allocated types in Rust.
mod desse;

pub use crate::desse::{Desse, DesseSized};

#[cfg(feature = "derive")]
pub use desse_derive::*;
