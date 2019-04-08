# Desse

`Desse` aims to be an ultra fast binary serialization and deserialization library for Rust. Throughout the whole
development, **performance** will take priority over all other things. Every change will be benchmarked rigorously and
will not be allowed if it degrades performance without sufficient feature addition.

## Development Roadmap

Below is a high level development plan for `desse` crate:

### Phase 1: Minimum Viable Product (`v0.1.*`)

- ~~Serialization and deserialization of `struct`s with constant size (known at compile time)~~
- Serialization and deserialization of `enum`s with constant size (known at compile time)

### Phase 2: Binary Encoding Scheme (`v0.2.*`)

- Develop a backwards compatible binary encoding scheme (implement as an optional feature).
- Support for user defined encoding scheme.

### Phase 3: Generics (`v0.3.*`)

- Serialization and deserialization of `struct`s and `enum`s with generics (generics should either be `const` or
  should implement `Desse` trait).

### Phase 4: Dynamically Allocated Data (`v0.4.*`)

- Serialization and deserialization of dynamically allocated types, such as, `String`, `Vec`, etc. (implement as an
  optional/default feature).

### **Release**: Stabilization (`v0.5.*`)

- Bug fixes and stabilization of APIs.
- Release as **LTS**.

