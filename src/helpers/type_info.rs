use std::{
	any::type_name,
	fmt::Debug,
	marker::PhantomData,
};
use std::fmt::Formatter;

struct TypeInfoHelper<T> {
	_phantom_data: PhantomData<T>,
}

impl<T> Debug for TypeInfoHelper<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "TypeInfo<{}> {{}}", type_name::<T>())
	}
}

impl<T> TypeInfoHelper<T> {
	fn new() -> Self {
		Self{_phantom_data: Default::default()}
	}
}

pub trait TypeInfo: Debug {
	fn get_name(&self) -> &'static str {
		type_name::<Self>()
	}
}

impl<T> TypeInfo for TypeInfoHelper<T> {
	fn get_name(&self) -> &'static str {
		type_name::<T>()
	}
}

pub fn make_type_info<T: 'static>() -> Box<dyn TypeInfo> {
	Box::new(TypeInfoHelper::<T>::new())
}