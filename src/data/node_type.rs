//! Type definition [`NodeType`]

use std::fmt;
use serde::de;

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

impl fmt::Display for NodeType {
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

impl<'data> From<de::Unexpected<'data>> for NodeType {
    fn from(value: de::Unexpected) -> Self {
        match value {
            de::Unexpected::Bool(_) => NodeType::Raw,
            de::Unexpected::Unsigned(_) => NodeType::Raw,
            de::Unexpected::Signed(_) => NodeType::Raw,
            de::Unexpected::Float(_) => NodeType::Raw,
            de::Unexpected::Char(_) => NodeType::Raw,
            de::Unexpected::Str(_) => NodeType::String,
            de::Unexpected::Bytes(_) => NodeType::Raw,
            de::Unexpected::Unit => NodeType::List,
            de::Unexpected::Option => NodeType::Null,
            de::Unexpected::NewtypeStruct => NodeType::Tagged,
            de::Unexpected::Seq => NodeType::List,
            de::Unexpected::Map => NodeType::Map,
            de::Unexpected::Enum => NodeType::Tagged,
            de::Unexpected::UnitVariant => NodeType::Raw,
            de::Unexpected::NewtypeVariant => NodeType::Tagged,
            de::Unexpected::TupleVariant => NodeType::Tagged,
            de::Unexpected::StructVariant => NodeType::Tagged,
            de::Unexpected::Other(_) => NodeType::Raw,
        }
    }
}
