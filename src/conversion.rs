/*!
This mod contains conversion traits for common used types, that allows them to be created from
a stream of bytes. Furthermore, it allows every type that implements [FromStr] plus some other
common types to be converted.

**DO NOT IMPLEMENT ANY OF THESE TRAITS MANUALLY!** If you do, it will likely result in compile
errors when the compiler cannot choose which trait to use in the proc-macro generated code.
Instead, if you want to provide a custom conversion method, just implement it as a method for
your type:

```rust
use futures_util::{FutureExt, StreamExt};
use gotham_formdata::{conversion::ConversionFuture, value::{BytesOrString, Value}, FormData};

/// This type parses Base64-encoded values to a [Vec<u8>].
struct Base64(Vec<u8>);

impl Base64 {
	// the method signature needs to be roughly equivalent to this
	async fn convert_value<E>(
			name: &str,
			value: Value<'_, gotham_formdata::Error<E>>
	) -> Result<Self, gotham_formdata::Error<E>>
	where
		E: std::error::Error
	{
		let decoded = match value.value {
			BytesOrString::Bytes(mut stream) => {
				let mut buf: Vec<u8> = Vec::new();
				while let Some(data) = stream.next().await {
					buf.extend_from_slice(&data?);
				}
				base64::decode(&buf)
			},
			BytesOrString::String(string) => base64::decode(string.as_bytes())
		}.map_err(|err| gotham_formdata::Error::IllegalField(name.to_owned(), err.into()))?;

		Ok(Self(decoded))
	}
}

#[derive(FormData)]
struct MyData {
	foo: Base64
}
# mod base64 { pub fn decode(input: &[u8]) -> Result<Vec<u8>, std::convert::Infallible> { unimplemented!() } }
```
*/

use crate::{
	value::{BytesOrString, Value},
	Error
};
use bytes::{Bytes, BytesMut};
use futures_util::{future::FutureExt, stream::StreamExt};
use gotham::anyhow;
use std::{future::Future, pin::Pin, str::FromStr};

/// Re-exports for use in derive macro.
#[doc(hidden)]
pub mod prelude {
	pub use super::{ConvertFromStr, ConvertRawBytes};
}

/// The future returned from conversion methods.
pub type ConversionFuture<'a, T, Err> = Pin<Box<dyn Future<Output = Result<T, Err>> + Send + 'a>>;

/// This trait is used to convert types that implement [FromStr] from a stream of bytes.
///
/// **DO NOT IMPLEMENT MANUALLY!** Look at the [module documentation](self) for an example how
/// to convert custom types.
pub trait ConvertFromStr<Err>: Sized {
	/// Perform the conversion.
	fn convert_value<'a>(name: &'a str, value: Value<'a, Err>) -> ConversionFuture<'a, Self, Err>;
}

impl<E, T> ConvertFromStr<Error<E>> for T
where
	E: std::error::Error,
	T: FromStr,
	T::Err: Into<anyhow::Error>
{
	fn convert_value<'a>(name: &'a str, value: Value<'a, Error<E>>) -> ConversionFuture<'a, Self, Error<E>> {
		async move {
			let buf = match value.value {
				BytesOrString::Bytes(mut stream) => {
					let mut buf = String::new();
					while let Some(data) = stream.next().await {
						let data = data?;
						let str = String::from_utf8_lossy(data.as_ref());
						buf.push_str(&str);
					}
					buf.into()
				},
				BytesOrString::String(buf) => buf
			};

			buf.parse::<Self>()
				.map_err(|err| Error::IllegalField(name.to_owned(), err.into()))
		}
		.boxed()
	}
}

/// This trait is used to convert `Vec<u8>` and similar types from a stream of bytes.
///
/// **DO NOT IMPLEMENT MANUALLY!** Look at the [module documentation](self) for an example how
/// to convert custom types.
pub trait ConvertRawBytes<'a, Err>: Sized {
	/// Perform the conversion.
	fn convert_value(name: &'a str, value: Value<'a, Err>) -> ConversionFuture<'a, Self, Err>;
}

impl<'a, Err: 'a> ConvertRawBytes<'a, Err> for Vec<u8> {
	fn convert_value(_name: &'a str, value: Value<'a, Err>) -> ConversionFuture<'a, Self, Err> {
		async move {
			match value.value {
				BytesOrString::Bytes(mut stream) => {
					let mut buf: Vec<u8> = Vec::new();
					while let Some(data) = stream.next().await {
						buf.extend_from_slice(&data?);
					}
					Ok(buf)
				},
				BytesOrString::String(string) => Ok(string.as_bytes().to_vec())
			}
		}
		.boxed()
	}
}

impl<'a, Err: 'a> ConvertRawBytes<'a, Err> for BytesMut {
	fn convert_value(_name: &'a str, value: Value<'a, Err>) -> ConversionFuture<'a, Self, Err> {
		async move {
			match value.value {
				BytesOrString::Bytes(mut stream) => {
					let mut buf = BytesMut::new();
					while let Some(data) = stream.next().await {
						buf.extend_from_slice(&data?);
					}
					Ok(buf)
				},
				BytesOrString::String(string) => Ok(string.as_bytes().into())
			}
		}
		.boxed()
	}
}

impl<'a, Err: 'a> ConvertRawBytes<'a, Err> for Bytes {
	fn convert_value(name: &'a str, value: Value<'a, Err>) -> ConversionFuture<'a, Self, Err> {
		BytesMut::convert_value(name, value).map(|res| res.map(Into::into)).boxed()
	}
}
