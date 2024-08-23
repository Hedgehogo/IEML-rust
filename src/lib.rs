//! # Serde IEML
//!
//! A Rust library for using the [Serde](https://crates.io/crates/serde) serialization framework with data in IEML document format.

pub mod data;
pub mod de;
pub mod error;

pub use de::{from_source, from_source_advanced};
