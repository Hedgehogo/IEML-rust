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

pub(crate) fn equal_list<'a, 'b>(this: &'a ListCell, this_data: &'a Data, other: &'b ListCell, other_data: &'b Data) -> bool {
    let mut other_iter = other.iter();
    this.iter().all(|i| other_iter.next().map_or(false, |j| {
        this_data.get(*i) == other_data.get(*j)
    }))
}

pub(crate) fn equal_map<'a, 'b>(this: &'a MapCell, this_data: &'a Data, other: &'b MapCell, other_data: &'b Data) -> bool {
    if this.len() == other.len() {
        let mut sorted_self = this.iter().collect::<Vec<(&String, &usize)>>();
        sorted_self.sort_by(|i, j| i.0.cmp(&j.0));
        let mut sorted_other = other.iter().collect::<Vec<(&String, &usize)>>();
        sorted_other.sort_by(|i, j| i.0.cmp(&j.0));
        
        let mut other_iter = sorted_other.iter();
        sorted_self.iter().all(|i| other_iter.next().map_or(false, |j| {
            i.0 == i.0 && this_data.get(*i.1) == other_data.get(*j.1)
        }))
    } else {
        false
    }
}

pub(crate) fn equal_tag<'a, 'b>(this: &'a TagCell, this_data: &'a Data, other: &'b TagCell, other_data: &'b Data) -> bool {
    return this.tag == other.tag &&
        this_data.get(this.cell_index) == other_data.get(other.cell_index)
}

pub(crate) fn equal_file<'a, 'b>(this: &'a FileCell, this_data: &'a Data, other: &'b FileCell, other_data: &'b Data) -> bool {
    return this.path == other.path &&
        this.anchors.len() == other.anchors.len() &&
        this_data.get(this.cell_index) == other_data.get(other.cell_index) &&
        equal_map(&this.file_anchors, this_data, &other.file_anchors, other_data)
}

pub(crate) fn equal_take_anchor<'a, 'b>(this: &'a AnchorCell, this_data: &'a Data, other: &'b AnchorCell, other_data: &'b Data) -> bool {
    return this.name == other.name &&
        this_data.get(this.cell_index) == other_data.get(other.cell_index)
}

pub(crate) fn equal_get_anchor(this: &AnchorCell, other: &AnchorCell) -> bool {
    return this.name == other.name
}

pub(crate) fn equal_data<'a, 'b>(this: &'a DataCell, this_data: &'a Data, other: &'b DataCell, other_data: &'b Data) -> bool {
    match (this, other) {
        (DataCell::Null, DataCell::Null) => true,
        (DataCell::Raw(i), DataCell::Raw(j)) => i == j,
        (DataCell::String(i), DataCell::String(j)) => i == j,
        (DataCell::List(i), DataCell::List(j)) => equal_list(i, this_data, j, other_data),
        (DataCell::Map(i), DataCell::Map(j)) => equal_map(i, this_data, j, other_data),
        (DataCell::Tag(i), DataCell::Tag(j)) => equal_tag(i, this_data, j, other_data),
        (DataCell::File(i), DataCell::File(j)) => equal_file(i, this_data, j, other_data),
        (DataCell::TakeAnchor(i), DataCell::TakeAnchor(j)) => equal_take_anchor(i, this_data, j, other_data),
        (DataCell::GetAnchor(i), DataCell::GetAnchor(j)) => equal_get_anchor(i, j),
        _ => false,
    }
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
