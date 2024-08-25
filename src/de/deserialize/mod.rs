//! Deserialize an IEML data structure to an Rust data structure.

pub mod deserializer;
pub mod anchor_id;
pub mod buffer_anchors;
mod access;

pub use deserializer::{Deserializer, Error, Result};
