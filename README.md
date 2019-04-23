# Desse [![Build Status](https://travis-ci.org/devashishdxt/desse.svg?branch=master)](https://travis-ci.org/devashishdxt/desse)
Ultra fast binary serialization and deserialization for types with a constant size (known at compile time). This
crate cannot be used to serialize or deserialize dynamically allocated types, such as,
[`String`](std::string::String), [`Vec`](std::vec::Vec), [`HashMap`](std::collections::HashMap), etc., and types 
with unknown size at compile time such as `slices`, `&str`, etc.

## Binary Encoding Scheme
This crate uses a minimal binary encoding scheme such that the size of encoded object will be smaller than (in cases
where Rust adds padding bytes for alignment) or equal to it's size in a running Rust program. For example, consider
the following `struct`:

```
struct MyStruct {
    a: u8,
    b: u16,
}
```

`Desse::serialize` will serialize this struct in `[u8; 3]` where `3` is the sum of sizes of `u8` and `u16`.

## Usage
Add `desse` in your `Cargo.toml`'s `dependencies` section.
```
[dependencies]
desse = "0.1"
```

`Desse` trait can be implemented for any struct or enum (whose size is known at compile time) using `derive` macro. This
crate also provides a `derive` macro for implementing `DesseSized` trait which is necessary for implementing `Desse`
trait.
```
use desse::{Desse, DesseSized};

#[derive(Debug, PartialEq, Desse, DesseSized)]
struct MyStruct {
    a: u8,
    b: u16,
}
```

Now, you can use `Desse::serialize` and `Desse::deserialize_from` for serialization and deserialization of this 
struct.

```
let my_struct = MyStruct { a: 5, b: 1005 };
let serialized: [u8; 3] = my_struct.serialize();
let new_struct = MyStruct::deserialize_from(&serialized);

assert_eq!(my_struct, new_struct);
```

Note that `Desse::serialize` returns an array of fixed length (`3` in above case) and `Desse::deserialize` takes
reference to an array of fixed length as argument.

## Performance
This crate values performance more than anything. We don't shy away from using tested and verified **unsafe** code
if it improves performance.

### Benchmarks
Below are the benchmark results of comparison between `desse` and `bincode` serializing and deserializing same `struct`:
```
struct::serialize/desse::serialize
                        time:   [1.6228 ns 1.6326 ns 1.6434 ns]
                        change: [-1.1985% +0.0554% +1.2769%] (p = 0.94 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

struct::serialize/bincode::serialize
                        time:   [19.991 ns 20.081 ns 20.201 ns]
                        change: [-1.0739% +0.3569% +1.7361%] (p = 0.63 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

struct::deserialize/desse::deserialize
                        time:   [1.6063 ns 1.6101 ns 1.6144 ns]
                        change: [-1.3079% -0.1278% +1.0394%] (p = 0.84 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

struct::deserialize/bincode::deserialize
                        time:   [22.004 ns 22.094 ns 22.209 ns]
                        change: [-1.1573% +0.0698% +1.3631%] (p = 0.92 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
```

It is clear from above benchmarks that `bincode` takes `20.081 ns` on an average for serialization whereas `desse` takes
`1.6326 ns`. The results are also similar for deserialization where `bincode` takes `22.094 ns` and `desse` takes
`1.6101 ns`.

You can run benchmarks by running following command:
```
cargo bench
```

## Future Improvements
Once [`const_generics`](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md) is implemented
in Rust, we can provide default implementations for many types such as, `impl Desse for [T; n] where T: Desse`, and
other variable size statically allocated types in Rust.

## License
Licensed under either of
- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
