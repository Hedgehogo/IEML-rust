//! API for reading IEML data structure.
//!
//! All structure comparison operations declared in this module compare only the consistency of node organization and their contents directly, but do not compare marks.

pub mod analyse_anchors;
pub mod anchor_id;
pub mod anchors;
pub mod buffer_anchors;
pub mod clear;
pub mod deserialize;
pub mod to_match;
pub mod type_view;
pub mod view;

pub use view::*;
