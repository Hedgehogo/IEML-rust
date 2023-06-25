use super::{
	Node,
	NodeData,
};

pub struct TagData {}
pub struct FileData {}
pub struct TakeAnchorData {}
pub struct GetAnchorData {}

pub trait GetFromStep {
	fn get<'a>(data: &'a NodeData<'a>) -> Option<&'a Node<'a>>;
}

impl GetFromStep for TagData {
	fn get<'a>(data: &'a NodeData<'a>) -> Option<&'a Node<'a>> {
		if let NodeData::Tag(tag_data) = data {
			Some(&*tag_data.data)
		} else {
			None
		}
	}
}

impl GetFromStep for FileData {
	fn get<'a>(data: &'a NodeData<'a>) -> Option<&'a Node<'a>> {
		if let NodeData::File(file_data) = data {
			Some(&*file_data.data)
		} else {
			None
		}
	}
}

impl GetFromStep for TakeAnchorData {
	fn get<'a>(data: &'a NodeData<'a>) -> Option<&'a Node<'a>> {
		if let NodeData::TakeAnchor(take_anchor_data) = data {
			take_anchor_data.keeper.get(&take_anchor_data.name)
		} else {
			None
		}
	}
}

impl GetFromStep for GetAnchorData {
	fn get<'a>(data: &'a NodeData<'a>) -> Option<&'a Node<'a>> {
		if let NodeData::GetAnchor(get_anchor_data) = data {
			get_anchor_data.keeper.get(&get_anchor_data.name)
		} else {
			None
		}
	}
}

pub trait GetFromStepType {
	fn get<'a>(data: &'a NodeData<'a>) -> Option<&'a Node<'a>>;
}

macro_rules! impl_get_from_step_type {
	($($name:ident)*) => {
		impl<$($name : GetFromStep),*> GetFromStepType for ($($name),*) {
			fn get<'a>(data: &'a NodeData<'a>) -> Option<&'a Node<'a>> {
				$(
					if let Some(i) = $name::get(data) {
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

pub(crate) fn get_from_step<'a, T: GetFromStepType>(data: &'a NodeData<'a>) -> Option<&'a Node<'a>> {
	return T::get(data)
}

pub(crate) fn get_from<'a, T: GetFromStepType>(data: &'a Node<'a>) -> &'a Node<'a> {
	if let Some(i) = T::get(&data.data) {
		return get_from::<T>(i);
	}
	data
}

