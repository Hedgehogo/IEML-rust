//! Type definition [`ToMatchView`]

use super::analyse_anchors::AnalyseAnchors;
use std::fmt::Debug;

pub use super::type_view::{
    anchor_view::AnchorView, document_view::DocumentView, list_view::ListView, map_view::MapView,
    null_view::NullView, raw_view::RawView, string_view::StringView, tagged_view::TaggedView,
};

/// A structure designed for pattern-matching, which the regular [`View`][`super::view::View`] does not allow.
#[derive(Clone, Eq)]
pub enum ToMatchView<'data, A: AnalyseAnchors<'data>> {
    Null(NullView),
    Raw(RawView<'data>),
    String(StringView<'data>),
    List(ListView<'data, A>),
    Map(MapView<'data, A>),
    Tagged(TaggedView<'data, A>),
    Document(DocumentView<'data, A>),
    Anchor(AnchorView<'data, A>),
}

impl<'data, A: AnalyseAnchors<'data>> Debug for ToMatchView<'data, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToMatchView::Null(i) => write!(f, "Null({:?})", i),
            ToMatchView::Raw(i) => write!(f, "Raw({:?})", i),
            ToMatchView::String(i) => write!(f, "String({:?})", i),
            ToMatchView::List(i) => write!(f, "List({:?})", i),
            ToMatchView::Map(i) => write!(f, "Map({:?})", i),
            ToMatchView::Tagged(i) => write!(f, "Tagged({:?})", i),
            ToMatchView::Document(i) => write!(f, "Document({:?})", i),
            ToMatchView::Anchor(i) => write!(f, "Anchor({:?})", i),
        }
    }
}

impl<'data, A: AnalyseAnchors<'data>> PartialEq for ToMatchView<'data, A> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ToMatchView::Null(i), ToMatchView::Null(j)) => i == j,
            (ToMatchView::Raw(i), ToMatchView::Raw(j)) => i == j,
            (ToMatchView::String(i), ToMatchView::String(j)) => i == j,
            (ToMatchView::List(i), ToMatchView::List(j)) => i == j,
            (ToMatchView::Map(i), ToMatchView::Map(j)) => i == j,
            (ToMatchView::Tagged(i), ToMatchView::Tagged(j)) => i == j,
            (ToMatchView::Document(i), ToMatchView::Document(j)) => i == j,
            (ToMatchView::Anchor(i), ToMatchView::Anchor(j)) => i == j,
            _ => false,
        }
    }
}
