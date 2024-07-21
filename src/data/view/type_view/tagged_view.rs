use super::super::{
    super::{name::NameRef, data::Data, mark::Mark, node::tag_node::TaggedNode},
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use std::fmt::{self, Debug, Formatter};

/// Structure for reading Tagged node data.
#[derive(Clone, Eq)]
pub struct TaggedView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data TaggedNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> TaggedView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data TaggedNode,
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

    /// Gets the mark.
    pub fn mark(&self) -> Mark {
        self.mark
    }

    /// Gets the tag.
    pub fn tag(&self) -> NameRef<'data> {
        (&self.node.tag).into()
    }

    /// Gets the view on the child node.
    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for TaggedView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        self.tag() == other.tag() && self.view() == other.view()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for TaggedView<'data, A> {
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
