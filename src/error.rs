use gotham::anyhow;
use mime::Mime;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
/// This error type is used when parsing form data from a request body was unsuccessful.
pub enum Error<Err: std::error::Error + 'static> {
	/// The body of the request could not be read.
	#[error("The body of the request could not be read")]
	IllegalBody(#[from] gotham::hyper::Error),
	/// The content type of the body was not a valid mime type.
	#[error("The 'Content-Type' header could not be parsed: {0}")]
	IllegalContentType(#[from] mime::FromStrError),
	/// The content type of the body contained unreadable bytes.
	#[error("The 'Content-Type' header could not be parsed: {0}")]
	IllegalContentTypeValue(#[from] gotham::hyper::header::ToStrError),
	/// The value of a field could not be parsed into that field's type.
	#[error("The field {0} could not be parsed: {1}")]
	IllegalField(String, #[source] anyhow::Error),
	/// The body was parsed but contained data that did not pass validation.
	#[error("The body contained invalid data: {0}")]
	InvalidData(#[source] Err),
	/// An I/O error occured while reading the body.
	#[error("I/O Error while reading body: {0}")]
	IoError(#[from] std::io::Error),
	/// The body was a multipart body but did not specify a boundary.
	#[error("The request failed to specify the multipart boundary")]
	MissingBoundary,
	/// The request did not specify a content type.
	#[error("The request is missing a 'Content-Type' header")]
	MissingContentType,
	/// The body is missing a required field.
	#[error("Missing Field '{0}'")]
	MissingField(String),
	/// The body's content type is not supported.
	#[error("Unknown 'Content-Type' header value: {0}")]
	UnknownContentType(Mime),
	/// The body contained a field that was not expected.
	#[error("Unknown Field '{0}'")]
	UnknownField(String)
}
