use std::fmt::Debug;

pub use super::type_view::{
    file_view::FileView, get_anchor_view::GetAnchorView, list_view::ListView, map_view::MapView,
    null_view::NullView, raw_view::RawView, string_view::StringView, tagged_view::TaggedView,
    take_anchor_view::TakeAnchorView,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToMatchView<'data> {
    Null(NullView),
    Raw(RawView<'data>),
    String(StringView<'data>),
    List(ListView<'data>),
    Map(MapView<'data>),
    Tagged(TaggedView<'data>),
    File(FileView<'data>),
    TakeAnchor(TakeAnchorView<'data>),
    GetAnchor(GetAnchorView<'data>),
}
