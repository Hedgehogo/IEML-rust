//! Type definition [`Data`]

use super::{
    node::node::MarkedNode,
    view::{analyse_anchors::AnalyseAnchors, DocumentView, View},
};
use crate::de::deserialize::{buffer_anchors::BufferAnchors, Deserializer};
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
            .expect("Incorrect structure, the node does not exist.")
    }

    /// Gets the view on the top document node.
    pub fn view(&self) -> View {
        self.view_with_analyse(())
    }

    /// Gets the view on the top document node, passing the anchor analyzer to it.
    pub fn view_with_analyse<'data, A: AnalyseAnchors<'data>>(
        &'data self,
        anchor_analyser: A,
    ) -> View<'data, A> {
        View::new(
            self.data
                .last()
                .expect("Incorrect structure, the node does not exist."),
            self,
            anchor_analyser,
        )
    }

    /// Gets the document view on the top document node.
    pub fn document_view(&self) -> DocumentView<'_, ()> {
        self.document_view_with_analyse(())
    }

    /// Gets the document view on the top document node, passing the anchor analyzer to it.
    pub fn document_view_with_analyse<'data, A: AnalyseAnchors<'data>>(
        &'data self,
        anchor_analyser: A,
    ) -> DocumentView<'data, A> {
        self.view_with_analyse(anchor_analyser)
            .document()
            .expect("Incorrect structure, the top node is not a document.")
    }

    /// Gets the deserializer, passing the anchor analyzer to it.
    pub fn deserializer(&self) -> Deserializer<()> {
        self.deserializer_with_bufferiser(())
    }

    /// Gets the deserializer, passing the anchor analyzer to it.
    pub fn deserializer_with_bufferiser<'data, B: BufferAnchors<'data>>(
        &'data self,
        anchor_bufferiser: B,
    ) -> Deserializer<'data, B> {
        Deserializer::new(self.view_with_analyse(anchor_bufferiser))
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.view())
    }
}
