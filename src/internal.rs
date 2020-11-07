use crate::{Error, FormData};
use gotham::{
	hyper::{
		body,
		header::{HeaderMap, CONTENT_TYPE},
		Body
	},
	state::State
};
use mime::Mime;
use multipart::server::Multipart;
use serde::de::DeserializeOwned;
use std::{
	io::{Cursor, Read},
	sync::Arc
};

pub trait FormDataBuilder: Default {
	type Data: FormData;
	/// The error that can occur during verification.
	type Err: std::error::Error + 'static;

	fn add_entry(&mut self, name: Arc<str>, value: String) -> Result<(), Error<Self::Err>>;
	fn build(self) -> Result<Self::Data, Error<Self::Err>>;
}

pub fn get_content_type<Err: std::error::Error>(state: &State) -> Result<Mime, Error<Err>> {
	let headers: &HeaderMap = state.borrow();
	Ok(headers
		.get(CONTENT_TYPE)
		.ok_or(Error::MissingContentType)?
		.to_str()?
		.parse()?)
}

pub fn get_body(state: &mut State) -> Body {
	state.take()
}

pub fn is_urlencoded(content_type: &Mime) -> bool {
	content_type.essence_str() == "application/x-www-form-urlencoded"
}

pub async fn parse_urlencoded<T: DeserializeOwned, Err: std::error::Error + 'static>(body: Body) -> Result<T, Error<Err>> {
	let body = body::to_bytes(body).await?;
	serde_urlencoded::from_bytes(&body).map_err(Into::into)
}

pub fn is_multipart(content_type: &Mime) -> bool {
	content_type.essence_str() == "multipart/form-data"
}

pub async fn parse_multipart<T: FormDataBuilder>(body: Body, content_type: &Mime) -> Result<T::Data, Error<T::Err>> {
	let boundary = content_type.get_param("boundary").ok_or(Error::MissingBoundary)?.as_str();
	let body = body::to_bytes(body).await?;
	let mut multipart = Multipart::with_body(Cursor::new(body), boundary);

	let mut builder = T::default();
	while let Some(mut field) = multipart.read_entry()? {
		let name = field.headers.name;
		let mut value = String::new();
		field.data.read_to_string(&mut value)?;
		builder.add_entry(name, value)?;
	}
	builder.build()
}
