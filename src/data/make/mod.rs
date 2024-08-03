//! Universal API to ensure that valid Data is guaranteed to be retrieved.
//! This API allows you to write, among other things, full-fledged parsers without thinking about the validity of [`Data`][`super::data::Data`]
//! 
//! A valid [`Data`][`super::data::Data`] is one that does not contain nodes that cannot be obtained by obtaining child nodes from the top node. 
//! And it does not contain nodes referring to non-existent nodes.

pub mod combinator;
pub mod error;
mod init;
pub mod maker;

pub use combinator::*;
pub use error::marked::*;
