use super::super::{
    super::{name::Name, data::Data, mark::Mark, node::anchor_creation_node::AnchorCreationNode},
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use std::fmt::{self, Debug, Formatter};

/// Structure for reading AnchorCreation node data.
#[derive(Clone, Eq)]
pub struct AnchorCreationView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data AnchorCreationNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> AnchorCreationView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data AnchorCreationNode,
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
    pub fn name(&self) -> Name<&'data str> {
        (&self.node.name).into()
    }

    /// Gets the view on the child node.
    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for AnchorCreationView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.view() == other.view()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for AnchorCreationView<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AnchorCreationView {{ mark: {:?}, name: {:?}, view: {:?} }}",
            self.mark,
            self.name(),
            self.view()
        )
    }
}
