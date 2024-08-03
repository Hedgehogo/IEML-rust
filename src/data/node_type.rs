//! Type definition [`NodeType`]

use std::fmt::Display;

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
    Document,
    Anchor,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            NodeType::Null => "null",
            NodeType::Raw => "number, boolean, raw data",
            NodeType::String => "string",
            NodeType::List => "list",
            NodeType::Map => "map",
            NodeType::Tagged => "tagged",
            NodeType::Document => "document",
            NodeType::Anchor => "anchor",
        };

        write!(f, "{}", result)
    }
}