use super::{
    super::super::{
        cell::{tag_cell::TaggedCell, Data},
        mark::Mark,
    },
    View,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq)]
pub struct TaggedView<'data> {
    mark: Mark,
    cell: &'data TaggedCell,
    data: &'data Data,
}

impl<'data> TaggedView<'data> {
    pub(super) fn new(mark: Mark, cell: &'data TaggedCell, data: &'data Data) -> Self {
        Self { mark, cell, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn tag(&self) -> &'data str {
        self.cell.tag.as_str()
    }

    pub fn view(&self) -> View<'data> {
        View::new(self.data.get(self.cell.cell_index), self.data)
    }
}

impl<'data> PartialEq for TaggedView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.tag() == other.tag() && self.view() == other.view()
    }
}

impl<'data> Debug for TaggedView<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TaggedView {{ mark: {:?}, tag: {:?}, view: {:?} }}",
            self.mark,
            self.tag(),
            self.view()
        )
    }
}
