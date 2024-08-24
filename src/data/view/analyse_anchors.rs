//! Trait definition [`AnalyseAnchors`]

/// Trait allowing you to track the anchor organization and save some data about it.
///
/// The analyzer is supposed to be a reference data structure to some buffer.
///
/// For example, this allows you to deserialize anchor data only once, and then only take a reference to already created structures.
pub trait AnalyseAnchors<'data>: Clone {
    /// Saying that a view to the child document has been received, this is important for anchors as each document creates its own scope for them.
    fn child(&self, path: &'data str) -> Self;

    /// Gets the parent analyzer if it exists.
    fn parent(&self) -> Option<Self>;
}

impl<'data> AnalyseAnchors<'data> for () {
    fn child(#[allow(unused)] &self, _path: &'data str) -> Self {}

    fn parent(#[allow(unused)] &self) -> Option<Self> {
        Some(())
    }
}
