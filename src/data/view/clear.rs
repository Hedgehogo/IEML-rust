use super::node::Node;

pub struct Tagged {}

pub struct File {}

pub struct TakeAnchor {}

pub struct GetAnchor {}

pub trait Clear<'data> {
    fn clear(node: Node<'data>) -> Option<Node<'data>>;
}

impl<'data> Clear<'data> for Tagged {
    fn clear(node: Node<'data>) -> Option<Node<'data>> {
        node.clear_step_tagged()
    }
}

impl<'data> Clear<'data> for File {
    fn clear(node: Node<'data>) -> Option<Node<'data>> {
        node.clear_step_file()
    }
}

impl<'data> Clear<'data> for TakeAnchor {
    fn clear(node: Node<'data>) -> Option<Node<'data>> {
        node.clear_step_take_anchor()
    }
}

impl<'data> Clear<'data> for GetAnchor {
    fn clear(node: Node<'data>) -> Option<Node<'data>> {
        node.clear_step_get_anchor()
    }
}

macro_rules! impl_get_from_step_type {
	($($name:ident)*) => {
		impl<'data, $($name : Clear<'data>),*> Clear<'data> for ($($name, )*) {
			fn clear(_node: Node<'data>) -> Option<Node<'data>> {
				$(
					if let Some(i) = $name::clear(_node) {
						return Some(i);
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

pub(crate) fn clear_step<'data, T: Clear<'data>>(
    node: Node<'data>,
) -> Option<Node<'data>> {
    T::clear(node)
}

pub(crate) fn clear<'data, T: Clear<'data>>(
    node: Node<'data>,
) -> Node<'data> {
    match T::clear(node) {
        Some(i) => clear::<T>(i),
        None => node,
    }
}
