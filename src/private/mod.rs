use crate::{value::BytesOrString, Error, FormData};
use futures_util::{future::BoxFuture, stream::TryStreamExt};
use gotham::{
	anyhow,
	hyper::{
		body::{self, Body},
		header::{HeaderMap, CONTENT_TYPE}
	}
};
use mime::{Mime, APPLICATION_WWW_FORM_URLENCODED, BOUNDARY, MULTIPART_FORM_DATA};
use multer::Multipart;
use serde::de::DeserializeOwned;
use std::{borrow::Cow, fmt::Display, future::Future, pin::Pin};

pub use futures_util::{FutureExt, StreamExt};
pub use gotham::{hyper::body::Bytes, state::State};
pub use log::debug;
pub use validator::Validate;

mod deserializer;
use deserializer::Deserializer;

pub type FormDataBuilderFuture<'a> = Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type Value<'a, E = Error> = crate::value::Value<'a, E>;

pub trait FormDataBuilder: Default {
	type Data: FormData;

	fn add_entry<'a>(&'a mut self, name: Cow<'a, str>, value: Value<'a, Error>) -> FormDataBuilderFuture<'a>;
	fn build(self) -> Result<Self::Data, Error>;
}

pub fn get_content_type(state: &State) -> Result<Mime, Error> {
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

pub async fn parse<T: FormDataBuilder>(body: Body, content_type: Mime) -> Result<T::Data, Error> {
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

async fn parse_urlencoded<T: FormDataBuilder>(body: Body) -> Result<T::Data, Error> {
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

async fn parse_multipart<T: FormDataBuilder>(body: Body, content_type: &Mime) -> Result<T::Data, Error> {
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

pub trait Parse<T> {
	type Err: Into<anyhow::Error>;
	type Fut: Future<Output = Result<T, Self::Err>> + Send;

	fn parse(self) -> Self::Fut;
}

impl<'de, T, Err> Parse<T> for Value<'de, Err>
where
	T: DeserializeOwned,
	Err: Display + 'static
{
	type Err = deserializer::Error;
	type Fut = BoxFuture<'de, Result<T, Self::Err>>;

	fn parse(self) -> Self::Fut {
		async move {
			let mut deserializer = match self.value {
				BytesOrString::Bytes(mut stream) => {
					let mut buf: Vec<u8> = Vec::new();
					while let Some(data) = stream.next().await {
						buf.extend_from_slice(&data.map_err(|e| deserializer::Error(e.to_string()))?);
					}
					Deserializer::Bytes(buf)
				},
				BytesOrString::String(s) => Deserializer::Text(s)
			};
			T::deserialize(&mut deserializer)
		}
		.boxed()
	}
}
