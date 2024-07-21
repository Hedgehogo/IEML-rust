//! This module is designed to describe [`Data`]

use super::{
    node::node::MarkedNode,
    view::{analyse_anchors::AnalyseAnchors, view::View},
};
use std::fmt;

/// Structure intended exclusively for IEML data storage
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub(crate) data: Vec<MarkedNode>,
}

impl Data {
    #[allow(dead_code)]
    pub(crate) fn new<const N: usize>(data: [MarkedNode; N]) -> Self {
        Self {
            data: Vec::from(data),
        }
    }

    pub(crate) fn get(&self, index: usize) -> &MarkedNode {
        self.data
            .get(index)
            .expect("Incorrect document structure, node does not exist.")
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> &mut MarkedNode {
        self.data
            .get_mut(index)
            .expect("Incorrect document structure, node does not exist.")
    }

    /// Gets the view on the top file node.
    pub fn view(&self) -> View {
        View::new(
            self.data
                .last()
                .expect("Incorrect document structure, node does not exist."),
            self,
            (),
        )
    }

    /// Gets the view on the top file node, passing the anchor analyzer to it.
    pub fn view_with_analyse<'data, A: AnalyseAnchors<'data>>(
        &'data self,
        anchor_analyser: A,
    ) -> View<'data, A> {
        View::new(
            self.data
                .last()
                .expect("Incorrect document structure, node does not exist."),
            self,
            anchor_analyser,
        )
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.view())
    }
}
