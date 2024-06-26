use super::super::{
    super::{data::Data, mark::Mark, node::file_node::FileNode},
    analyse_anchors::AnalyseAnchors,
    anchors::Anchors,
    view::View,
};
use std::{
    fmt::{self, Debug, Formatter},
    path::Path,
};

#[derive(Clone, Eq)]
pub struct FileView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data FileNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> FileView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data FileNode,
        data: &'data Data,
        anchor_analyser: A,
    ) -> Self {
        Self {
            mark,
            node,
            data,
            anchor_analyser,
        }
    }

    pub fn mark(&self) -> Mark {
        self.mark
    }

    pub fn path(&self) -> &'data Path {
        self.node.path.as_path()
    }

    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }

    pub fn anchors(&self) -> Anchors<'data, A> {
        let anchor_analyser = self.anchor_analyser.clone();
        Anchors::new(self.mark, self.node, self.data, anchor_analyser)
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for FileView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        self.anchors().file_anchors() == other.anchors().file_anchors()
            && self.view() == other.view()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for FileView<'data, A> {
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
