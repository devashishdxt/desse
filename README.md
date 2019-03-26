# Desse
This crate exposes functionality for serialization and deserialization of types with a constant size, known at 
compile time. Any dynamically allocated types, such as, [`String`](std::string::String), [`Vec`](std::vec::Vec), 
[`HashMap`](std::collections::HashMap), etc. cannot be serialized using this crate.

## Future improvements
Once [`const_generics`](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md) is implemented
in Rust, we can start adding variable size statically allocated types in this library.

## License
Licensed under either of
- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)
at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
