use super::{super::mark::Mark, Data};
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
            (DataCell::Tagged(i), DataCell::Tagged(j)) => {
                TaggedCell::equal((i, this.1), (j, other.1))
            }
            (DataCell::File(i), DataCell::File(j)) => FileCell::equal((i, this.1), (j, other.1)),
            (DataCell::TakeAnchor(i), DataCell::TakeAnchor(j)) => {
                TakeAnchorCell::equal((i, this.1), (j, other.1))
            }
            (DataCell::GetAnchor(i), DataCell::GetAnchor(j)) => i.equal(j),
            _ => false,
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
