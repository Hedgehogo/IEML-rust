use super::BasicNode;
use std::error::Error;

pub struct Tag {}

pub struct File {}

pub struct TakeAnchor {}

pub struct GetAnchor {}

pub trait ClearStep<E: Error + PartialEq + Eq> {
    fn clear(node: BasicNode<E>) -> Option<BasicNode<E>>;
}

impl<E: Error + PartialEq + Eq> ClearStep<E> for Tag {
    fn clear(node: BasicNode<E>) -> Option<BasicNode<E>> {
        node.clear_step_tag()
    }
}

impl<E: Error + PartialEq + Eq> ClearStep<E> for File {
    fn clear(node: BasicNode<E>) -> Option<BasicNode<E>> {
        node.clear_step_file()
    }
}

impl<E: Error + PartialEq + Eq> ClearStep<E> for TakeAnchor {
    fn clear(node: BasicNode<E>) -> Option<BasicNode<E>> {
        node.clear_step_take_anchor()
    }
}

impl<E: Error + PartialEq + Eq> ClearStep<E> for GetAnchor {
    fn clear(node: BasicNode<E>) -> Option<BasicNode<E>> {
        node.clear_step_get_anchor()
    }
}

pub trait ClearStepType<E: Error + PartialEq + Eq> {
    fn clear(node: BasicNode<E>) -> Option<BasicNode<E>>;
}

macro_rules! impl_get_from_step_type {
	($($name:ident)*) => {
		impl<E: Error + PartialEq + Eq, $($name : ClearStep<E>),*> ClearStepType<E> for ($($name, )*) {
			fn clear(_node: BasicNode<E>) -> Option<BasicNode<E>> {
				$(
					if let Some(i) = $name::clear(_node) {
						return Some(i)
					}
				)*
				None
			}
		}
	}
}

impl_get_from_step_type!();
impl_get_from_step_type!(A);
impl_get_from_step_type!(A B);
impl_get_from_step_type!(A B C);
impl_get_from_step_type!(A B C D);

pub(crate) fn clear_step<E: Error + PartialEq + Eq, T: ClearStepType<E>>(
    node: BasicNode<E>,
) -> Option<BasicNode<E>> {
    T::clear(node)
}

pub(crate) fn clear<E: Error + PartialEq + Eq, T: ClearStepType<E>>(
    node: BasicNode<E>,
) -> BasicNode<E> {
    match T::clear(node) {
        Some(i) => clear::<E, T>(i),
        None => node,
    }
}
