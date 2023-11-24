use super::super::{
    error::marked,
    node::{BasicListIter, BasicMapIter, BasicNode},
};
use crate::helpers::to_value::{to_bool, to_number};

pub trait Decode<'a> {
    fn decode(node: BasicNode<'a>) -> Result<Self, marked::DecodeError>
    where
        Self: Sized;
}

macro_rules! impl_number_decode {
	($T:ty) => {
		impl<'a> Decode<'a> for $T {
			fn decode(node: BasicNode<'a>) -> Result<Self, marked::DecodeError> {
                to_number::<Self>(node.raw()?).ok_or(marked::DecodeError::Failed)
			}
		}
	};
	($($T:ty),*) => { $(impl_number_decode!($T);)* };
}

impl_number_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl<'a> Decode<'a> for bool {
    fn decode(node: BasicNode<'a>) -> Result<Self, marked::DecodeError> {
        to_bool(node.raw()?).ok_or(marked::DecodeError::Failed)
    }
}

impl<'a> Decode<'a> for &'a str {
    fn decode(node: BasicNode<'a>) -> Result<Self, marked::DecodeError> {
        Ok(node.string()?)
    }
}

impl<'a> Decode<'a> for BasicListIter<'a> {
    fn decode(node: BasicNode<'a>) -> Result<Self, marked::DecodeError> {
        Ok(node.list_iter()?)
    }
}

impl<'a> Decode<'a> for BasicMapIter<'a> {
    fn decode(node: BasicNode<'a>) -> Result<Self, marked::DecodeError> {
        Ok(node.map_iter()?)
    }
}
