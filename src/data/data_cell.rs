use std::{
    collections::HashMap,
    path::Path,
};
use super::{
    node_type::NodeType,
    mark::Mark,
};

pub(crate) type RawCell = String;

pub(crate) type StringCell = String;

pub(crate) type ListCell = Vec<usize>;

pub(crate) type MapCell = HashMap<String, usize>;

pub(crate) type Tag = String;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct TagCell {
    pub(crate) cell_index: usize,
    pub(crate) tag: Tag,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct FileCell {
    pub(crate) cell_index: usize,
    pub(crate) path: Box<Path>,
    pub(crate) anchors: HashMap<String, usize>,
    pub(crate) file_anchors: HashMap<String, usize>,
    pub(crate) parent: Option<usize>,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct AnchorCell {
    pub name: String,
    pub cell_index: usize,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub(crate) enum DataCell {
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
pub(crate) struct MarkedDataCell {
    pub cell: DataCell,
    pub mark: Mark,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub(crate) data: HashMap<usize, MarkedDataCell>,
}

impl Data {
    pub(crate) fn get(&self, index: usize) -> &MarkedDataCell {
        self.data.get(&index).expect("Incorrect document structure, Cell does not exist.")
    }
}

impl<const N: usize> From<[(usize, MarkedDataCell); N]> for Data {
    fn from(arr: [(usize, MarkedDataCell); N]) -> Self {
        Self { data: HashMap::<usize, MarkedDataCell>::from(arr)}
    }
}
