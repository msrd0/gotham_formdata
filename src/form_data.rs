use gotham::state::State;
use std::{future::Future, pin::Pin};

#[allow(type_alias_bounds)]
pub type FormDataFuture<T: FormData> = Pin<Box<dyn Future<Output = Result<T, T::Err>> + Send>>;

pub trait FormData: Sized {
	type Err;

	fn parse_form_data(state: &mut State) -> FormDataFuture<Self>;
}

pub trait FormDataFromState {
	fn parse_form_data<T: FormData>(&mut self) -> FormDataFuture<T>;
}

impl FormDataFromState for State {
	fn parse_form_data<T: FormData>(&mut self) -> FormDataFuture<T> {
		T::parse_form_data(self)
	}
}
