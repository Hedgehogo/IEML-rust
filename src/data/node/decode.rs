use std::error::Error;
use crate::helpers::to_value::{to_bool, to_number};
use super::super::{
    error::marked,
    node::{BasicNode, BasicListIter, BasicMapIter},
    data_cell::{StringCell},
};

pub trait Decode<'a, E: Error + PartialEq + Eq> {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>>
    where
        Self: Sized;
}

macro_rules! impl_number_decode {
	($T:ty) => {
		impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for $T {
			fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
                to_number::<Self>(node.get_raw()?).ok_or(marked::DecodeError::Failed)
			}
		}
	};
	($($T:ty),*) => { $(impl_number_decode!($T);)* };
}

impl_number_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for bool {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
        to_bool(node.get_raw()?).ok_or(marked::DecodeError::Failed)
    }
}

impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for &'a StringCell {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
        Ok(node.get_string()?)
    }
}

impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for BasicListIter<'a, E> {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
        Ok(node.get_list_iter()?)
    }
}

impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for BasicMapIter<'a, E> {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
        Ok(node.get_map_iter()?)
    }
}

