use super::super::{
    super::{data::Data, mark::Mark, node::get_anchor_node::GetAnchorNode},
    analyse_anchors::AnalyseAnchors,
    view::View,
};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Eq)]
pub struct GetAnchorView<'data, A: AnalyseAnchors<'data>> {
    mark: Mark,
    node: &'data GetAnchorNode,
    data: &'data Data,
    anchor_analyser: A,
}

impl<'data, A: AnalyseAnchors<'data>> GetAnchorView<'data, A> {
    pub(in super::super) fn new(
        mark: Mark,
        node: &'data GetAnchorNode,
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

    pub fn name(&self) -> &'data str {
        self.node.name.as_str()
    }

    pub fn view(&self) -> View<'data, A> {
        let node = self.data.get(self.node.node_index);
        View::new(node, self.data, self.anchor_analyser.clone())
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for GetAnchorView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Debug for GetAnchorView<'data, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GetAnchorView {{ mark: {:?}, name: {:?} }}",
            self.mark,
            self.name()
        )
    }
}
