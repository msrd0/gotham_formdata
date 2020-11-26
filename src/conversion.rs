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

/// Re-exports for use in derive macro.
#[doc(hidden)]
pub mod prelude {
	pub use super::{ConvertFromStr, ConvertRawBytes};
}

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
				let data = data?;
				let str = String::from_utf8_lossy(data.as_ref());
				buf.push_str(&str);
			}

			buf.parse::<Self>()
				.map_err(|err| Error::IllegalField(name.to_owned(), err.into()))
		}
		.boxed()
	}
}

/// This trait is used to convert `Vec<u8>` and similar types from a stream of bytes.
pub trait ConvertRawBytes<'a, Err>: Sized {
	/// Perform the conversion.
	fn convert_byte_stream(name: &'a str, stream: ByteStream<Err>) -> ConversionFuture<'a, Self, Err>;
}

impl<'a, E: 'a> ConvertRawBytes<'a, E> for Vec<u8> {
	fn convert_byte_stream(_name: &'a str, mut stream: ByteStream<E>) -> ConversionFuture<'a, Self, E> {
		async move {
			let mut buf: Vec<u8> = Vec::new();
			while let Some(data) = stream.next().await {
				buf.extend_from_slice(&data?);
			}
			Ok(buf)
		}
		.boxed()
	}
}
