use crate::Error;
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
use std::io::{Cursor, Read};

pub fn get_content_type(state: &State) -> Result<Mime, Error> {
	let headers: &HeaderMap = state.borrow();
	Ok(headers
		.get(CONTENT_TYPE)
		.ok_or(Error::MissingContentType)?
		.to_str()?
		.parse()?)
}

pub fn get_boundary(content_type: &Mime) -> Result<&str, Error> {
	Ok(content_type.get_param("boundary").ok_or(Error::MissingBoundary)?.as_str())
}

pub fn get_body(state: &mut State) -> Body {
	state.take()
}

pub async fn get_multipart(body: Body, boundary: &str) -> Result<Multipart<impl Read>, Error> {
	let body = body::to_bytes(body).await?;
	log::debug!("Parsing body {:?} with boundary '{}'", body, boundary);
	Ok(Multipart::with_body(Cursor::new(body), boundary))
}
