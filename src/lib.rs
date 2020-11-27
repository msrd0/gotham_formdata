/*!
This crate is an extension to the popular [gotham web framework][gotham] for Rust. It aims to
reduce boilerplate necessary to read request bodies today as a stop-gap until gotham finally
implements a [body extractor].

# :sparkles: Features

- Parse `application/x-www-form-urlencoded` request bodies
- Parse `multipart/form-data` request bodies
- Verify the parsed request body

# :warning: Warning

This crate is asynchronous, but does not yet enforce uploads limits. **YOU ARE RESPONSIBLE
FOR ENFORCING UPLOAD LIMITS.**

# :spiral_notepad: Example

```rust
# use gotham::handler::HandlerError;
# use gotham::helpers::http::response::*;
# use gotham::hyper::{Body, Response, StatusCode};
# use gotham::state::State;
# use mime::TEXT_PLAIN;
use gotham_formdata::FormData;

#[derive(FormData)]
struct LoginData {
	#[validate(regex = "[a-zA-Z0-9_]", error = "The username contains illegal characters.")]
	username: String,
	#[validate(min_length = 6)]
	password: String
}

async fn login_handler(state: &mut State) -> Result<Response<Body>, HandlerError> {
	let login_data: LoginData = FormData::parse_form_data(state).await?;
	Ok(if login_data.password == "secret" {
		create_response(state, StatusCode::OK, TEXT_PLAIN, login_data.username)
	} else {
		create_empty_response(state, StatusCode::FORBIDDEN)
	})
}
```

# :label: Versioning

Like all rust crates, this crate will follow semantic versioning guidelines. However, changing
the MSRV (minimum supported rust version) is not considered a breaking change.

# :page_with_curl: License

```text
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

 [body extractor]: https://github.com/gotham-rs/gotham/issues/11
 [gotham]: https://github.com/gotham-rs/gotham
 [multipart]: https://crates.io/crates/multipart
*/
#![warn(missing_docs, rust_2018_idioms)]
#![deny(missing_debug_implementations, unreachable_pub)]

#[doc(hidden)]
/// Not public API.
pub mod export {
	pub use futures_util::{future::FutureExt, stream::StreamExt};
	pub use gotham::{hyper::body::Bytes, state::State};
	pub use log;
}

#[doc(inline)]
pub use gotham_formdata_derive::*;

pub mod conversion;

mod error;
pub use error::*;

mod form_data;
pub use form_data::*;

#[doc(hidden)]
/// Not public API.
pub mod internal;

pub mod validate;

pub mod value;
