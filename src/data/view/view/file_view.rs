use super::{
    super::{
        super::{
            node::{file_node::FileNode, Data},
            mark::Mark,
        },
        anchors::Anchors,
    },
    View,
};
use std::{
    fmt::{self, Debug, Formatter},
    path::Path,
};

#[derive(Clone, Copy, Eq)]
pub struct FileView<'data> {
    mark: Mark,
    node: &'data FileNode,
    data: &'data Data,
}

impl<'data> FileView<'data> {
    pub(super) fn new(mark: Mark, node: &'data FileNode, data: &'data Data) -> Self {
        Self { mark, node, data }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn path(&self) -> &'data Path {
        self.node.path.as_path()
    }

    pub fn view(&self) -> View<'data> {
        View::new(self.data.get(self.node.node_index), self.data)
    }

    pub fn anchors(&self) -> Anchors<'data> {
        Anchors::new(self.mark, self.node, self.data)
    }
}

impl<'data> PartialEq for FileView<'data> {
    fn eq(&self, other: &Self) -> bool {
        self.anchors().file_anchors() == other.anchors().file_anchors() && self.view() == other.view()
    }
}

impl<'data> Debug for FileView<'data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileView {{ mark: {:?}, anchors {:?}, view: {:?} }}",
            self.mark,
            self.anchors(),
            self.view()
        )
    }
}
