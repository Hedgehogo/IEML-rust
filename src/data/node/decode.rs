use super::super::{
	error::marked,
	node::Node,
	node_data::{StringData, ListData, MapData},
};

pub trait Decode<'a> {
	fn decode(node: &'a Node<'a>) -> Result<Self, marked::DecodeError> where Self: Sized + 'a;
}

macro_rules! impl_number_decode {
	($T:ty) => {
		impl<'a> Decode<'a> for $T {
			fn decode(node: &'a Node<'a>) -> Result<Self, marked::DecodeError> {
				todo!()
			}
		}
	};
	($($T:ty),*) => { $(impl_number_decode!($T);)* };
}

impl_number_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl<'a> Decode<'a> for bool {
	fn decode(node: &'a Node<'a>) -> Result<Self, marked::DecodeError> {
		todo!()
	}
}

impl<'a> Decode<'a> for &'a StringData {
	fn decode(node: &'a Node<'a>) -> Result<Self, marked::DecodeError> {
		Ok(node.get_string()?)
	}
}

impl<'a> Decode<'a> for &'a ListData<'a> {
	fn decode(node: &'a Node<'a>) -> Result<Self, marked::DecodeError> {
		Ok(node.get_list()?)
	}
}

impl<'a> Decode<'a> for &'a MapData<'a> {
	fn decode(node: &'a Node<'a>) -> Result<Self, marked::DecodeError> {
		Ok(node.get_map()?)
	}
}

