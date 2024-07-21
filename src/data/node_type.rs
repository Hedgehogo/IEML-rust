//! This module is designed to describe [`NodeType`]

/// Describes the node type without storing the data itself
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeType {
    Null = 0,
    Raw,
    String,
    List,
    Map,
    Tagged,
    File,
    TakeAnchor,
    GetAnchor,
}
