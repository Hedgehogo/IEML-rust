use super::{
    super::error::marked,
    analyse_anchors::AnalyseAnchors,
    type_view::{list_view::ListView, map_view::MapView},
    view::View,
};
use crate::parse::utils::to_value::{to_bool, to_number};
use std::error::Error;

pub trait Deserialize<'data, A: AnalyseAnchors<'data>, E: Error + PartialEq + Eq> {
    fn deserialize(view: View<'data, A>) -> Result<Self, marked::DeserializeError<E>>
    where
        Self: Sized;
}

macro_rules! impl_number_decode {
	($T:ty) => {
		impl<'data, A: AnalyseAnchors<'data>, E: Error + PartialEq + Eq> Deserialize<'data, A, E> for $T {
			fn deserialize(view: View<'data, A>) -> Result<Self, marked::DeserializeError<E>> {
                to_number::<Self>(view.raw()?.raw()).ok_or(marked::DeserializeError::Failed)
			}
		}
	};
	($($T:ty),*) => { $(impl_number_decode!($T);)* };
}

impl_number_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl<'data, A: AnalyseAnchors<'data>, E: Error + PartialEq + Eq> Deserialize<'data, A, E> for bool {
    fn deserialize(view: View<'data, A>) -> Result<Self, marked::DeserializeError<E>> {
        to_bool(view.raw()?.raw()).ok_or(marked::DeserializeError::Failed)
    }
}

impl<'data, A: AnalyseAnchors<'data>, E: Error + PartialEq + Eq> Deserialize<'data, A, E>
    for &'data str
{
    fn deserialize(view: View<'data, A>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(view.string()?.string())
    }
}

impl<'data, A: AnalyseAnchors<'data>, E: Error + PartialEq + Eq> Deserialize<'data, A, E>
    for ListView<'data, A>
{
    fn deserialize(view: View<'data, A>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(view.list()?)
    }
}

impl<'data, A: AnalyseAnchors<'data>, E: Error + PartialEq + Eq> Deserialize<'data, A, E>
    for MapView<'data, A>
{
    fn deserialize(view: View<'data, A>) -> Result<Self, marked::DeserializeError<E>> {
        Ok(view.map()?)
    }
}
