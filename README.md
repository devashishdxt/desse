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

`Desse` trait can be implemented for any struct (whose size is known at compile time) using `derive` macro. This
crate also provides a `derive` macro for implementing `DesseSized` trait which is necessary for implementing `Desse`
trait.
```
use desse::{Desse, DesseSized};

#[derive(Desse, DesseSized)]
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
                        time:   [1.0707 ns 1.0733 ns 1.0765 ns]
                        change: [-0.4340% +1.0979% +2.8290%] (p = 0.18 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

struct::serialize/bincode::serialize
                        time:   [16.005 ns 16.042 ns 16.081 ns]
                        change: [-1.7999% -0.5179% +0.8202%] (p = 0.45 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

struct::deserialize/desse::deserialize
                        time:   [1.3383 ns 1.3433 ns 1.3503 ns]
                        change: [-1.4627% -0.0804% +1.4476%] (p = 0.92 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

struct::deserialize/bincode::deserialize
                        time:   [15.020 ns 15.069 ns 15.133 ns]
                        change: [-1.0730% +0.2426% +1.5766%] (p = 0.74 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe
```

It is clear from above benchmarks that `bincode` takes `16.042 ns` on an average for serialization whereas `desse` takes
`1.0733 ns`. The results are also similar for deserialization where `bincode` takes `15.069 ns` and `desse` takes
`1.3433 ns`.

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
