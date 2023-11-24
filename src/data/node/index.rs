use super::super::error::marked;
use super::BasicNode;

pub trait NodeIndex<'a> {
    type Error;

    fn at(node: BasicNode<'a>, index: Self) -> Result<BasicNode<'a>, Self::Error>;
}

impl<'a> NodeIndex<'a> for usize {
    type Error = marked::ListError;

    fn at(node: BasicNode<'a>, index: Self) -> Result<BasicNode<'a>, Self::Error> {
        node.at_index(index)
    }
}

impl<'a> NodeIndex<'a> for &str {
    type Error = marked::MapError;

    fn at(node: BasicNode<'a>, index: Self) -> Result<BasicNode<'a>, Self::Error> {
        node.at_key(index)
    }
}
