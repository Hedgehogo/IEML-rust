use super::{
    super::{error::marked, view::view::View},
    view::list_view::ListView,
    view::map_view::MapView,
};
use crate::parse::utils::to_value::{to_bool, to_number};
use std::error::Error;

pub trait Deserialize<'data, E: Error + PartialEq + Eq> {
    fn decode(view: View<'data>) -> Result<Self, marked::DeserializeError<E>>
    where
        Self: Sized;
}

macro_rules! impl_number_decode {
	($T:ty) => {
		impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for $T {
			fn decode(view: View<'data>) -> Result<Self, marked::DeserializeError<E>> {
                to_number::<Self>(view.raw()?.raw()).ok_or(marked::DeserializeError::Failed)
			}
		}
	};
	($($T:ty),*) => { $(impl_number_decode!($T);)* };
}

impl_number_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for bool {
    fn decode(view: View<'data>) -> Result<Self, marked::DeserializeError<E>> {
        to_bool(view.raw()?.raw()).ok_or(marked::DeserializeError::Failed)
    }
}

impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for &'data str {
    fn decode(view: View<'data>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(view.string()?.string())
    }
}

impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for ListView<'data> {
    fn decode(view: View<'data>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(view.list()?)
    }
}

impl<'data, E: Error + PartialEq + Eq> Deserialize<'data, E> for MapView<'data> {
    fn decode(view: View<'data>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(view.map()?)
    }
}
