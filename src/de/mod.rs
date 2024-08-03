//! Deserialize IEML input to a Rust data structure.

pub mod parse;

pub use parse::from::{from_source, from_source_with_anchors};
