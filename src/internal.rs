use crate::{validate::Validator, Error, FormData};
use futures_util::stream::{self, Stream, StreamExt, TryStreamExt};
use gotham::{
	hyper::{
		body::{self, Body, Bytes},
		header::{HeaderMap, CONTENT_TYPE}
	},
	state::State
};
use mime::{Mime, APPLICATION_WWW_FORM_URLENCODED, BOUNDARY, MULTIPART_FORM_DATA};
use multer::Multipart;
use std::{borrow::Cow, future::Future, pin::Pin};

pub fn assert_validator<V: Validator<T>, T: ?Sized>(_: &V) {}

pub type FormDataValue<Err> = Pin<Box<dyn Stream<Item = Result<Bytes, Error<Err>>> + Send>>;
pub type FormDataBuilderFuture<'a, Err> = Pin<Box<dyn Future<Output = Result<(), Error<Err>>> + Send + 'a>>;

pub trait FormDataBuilder: Default {
	type Data: FormData;
	/// The error that can occur during verification.
	type Err: std::error::Error + 'static;

	fn add_entry<'a>(
		&'a mut self,
		name: Cow<'a, str>,
		value: FormDataValue<Self::Err>
	) -> FormDataBuilderFuture<'a, Self::Err>;
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
	content_type.essence_str() == APPLICATION_WWW_FORM_URLENCODED.as_ref()
}

pub async fn parse_urlencoded<T: FormDataBuilder>(body: Body) -> Result<T::Data, Error<T::Err>> {
	let body = body::to_bytes(body).await?;

	let mut builder = T::default();
	for (name, value) in form_urlencoded::parse(&body) {
		let bytes = Bytes::copy_from_slice(value.as_bytes());
		builder
			.add_entry(name, stream::once(async move { Ok(bytes) }).boxed())
			.await?;
	}
	builder.build()
}

pub fn is_multipart(content_type: &Mime) -> bool {
	content_type.essence_str() == MULTIPART_FORM_DATA.as_ref()
}

pub async fn parse_multipart<T: FormDataBuilder>(body: Body, content_type: &Mime) -> Result<T::Data, Error<T::Err>> {
	let boundary = content_type.get_param(BOUNDARY).ok_or(Error::MissingBoundary)?.as_str();
	let mut multipart = Multipart::new(body, boundary);

	let mut builder = T::default();
	while let Some(field) = multipart.next_field().await? {
		let name = field.name().ok_or(Error::MissingContentDisposition)?;
		// unfortunately, we need to clone the name so we can move field which is our only way
		// of accessing the stream
		let name = name.to_owned();

		let value = field.map_err(Into::into);
		builder.add_entry(name.into(), value.boxed()).await?;
	}
	builder.build()
}
