use super::view::View;

pub struct Tagged {}

pub struct File {}

pub struct TakeAnchor {}

pub struct GetAnchor {}

pub trait Clear<'data> {
    fn clear(view: View<'data>) -> Option<View<'data>>;
}

impl<'data> Clear<'data> for Tagged {
    fn clear(view: View<'data>) -> Option<View<'data>> {
        view.clear_step_tagged()
    }
}

impl<'data> Clear<'data> for File {
    fn clear(view: View<'data>) -> Option<View<'data>> {
        view.clear_step_file()
    }
}

impl<'data> Clear<'data> for TakeAnchor {
    fn clear(view: View<'data>) -> Option<View<'data>> {
        view.clear_step_take_anchor()
    }
}

impl<'data> Clear<'data> for GetAnchor {
    fn clear(view: View<'data>) -> Option<View<'data>> {
        view.clear_step_get_anchor()
    }
}

macro_rules! impl_get_from_step_type {
	($($name:ident)*) => {
		impl<'data, $($name : Clear<'data>),*> Clear<'data> for ($($name, )*) {
			fn clear(_view: View<'data>) -> Option<View<'data>> {
				$(
					if let Some(i) = $name::clear(_view) {
						return Some(i);
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

pub(crate) fn clear_step<'data, T: Clear<'data>>(
    view: View<'data>,
) -> Option<View<'data>> {
    T::clear(view)
}

pub(crate) fn clear<'data, T: Clear<'data>>(
    view: View<'data>,
) -> View<'data> {
    match T::clear(view) {
        Some(i) => clear::<T>(i),
        None => view,
    }
}
