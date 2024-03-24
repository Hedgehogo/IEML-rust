use super::{
    super::{mark::Mark, node_type::NodeType},
    Data,
};
pub(crate) use super::{
    file_cell::FileCell, get_anchor_cell::GetAnchorCell, list_cell::ListCell, map_cell::MapCell,
    tag_cell::TagCell, take_anchor_cell::TakeAnchorCell,
};
use std::fmt::{self, Formatter};

pub(crate) type RawCell = String;
pub(crate) type StringCell = String;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum DataCell {
    #[default]
    Null,
    Raw(RawCell),
    String(StringCell),
    List(ListCell),
    Map(MapCell),
    Tag(TagCell),
    File(FileCell),
    TakeAnchor(TakeAnchorCell),
    GetAnchor(GetAnchorCell),
}

impl DataCell {
    pub(crate) fn equal<'this, 'other>(
        this: (&'this Self, &'this Data),
        other: (&'other Self, &'other Data),
    ) -> bool {
        match (this.0, other.0) {
            (DataCell::Null, DataCell::Null) => true,
            (DataCell::Raw(i), DataCell::Raw(j)) => i == j,
            (DataCell::String(i), DataCell::String(j)) => i == j,
            (DataCell::List(i), DataCell::List(j)) => ListCell::equal((i, this.1), (j, other.1)),
            (DataCell::Map(i), DataCell::Map(j)) => MapCell::equal((i, this.1), (j, other.1)),
            (DataCell::Tag(i), DataCell::Tag(j)) => TagCell::equal((i, this.1), (j, other.1)),
            (DataCell::File(i), DataCell::File(j)) => FileCell::equal((i, this.1), (j, other.1)),
            (DataCell::TakeAnchor(i), DataCell::TakeAnchor(j)) => {
                TakeAnchorCell::equal((i, this.1), (j, other.1))
            }
            (DataCell::GetAnchor(i), DataCell::GetAnchor(j)) => i.equal(j),
            _ => false,
        }
    }

    pub(crate) fn debug<'this>(
        this: (&'this Self, &'this Data),
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match this.0 {
            DataCell::Null => write!(f, "Null"),
            DataCell::Raw(i) => write!(f, "Raw({:?})", i),
            DataCell::String(i) => write!(f, "String({:?})", i),
            DataCell::List(i) => {
                write!(f, "List( ")?;
                ListCell::debug((i, this.1), f)?;
                write!(f, " )")
            }
            DataCell::Map(i) => {
                write!(f, "Map( ")?;
                MapCell::debug((i, this.1), f)?;
                write!(f, " )")
            }
            DataCell::Tag(i) => {
                write!(f, "Tag( ")?;
                TagCell::debug((i, this.1), f)?;
                write!(f, " )")
            }
            DataCell::File(i) => {
                write!(f, "File( ")?;
                FileCell::debug((i, this.1), f)?;
                write!(f, " )")
            }
            DataCell::TakeAnchor(i) => {
                write!(f, "TakeAnchor( ")?;
                TakeAnchorCell::debug((i, this.1), f)?;
                write!(f, " )")
            }
            DataCell::GetAnchor(i) => {
                write!(f, "GetAnchor( ")?;
                i.debug(f)?;
                write!(f, " )")
            }
        }
    }

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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MarkedDataCell {
    pub cell: DataCell,
    pub mark: Mark,
}

impl MarkedDataCell {
    pub fn new(cell: DataCell, mark: Mark) -> Self {
        Self { cell, mark }
    }
}
