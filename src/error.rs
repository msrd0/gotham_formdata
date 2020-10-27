use gotham::anyhow;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
	#[error("The body of the request could not be read")]
	IllegalBody(#[from] gotham::hyper::Error),
	#[error("The 'Content-Type' header could not be parsed: {0}")]
	IllegalContentType(#[from] mime::FromStrError),
	#[error("The 'Content-Type' header could not be parsed: {0}")]
	IllegalContentTypeValue(#[from] gotham::hyper::header::ToStrError),
	#[error("The field {0} could not be parsed: {1}")]
	IllegalField(String, #[source] anyhow::Error),
	#[error("I/O Error while reading body: {0}")]
	IoError(#[from] std::io::Error),
	#[error("The request failed to specify the multipart boundary")]
	MissingBoundary,
	#[error("The request is missing a 'Content-Type' header")]
	MissingContentType,
	#[error("Missing Field '{0}'")]
	MissingField(String),
	#[error("Unknown Field '{0}'")]
	UnknownField(String)
}
