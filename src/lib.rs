/*!
This crate is an extension to the popular [gotham web framework][gotham] for Rust. It aims to reduce
boilerplate necessary to read `multipart/form-data` requests as a stop-gap until gotham finally
implements a [body extractor].

Unfortunately, this crate uses the synchronous [multipart] crate, and does therefore not make the
best use of modern `async`/`.await` support.

# Example

```rust
# use gotham::anyhow::Result;
# use gotham::hyper::{Body, Response, StatusCode};
# use gotham::state::State;
# use gotham::helpers::http::response::*;
# use mime::TEXT_PLAIN;
use gotham_multipart::FormData;

#[derive(FormData)]
struct LoginData {
	username: String,
	password: String
}

async fn login_handler(state: &mut State) -> Result<Response<Body>> {
	let login_data: LoginData = FormData::parse_form_data(state).await?;
	Ok(if login_data.password == "secret" {
		create_response(state, StatusCode::OK, TEXT_PLAIN, login_data.username)
	} else {
		create_empty_response(state, StatusCode::FORBIDDEN)
	})
}
```

 [body extractor]: https://github.com/gotham-rs/gotham/issues/11
 [gotham]: https://github.com/gotham-rs/gotham
 [multipart]: https://crates.io/crates/multipart
*/
#![warn(rust_2018_idioms)]
#![deny(missing_debug_implementations, unreachable_pub)]

#[doc(hidden)]
pub mod export {
	pub use gotham::state::State;
	pub use futures_util::future::FutureExt;
}

#[doc(inline)]
pub use gotham_multipart_derive::*;

mod error;
pub use error::*;

mod form_data;
pub use form_data::*;

#[doc(hidden)]
pub mod internal;
