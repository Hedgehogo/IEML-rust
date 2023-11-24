use super::BasicNode;

pub struct Tag {}

pub struct File {}

pub struct TakeAnchor {}

pub struct GetAnchor {}

pub trait ClearStep {
    fn clear(node: BasicNode) -> Option<BasicNode>;
}

impl ClearStep for Tag {
    fn clear(node: BasicNode) -> Option<BasicNode> {
        node.clear_step_tag()
    }
}

impl ClearStep for File {
    fn clear(node: BasicNode) -> Option<BasicNode> {
        node.clear_step_file()
    }
}

impl ClearStep for TakeAnchor {
    fn clear(node: BasicNode) -> Option<BasicNode> {
        node.clear_step_take_anchor()
    }
}

impl ClearStep for GetAnchor {
    fn clear(node: BasicNode) -> Option<BasicNode> {
        node.clear_step_get_anchor()
    }
}

pub trait ClearStepType {
    fn clear(node: BasicNode) -> Option<BasicNode>;
}

macro_rules! impl_get_from_step_type {
	($($name:ident)*) => {
		impl<$($name : ClearStep),*> ClearStepType for ($($name, )*) {
			fn clear(_node: BasicNode) -> Option<BasicNode> {
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

pub(crate) fn clear_step<T: ClearStepType>(node: BasicNode) -> Option<BasicNode> {
    T::clear(node)
}

pub(crate) fn clear<T: ClearStepType>(node: BasicNode) -> BasicNode {
    match T::clear(node) {
        Some(i) => clear::<T>(i),
        None => node,
    }
}
