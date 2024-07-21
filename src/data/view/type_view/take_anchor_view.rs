use super::super::{
    super::{name::NameRef, data::Data, mark::Mark, node::take_anchor_node::TakeAnchorNode},
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use std::fmt::{self, Debug, Formatter};

/// Structure for reading TakeAnchor node data.
#[derive(Clone, Eq)]
pub struct TakeAnchorView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data TakeAnchorNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> TakeAnchorView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data TakeAnchorNode,
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

    /// Gets the name.
    pub fn name(&self) -> NameRef<'data> {
        (&self.node.name).into()
    }

    /// Gets the view on the child node.
    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for TakeAnchorView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.view() == other.view()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for TakeAnchorView<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TakeAnchorView {{ mark: {:?}, name: {:?}, view: {:?} }}",
            self.mark,
            self.name(),
            self.view()
        )
    }
}
