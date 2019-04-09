#![deny(missing_docs)]
//! Ultra fast binary serialization and deserialization for types with a constant size (known at compile time). This
//! crate cannot be used to serialize or deserialize dynamically allocated types, such as,
//! [`String`](std::string::String), [`Vec`](std::vec::Vec), [`HashMap`](std::collections::HashMap), etc., and types
//! with unknown size at compile time such as [`slice`](std::slice), [`&str`](std::str), etc.
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
//! ## Usage
//! Add `desse` in your `Cargo.toml`'s `dependencies` section.
//! ```
//! [dependencies]
//! desse = "0.1"
//! ```
//!
//! `Desse` trait can be implemented for any struct (whose size is known at compile time) using `derive` macro. This
//! crate also provides a `derive` macro for implementing `DesseSized` trait which is necessary for implementing `Desse`
//! trait.
//! ```
//! use desse::{Desse, DesseSized};
//!
//! #[derive(Desse, DesseSized)]
//! struct MyStruct {
//!     a: u8,
//!     b: u16,
//! }
//! ```
//!
//! Now, you can use `Desse::serialize` and `Desse::deserialize_from` for serialization and deserialization of this
//! struct.
//!
//! ```
//! let my_struct = MyStruct { a: 5, b: 1005 };
//! let serialized: [u8; 3] = my_struct.serialize();
//! let new_struct = MyStruct::deserialize_from(&serialized);
//!
//! assert_eq!(my_struct, new_struct);
//! ```
//!
//! Note that `Desse::serialize` returns an array of fixed length (`3` in above case) and `Desse::deserialize` takes
//! reference to an array of fixed length as argument.

#![no_std]

mod desse;

pub use crate::desse::{Desse, DesseSized};

#[cfg(feature = "derive")]
pub use desse_derive::*;
