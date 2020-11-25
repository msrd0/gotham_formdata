/*!
This mod contains conversion traits for common used types, that allows them to be created from
a stream of bytes. Furthermore, it allows every type that implements [FromStr] or [From<&[u8]>]
to be converted.
*/

use crate::Error;
use futures_util::{
	future::FutureExt,
	stream::{Stream, StreamExt}
};
use gotham::{anyhow, hyper::body::Bytes};
use std::{future::Future, pin::Pin, str::FromStr};

/// A stream of bytes.
pub type ByteStream<Err> = Pin<Box<dyn Stream<Item = Result<Bytes, Err>> + Send>>;
/// The future returned from conversion methods.
pub type ConversionFuture<'a, T, Err> = Pin<Box<dyn Future<Output = Result<T, Err>> + Send + 'a>>;

/// This trait is used to convert types that implement [FromStr] from a stream of bytes.
pub trait ConvertFromStr<Err>: Sized {
	/// Perform the conversion.
	fn convert_byte_stream<'a>(name: &'a str, stream: ByteStream<Err>) -> ConversionFuture<'a, Self, Err>;
}

impl<E, T> ConvertFromStr<Error<E>> for T
where
	E: std::error::Error,
	T: FromStr,
	T::Err: Into<anyhow::Error>
{
	fn convert_byte_stream<'a>(name: &'a str, mut stream: ByteStream<Error<E>>) -> ConversionFuture<'a, Self, Error<E>> {
		async move {
			let mut buf = String::new();
			while let Some(data) = stream.next().await {
				buf.push_str(::std::str::from_utf8(data?.as_ref())?);
			}

			buf.parse::<Self>()
				.map_err(|err| Error::IllegalField(name.to_owned(), err.into()))
		}
		.boxed()
	}
}
