/*!
This mod defines the [Value] used for parsing the form data.
*/

use bytes::Bytes;
use futures_util::stream::Stream;
use std::{
	borrow::Cow,
	fmt::{self, Debug},
	pin::Pin
};

pub use mime::Mime;

/// A stream of bytes.
pub type ByteStream<Err> = Pin<Box<dyn Stream<Item = Result<Bytes, Err>> + Send>>;

/// Either a stream of bytes or a string.
pub enum BytesOrString<'a, Err> {
	/// Byte stream.
	Bytes(ByteStream<Err>),
	/// String.
	String(Cow<'a, str>)
}

impl<'a, Err> Debug for BytesOrString<'a, Err> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Bytes(_) => write!(f, "BytesOrString::Bytes(...)"),
			Self::String(_) => write!(f, "BytesOrString::String(...)")
		}
	}
}

/// A value, either a string parsed from and urlencoded form, or bytes from a multipart body.
#[derive(Debug)]
pub struct Value<'a, Err> {
	/// The value of the field.
	pub value: BytesOrString<'a, Err>,
	/// The content-type of the field, if any.
	pub content_type: Option<Mime>
}
