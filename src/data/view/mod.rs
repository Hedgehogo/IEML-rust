//! The module is designed to provide an API for reading IEML data and then deserializing it.
//! 
//! All structure comparison operations declared in this module compare only the consistency of node organization and their contents directly, but do not compare marks. 

pub mod analyse_anchors;
pub mod anchors;
pub mod clear;
pub mod deserialize;
pub mod to_match;
pub mod type_view;
pub mod view;

pub use view::*;
