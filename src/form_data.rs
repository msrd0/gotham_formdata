use gotham::state::State;
use std::{future::Future, pin::Pin};

/// This is the return type of [FormData::parse_form_data].
#[allow(type_alias_bounds)]
pub type FormDataFuture<T: FormData> =
	Pin<Box<dyn Future<Output = Result<T, T::Err>> + Send>>;

/**
This is the trait implemented by `#[derive(FormData)]`. It provides a method to parse the struct
it is implemented for to be parsed from the request body contained in gotham's state.

You usually don't implement this trait directly, use the derive macro instead.
*/
pub trait FormData: Sized {
	/// The error type returned when parsing the request body was unsuccessful.
	type Err;

	/// Parse the struct from the request body contained in gotham's state.
	fn parse_form_data(state: &mut State) -> FormDataFuture<Self>;
}

/**
This is the equivalent of [FormData] from the state's perspective. Use this if you prefer
`state.parse_form_data::<MyData>()?` over `MyData::parse_form_data(&mut state)?`.
*/
pub trait FormDataFromState {
	/// Parse `T` from the request body contained in this state.
	fn parse_form_data<T: FormData>(&mut self) -> FormDataFuture<T>;
}

impl FormDataFromState for State {
	fn parse_form_data<T: FormData>(&mut self) -> FormDataFuture<T> {
		T::parse_form_data(self)
	}
}
