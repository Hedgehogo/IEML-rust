use std::error::Error;
use super::{
    BasicNode,
    DataCell,
};

pub struct TagData {}

pub struct FileData {}

pub struct TakeAnchorData {}

pub struct GetAnchorData {}

pub trait GetFromStep {
    fn get(cell: &DataCell) -> Option<usize>;
}

impl GetFromStep for TagData {
    fn get(cell: &DataCell) -> Option<usize> {
        if let DataCell::Tag(tag_cell) = cell {
            Some(tag_cell.cell_index)
        } else {
            None
        }
    }
}

impl GetFromStep for FileData {
    fn get(cell: &DataCell) -> Option<usize> {
        if let DataCell::File(file_cell) = cell {
            Some(file_cell.cell_index)
        } else {
            None
        }
    }
}

impl GetFromStep for TakeAnchorData {
    fn get(cell: &DataCell) -> Option<usize> {
        if let DataCell::TakeAnchor(take_anchor_cell) = cell {
            Some(take_anchor_cell.cell_index)
        } else {
            None
        }
    }
}

impl GetFromStep for GetAnchorData {
    fn get(cell: &DataCell) -> Option<usize> {
        if let DataCell::GetAnchor(get_anchor_cell) = cell {
            Some(get_anchor_cell.cell_index)
        } else {
            None
        }
    }
}

pub trait GetFromStepType {
    fn get(cell: &DataCell) -> Option<usize>;
}

macro_rules! impl_get_from_step_type {
	($($name:ident)*) => {
		impl<$($name : GetFromStep),*> GetFromStepType for ($($name, )*) {
			fn get(_cell: &DataCell) -> Option<usize> {
				$(
					if let Some(i) = $name::get(_cell) {
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

pub(crate) fn get_from_step<T: GetFromStepType, E: Error + PartialEq + Eq>(node: BasicNode<E>) -> Option<BasicNode<E>> {
    T::get(node.cell()).map(|i| BasicNode::new(i, node.data))
}

pub(crate) fn get_from<T: GetFromStepType, E: Error + PartialEq + Eq>(node: BasicNode<E>) -> BasicNode<E> {
    match T::get(node.cell()) {
        Some(i) => get_from::<T, E>(BasicNode::new(i, node.data)),
        None => node,
    }
}

