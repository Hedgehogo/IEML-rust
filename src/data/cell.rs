use std::{
    collections::HashMap,
    path::PathBuf,
    fmt::Formatter
};
use super::{
    node::Node,
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
    pub(crate) path: PathBuf,
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

pub(crate) fn equal_list<'a, 'b>(this: (&'a ListCell, &'a Data), other: (&'b ListCell, &'b Data)) -> bool {
    let mut other_iter = other.0.iter();
    this.0.iter().all(|i| other_iter.next().map_or(false, |j| {
        this.1.get(*i) == other.1.get(*j)
    }))
}

pub(crate) fn debug_list<'a, 'b>(this: (&'a ListCell, &'a Data), f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for i in this.0.iter() {
        write!(f, "{:?}, ", Node::new(this.1.get(*i), this.1))?;
    }
    write!(f, "]")
}

pub(crate) fn equal_map<'a, 'b>(this: (&'a MapCell, &'a Data), other: (&'b MapCell, &'b Data)) -> bool {
    if this.0.len() != other.0.len() {
        return false;
    }
    
    this.0.iter().all(|i| other.0.get(i.0).map_or(false, |j| {
        this.1.get(*i.1) == other.1.get(*j)
    }))
}

pub(crate) fn debug_map<'a, 'b>(this: (&'a MapCell, &'a Data), f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{")?;
    for i in this.0.iter() {
        write!(f, "{:?}: {:?}, ", i.0, Node::new(this.1.get(*i.1), this.1))?;
    }
    write!(f, "}}")
}

pub(crate) fn equal_tag<'a, 'b>(this: (&'a TagCell, &'a Data), other: (&'b TagCell, &'b Data)) -> bool {
    return this.0.tag == other.0.tag &&
        this.1.get(this.0.cell_index) == other.1.get(other.0.cell_index)
}

pub(crate) fn debug_tag<'a, 'b>(this: (&'a TagCell, &'a Data), f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "tag: {:?}, cell: {:?}", this.0.tag, Node::new(this.1.get(this.0.cell_index), this.1))
}

pub(crate) fn equal_file<'a, 'b>(this: (&'a FileCell, &'a Data), other: (&'b FileCell, &'b Data)) -> bool {
    return this.0.path == other.0.path &&
        this.0.anchors.len() == other.0.anchors.len() &&
        this.1.get(this.0.cell_index) == other.1.get(other.0.cell_index) &&
        equal_map((&this.0.file_anchors, this.1), (&other.0.file_anchors, other.1))
}

pub(crate) fn debug_file<'a, 'b>(this: (&'a FileCell, &'a Data), f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "file-path: {:?}, anchors: ", this.0.path.clone().into_os_string().into_string())?;
    debug_map((&this.0.file_anchors, this.1), f)?;
    write!(f, "cell: {:?}", Node::new(this.1.get(this.0.cell_index), this.1))
}

pub(crate) fn equal_take_anchor<'a, 'b>(this: (&'a AnchorCell, &'a Data), other: (&'b AnchorCell, &'b Data)) -> bool {
    return this.0.name == other.0.name &&
        this.1.get(this.0.cell_index) == other.1.get(other.0.cell_index)
}

pub(crate) fn debug_take_anchor<'a, 'b>(this: (&'a AnchorCell, &'a Data), f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "name: {:?}, cell: {:?}", this.0.name, Node::new(this.1.get(this.0.cell_index), this.1))
}

pub(crate) fn equal_get_anchor(this: &AnchorCell, other: &AnchorCell) -> bool {
    return this.name == other.name
}

pub(crate) fn debug_get_anchor<'a, 'b>(this: (&'a AnchorCell, &'a Data), f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "name: {:?}", this.0.name)
}

pub(crate) fn equal_data<'a, 'b>(this: (&'a DataCell, &'a Data), other: (&'b DataCell, &'b Data)) -> bool {
    match (this.0, other.0) {
        (DataCell::Null, DataCell::Null) => true,
        (DataCell::Raw(i), DataCell::Raw(j)) => i == j,
        (DataCell::String(i), DataCell::String(j)) => i == j,
        (DataCell::List(i), DataCell::List(j)) => equal_list((i, this.1), (j, other.1)),
        (DataCell::Map(i), DataCell::Map(j)) => equal_map((i, this.1), (j, other.1)),
        (DataCell::Tag(i), DataCell::Tag(j)) => equal_tag((i, this.1), (j, other.1)),
        (DataCell::File(i), DataCell::File(j)) => equal_file((i, this.1), (j, other.1)),
        (DataCell::TakeAnchor(i), DataCell::TakeAnchor(j)) => equal_take_anchor((i, this.1), (j, other.1)),
        (DataCell::GetAnchor(i), DataCell::GetAnchor(j)) => equal_get_anchor(i, j),
        _ => false,
    }
}

pub(crate) fn debug_data<'a, 'b>(this: (&'a DataCell, &'a Data), f: &mut Formatter<'_>) -> std::fmt::Result {
    match this.0 {
        DataCell::Null => write!(f, "Null"),
        DataCell::Raw(i) => write!(f, "Raw({:?})", i),
        DataCell::String(i) => write!(f, "String({:?})", i),
        DataCell::List(i) => {
            write!(f, "List( ")?;
            debug_list((i, this.1), f)?;
            write!(f, " )")
        },
        DataCell::Map(i) => {
            write!(f, "Map( ")?;
            debug_map((i, this.1), f)?;
            write!(f, " )")
        },
        DataCell::Tag(i) => {
            write!(f, "Tag( ")?;
            debug_tag((i, this.1), f)?;
            write!(f, " )")
        },
        DataCell::File(i) => {
            write!(f, "File( ")?;
            debug_file((i, this.1), f)?;
            write!(f, " )")
        },
        DataCell::TakeAnchor(i) => {
            write!(f, "TakeAnchor( ")?;
            debug_take_anchor((i, this.1), f)?;
            write!(f, " )")
        },
        DataCell::GetAnchor(i) => {
            write!(f, "GetAnchor( ")?;
            debug_get_anchor((i, this.1), f)?;
            write!(f, " )")
        },
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
