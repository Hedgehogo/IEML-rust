use std::{
    collections::HashMap,
    path::Path,
};
use crate::anchor_keeper::AnchorKeeper;
use super::{
    node_type::NodeType,
    mark::Mark,
};

pub type RawCell = String;

pub type StringCell = String;

pub type ListCell = Vec<usize>;

pub type MapCell = HashMap<String, usize>;

pub type Tag = String;

#[derive(Clone, PartialEq, Eq)]
pub struct TagCell {
    pub cell_index: usize,
    pub tag: Tag,
}

#[derive(Clone, PartialEq, Eq)]
pub struct FileCell {
    pub cell_index: usize,
    pub path: Box<Path>,
    pub anchor_keeper: AnchorKeeper,
}

#[derive(Clone, PartialEq, Eq)]
pub struct AnchorCell {
    pub name: String,
    pub cell_index: usize,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum DataCell {
    #[default]
    Null,
    Raw(RawCell),
    String(StringCell),
    List(ListCell),
    Map(MapCell),
    Tag(TagCell),
    File(FileCell),
    TakeAnchor(AnchorCell),
    GetAnchor(AnchorCell),
}

impl DataCell {
    pub(crate) fn get_node_type(&self) -> NodeType {
        match self {
            DataCell::Null => NodeType::Null,
            DataCell::Raw(_) => NodeType::Raw,
            DataCell::String(_) => NodeType::String,
            DataCell::List(_) => NodeType::List,
            DataCell::Map(_) => NodeType::Map,
            DataCell::Tag(_) => NodeType::Tag,
            DataCell::File(_) => NodeType::File,
            DataCell::TakeAnchor(_) => NodeType::TakeAnchor,
            DataCell::GetAnchor(_) => NodeType::GetAnchor,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct MarkedDataCell {
    pub cell: DataCell,
    pub mark: Mark,
}

pub(crate) type Data = HashMap<usize, MarkedDataCell>;
