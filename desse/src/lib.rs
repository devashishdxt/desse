#![deny(missing_docs)]
//! This crate exposes functionality for binary serialization and deserialization of types with a constant size, known
//! at compile time. Any dynamically allocated types, such as, [`String`](std::string::String), [`Vec`](std::vec::Vec),
//! [`HashMap`](std::collections::HashMap), etc. cannot be serialized using this crate.
//!
//! ## Philosophy
//! This crate values performance more than anything. We don't shy away from using tested and verified unsafe code if it
//! improves performance.
//!
//! - Current lines of **unsafe** code: **1**
//!
//! ## Future improvements
//! Once [`const_generics`](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md) is implemented
//! in Rust, we can start adding variable size statically allocated types in this library.
mod desse;

pub use crate::desse::{Desse, DesseSized};

#[cfg(feature = "derive")]
pub use desse_derive::*;
