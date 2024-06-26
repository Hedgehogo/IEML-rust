use super::{
    super::{error::marked, view::node::Node},
    node::list_node::ListNode,
    node::map_node::MapNode,
};
use crate::parse::utils::to_value::{to_bool, to_number};
use std::error::Error;

pub trait Deserialize<'data, E: Error + PartialEq + Eq> {
    fn decode(node: Node<'data>) -> Result<Self, marked::DeserializeError<E>>
    where
        Self: Sized;
}

macro_rules! impl_number_decode {
	($T:ty) => {
		impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for $T {
			fn decode(node: Node<'data>) -> Result<Self, marked::DeserializeError<E>> {
                to_number::<Self>(node.raw()?.raw()).ok_or(marked::DeserializeError::Failed)
			}
		}
	};
	($($T:ty),*) => { $(impl_number_decode!($T);)* };
}

impl_number_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for bool {
    fn decode(node: Node<'data>) -> Result<Self, marked::DeserializeError<E>> {
        to_bool(node.raw()?.raw()).ok_or(marked::DeserializeError::Failed)
    }
}

impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for &'data str {
    fn decode(node: Node<'data>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(node.string()?.string())
    }
}

impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for ListNode<'data> {
    fn decode(node: Node<'data>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(node.list()?)
    }
}

impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for MapNode<'data> {
    fn decode(node: Node<'data>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(node.map()?)
    }
}
