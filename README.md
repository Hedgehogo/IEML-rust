# Serde IEML

A Rust library for using the [Serde](https://crates.io/crates/serde) serialization framework with data in IEML document format. This crate is still in development.

## Progress
- [x] Data structures and APIs for working with them
- [x] Parsing from raw input to an intermediate data structure
- [x] Implementation of Serde traits for deserialization
- [ ] Generating a set of documents from an intermediate data structure
- [ ] Implementation of Serde traits for serialization

## Dependency

```toml
[dependencies]
serde = "1.0.*"
serde_ieml = "0.2.*"
```