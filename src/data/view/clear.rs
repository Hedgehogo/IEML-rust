use super::view::View;
use super::analyse_anchors::AnalyseAnchors;

pub struct Tagged {}

pub struct File {}

pub struct TakeAnchor {}

pub struct GetAnchor {}

pub trait Clear<'data, A: AnalyseAnchors<'data>> {
    fn clear(view: View<'data, A>) -> Option<View<'data, A>>;
}

impl<'data, A: AnalyseAnchors<'data>> Clear<'data, A> for Tagged {
    fn clear(view: View<'data, A>) -> Option<View<'data, A>> {
        view.clear_step_tagged()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Clear<'data, A> for File {
    fn clear(view: View<'data, A>) -> Option<View<'data, A>> {
        view.clear_step_file()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Clear<'data, A> for TakeAnchor {
    fn clear(view: View<'data, A>) -> Option<View<'data, A>> {
        view.clear_step_take_anchor()
    }
}

impl<'data, A: AnalyseAnchors<'data>> Clear<'data, A> for GetAnchor {
    fn clear(view: View<'data, A>) -> Option<View<'data, A>> {
        view.clear_step_get_anchor()
    }
}

macro_rules! impl_get_from_step_type {
	($($name:ident)*) => {
		impl<'data, A: AnalyseAnchors<'data>, $($name : Clear<'data, A>),*> Clear<'data, A> for ($($name, )*) {
			fn clear(_view: View<'data, A>) -> Option<View<'data, A>> {
				$(
					if let Some(i) = $name::clear(_view.clone()) {
						return Some(i);
					}
				)*
				None
			}
		}
	}
}

impl_get_from_step_type!();
impl_get_from_step_type!(A1);
impl_get_from_step_type!(A1 A2);
impl_get_from_step_type!(A1 A2 A3);
impl_get_from_step_type!(A1 A2 A3 A4);

pub(crate) fn clear_step<'data, T: Clear<'data, A>, A: AnalyseAnchors<'data>>(
    view: View<'data, A>,
) -> Option<View<'data, A>> {
    T::clear(view)
}

pub(crate) fn clear<'data, T: Clear<'data, A>, A: AnalyseAnchors<'data>>(
    view: View<'data, A>,
) -> View<'data, A> {
    match T::clear(view.clone()) {
        Some(i) => clear::<T, A>(i),
        None => view,
    }
}
