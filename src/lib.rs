/*!
This crate is an extension to the popular [gotham web framework][gotham] for Rust. It aims to reduce
boilerplate necessary to read `multipart/form-data` requests as a stop-gap until gotham finally
implements a [body extractor].

# Warning

This crate is synchronous. It does load the entire request body into memory. **DO NOT USE IN
PRODUCTION UNLESS YOU ENFORCE YOUR OWN UPLOAD LIMIT TO AVOID SERIOUS SECURITY VULNERABILITIES
IN YOUR SOFTWARE.**

For the same reason, file uploads are not supported, and this won't change unless this crate uses
an async multipart parser.

# Example

```rust
# use gotham::anyhow::Result;
# use gotham::hyper::{Body, Response, StatusCode};
# use gotham::state::State;
# use gotham::helpers::http::response::*;
# use mime::TEXT_PLAIN;
use gotham_formdata::FormData;

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

# License

```text
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

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
#![warn(rust_2018_idioms)]
#![deny(missing_debug_implementations, unreachable_pub)]

#[doc(hidden)]
pub mod export {
	pub use futures_util::future::FutureExt;
	pub use gotham::state::State;
	pub use log;
}

#[doc(inline)]
pub use gotham_formdata_derive::*;

mod error;
pub use error::*;

mod form_data;
pub use form_data::*;

#[doc(hidden)]
pub mod internal;
