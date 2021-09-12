use crate::{
	validate::Validator,
	value::{BytesOrString, Value},
	Error, FormData
};
use futures_util::stream::TryStreamExt;
use gotham::hyper::{
	body::{self, Body},
	header::{HeaderMap, CONTENT_TYPE}
};
use mime::{Mime, APPLICATION_WWW_FORM_URLENCODED, BOUNDARY, MULTIPART_FORM_DATA};
use multer::Multipart;
use std::{borrow::Cow, future::Future, pin::Pin};

pub use futures_util::{FutureExt, StreamExt};
pub use gotham::{hyper::body::Bytes, state::State};

#[cfg(feature = "regex-validation")]
pub use regex::Regex;
#[cfg(feature = "regex-validation")]
pub type LazyRegex = once_cell::sync::Lazy<Regex>;

pub fn assert_validator<V: Validator<T>, T: ?Sized>(_: &V) {}

pub type FormDataBuilderFuture<'a, Err> = Pin<Box<dyn Future<Output = Result<(), Error<Err>>> + Send + 'a>>;

pub trait FormDataBuilder: Default {
	type Data: FormData;
	/// The error that can occur during verification.
	type Err: std::error::Error + 'static;

	fn add_entry<'a>(
		&'a mut self,
		name: Cow<'a, str>,
		value: Value<'a, Error<Self::Err>>
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

pub async fn parse<T: FormDataBuilder>(body: Body, content_type: Mime) -> Result<T::Data, Error<T::Err>> {
	if is_urlencoded(&content_type) {
		parse_urlencoded::<T>(body).await
	} else if is_multipart(&content_type) {
		parse_multipart::<T>(body, &content_type).await
	} else {
		Err(Error::UnknownContentType(content_type))
	}
}

fn is_urlencoded(content_type: &Mime) -> bool {
	content_type.essence_str() == APPLICATION_WWW_FORM_URLENCODED.as_ref()
}

async fn parse_urlencoded<T: FormDataBuilder>(body: Body) -> Result<T::Data, Error<T::Err>> {
	let body = body::to_bytes(body).await?;

	let mut builder = T::default();
	for (name, value) in form_urlencoded::parse(&body) {
		let value = Value {
			value: BytesOrString::String(value),
			content_type: None
		};
		builder.add_entry(name, value).await?;
	}
	builder.build()
}

fn is_multipart(content_type: &Mime) -> bool {
	content_type.essence_str() == MULTIPART_FORM_DATA.as_ref()
}

async fn parse_multipart<T: FormDataBuilder>(body: Body, content_type: &Mime) -> Result<T::Data, Error<T::Err>> {
	let boundary = content_type.get_param(BOUNDARY).ok_or(Error::MissingBoundary)?.as_str();
	let mut multipart = Multipart::new(body, boundary);

	let mut builder = T::default();
	while let Some(field) = multipart.next_field().await? {
		let name = field.name().ok_or(Error::MissingContentDisposition)?.to_owned();
		let mime = field.content_type().cloned();

		let stream = field.map_err(Into::into).boxed();
		let value = Value {
			value: BytesOrString::Bytes(stream),
			content_type: mime
		};
		builder.add_entry(name.into(), value).await?;
	}
	builder.build()
}
