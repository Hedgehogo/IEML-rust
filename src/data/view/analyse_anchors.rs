use std::path::Path;

pub trait AnalyseAnchors<'data>: Clone {
    fn child(&self, path: &'data Path) -> Self;

    fn parent(&self) -> Option<Self>;
}

impl<'data> AnalyseAnchors<'data> for () {
    fn child(#[allow(unused)] &self, _path: &'data Path) -> Self {}

    fn parent(#[allow(unused)] &self) -> Option<Self> {
        Some(())
    }
}
