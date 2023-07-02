use crate::helpers::to_value::{to_bool, to_number};
use super::super::{
    error::marked,
    node::{Node, ListIter, MapIter},
    data_cell::{StringCell},
};

pub trait Decode<'a> {
    fn decode(node: Node<'a>) -> Result<Self, marked::DecodeError>
    where
        Self: Sized;
}

macro_rules! impl_number_decode {
	($T:ty) => {
		impl<'a> Decode<'a> for $T {
			fn decode(node: Node<'a>) -> Result<Self, marked::DecodeError> {
                to_number::<Self>(node.get_raw()?).ok_or(marked::DecodeError::Failed)
			}
		}
	};
	($($T:ty),*) => { $(impl_number_decode!($T);)* };
}

impl_number_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl<'a> Decode<'a> for bool {
    fn decode(node: Node<'a>) -> Result<Self, marked::DecodeError> {
        to_bool(node.get_raw()?).ok_or(marked::DecodeError::Failed)
    }
}

impl<'a> Decode<'a> for &'a StringCell {
    fn decode(node: Node<'a>) -> Result<Self, marked::DecodeError> {
        Ok(node.get_string()?)
    }
}

impl<'a> Decode<'a> for ListIter<'a> {
    fn decode(node: Node<'a>) -> Result<Self, marked::DecodeError> {
        Ok(node.get_list_iter()?)
    }
}

impl<'a> Decode<'a> for MapIter<'a> {
    fn decode(node: Node<'a>) -> Result<Self, marked::DecodeError> {
        Ok(node.get_map_iter()?)
    }
}

