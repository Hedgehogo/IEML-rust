use super::super::mark::Mark;
pub(crate) use super::{
    file_cell::FileCell, get_anchor_cell::GetAnchorCell, list_cell::ListCell, map_cell::MapCell,
    tag_cell::TaggedCell, take_anchor_cell::TakeAnchorCell,
};

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
    Tagged(TaggedCell),
    File(FileCell),
    TakeAnchor(TakeAnchorCell),
    GetAnchor(GetAnchorCell),
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
