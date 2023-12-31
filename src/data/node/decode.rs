use super::super::{
    error::marked,
    node::{BasicListIter, BasicMapIter, BasicNode},
};
use crate::helpers::to_value::{to_bool, to_number};
use std::error::Error;

pub trait Decode<'a, E: Error + PartialEq + Eq> {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>>
    where
        Self: Sized;
}

macro_rules! impl_number_decode {
	($T:ty) => {
		impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for $T {
			fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
                to_number::<Self>(node.raw()?).ok_or(marked::DecodeError::Failed)
			}
		}
	};
	($($T:ty),*) => { $(impl_number_decode!($T);)* };
}

impl_number_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for bool {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
        to_bool(node.raw()?).ok_or(marked::DecodeError::Failed)
    }
}

impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for &'a str {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
        Ok(node.string()?)
    }
}

impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for BasicListIter<'a, E> {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
        Ok(node.list_iter()?)
    }
}

impl<'a, E: Error + PartialEq + Eq> Decode<'a, E> for BasicMapIter<'a, E> {
    fn decode(node: BasicNode<'a, E>) -> Result<Self, marked::DecodeError<E>> {
        Ok(node.map_iter()?)
    }
}
