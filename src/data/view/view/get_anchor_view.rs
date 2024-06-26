use super::{
    super::super::{
        cell::{get_anchor_cell::GetAnchorCell, Data},
        mark::Mark,
    },
    View,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct GetAnchorView<'data> {
    mark: Mark,
    cell: &'data GetAnchorCell,
    data: &'data Data,
}

impl<'data> GetAnchorView<'data> {
    pub(super) fn new(mark: Mark, cell: &'data GetAnchorCell, data: &'data Data) -> Self {
        Self { mark, cell, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn name(&self) -> &'data str {
        self.cell.name.as_str()
    }

    pub fn view(&self) -> View<'data> {
        View::new(self.data.get(self.cell.cell_index), self.data)
    }
}

impl<'data> PartialEq for GetAnchorView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl<'data> Debug for GetAnchorView<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GetAnchorView {{ mark: {:?}, name: {:?} }}",
            self.mark,
            self.name()
        )
    }
}
