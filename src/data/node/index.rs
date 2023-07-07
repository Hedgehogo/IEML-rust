use std::error::Error;
use super::super::error::marked;
use super::BasicNode;

pub trait NodeIndex<'a, E: Error + PartialEq + Eq> {
    type Error;
    
    fn at(node: BasicNode<'a, E>, index: Self) -> Result<BasicNode<'a, E>, Self::Error>;
}

impl<'a, E: Error + PartialEq + Eq> NodeIndex<'a, E> for usize {
    type Error = marked::ListError;
    
    fn at(node: BasicNode<'a, E>, index: Self) -> Result<BasicNode<'a, E>, Self::Error> {
        node.at_index(index)
    }
}

impl<'a, E: Error + PartialEq + Eq> NodeIndex<'a, E> for &str {
    type Error = marked::MapError;
    
    fn at(node: BasicNode<'a, E>, index: Self) -> Result<BasicNode<'a, E>, Self::Error> {
        node.at_key(index)
    }
}