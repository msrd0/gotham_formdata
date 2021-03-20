<!-- alpha code warning with spacing above -->
<br/>
<div align="center">
	:warning: <span color="#ffcc4d">This crate is still alpha. APIs are subject to change.</span> :warning:
</div>

<!-- headline -->
<div align="center">
	<h1>gotham_formdata :writing_hand:</h1>
</div>

<!-- badges -->
<div align="center">
	<a href="https://crates.io/crates/gotham_formdata">
        <img alt="crates.io" src="https://img.shields.io/crates/v/gotham_formdata.svg"/>
    </a>
	<a href="https://docs.rs/crate/gotham_formdata">
        <img alt="docs.rs" src="https://docs.rs/gotham_formdata/badge.svg"/>
    </a>
	<a href="https://msrd0.github.io/gotham_formdata/doc/gotham_formdata/index.html">
		<img alt="docs for master" src="https://img.shields.io/badge/docs-master-blue.svg"/>
	</a>
    <a href="https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html">
        <img alt="Rust 1.45+" src="https://img.shields.io/badge/rustc-1.45+-orange.svg"/>
    </a>
	<a href="https://www.apache.org/licenses/LICENSE-2.0">
		<img alt="License Apache-2.0" src="https://img.shields.io/badge/license-Apache--2.0-blue.svg"/>
	</a>
</div>
<div align="center">
	<a href="https://github.com/msrd0/gotham_formdata/actions?query=workflow%3ARust">
		<img alt="GitHub Actions" src="https://github.com/msrd0/gotham_formdata/workflows/Rust/badge.svg"/>
	</a>
	<a href="https://msrd0.github.io/gotham_formdata/tarpaulin-report.html">
		<img alt="Coverage Report" src="https://msrd0.github.io/gotham_formdata/coverage.svg"/>
	</a>
	<a href="https://deps.rs/repo/github/msrd0/gotham_formdata">
		<img alt="dependencies" src="https://deps.rs/repo/github/msrd0/gotham_formdata/status.svg"/>
	</a>
</div>
<br/>

This crate is an extension to the popular [gotham web framework][gotham] for Rust. It aims to
reduce boilerplate necessary to read request bodies today as a stop-gap until gotham finally
implements a [body extractor].

## :sparkles: Features

- Parse `application/x-www-form-urlencoded` request bodies
- Parse `multipart/form-data` request bodies
- Verify the parsed request body
- `#![forbid(unsafe_code)]` ensures that all functionality is implemented in 100% safe Rust code

## :warning: Warning

This crate is asynchronous, but does not yet enforce uploads limits. **YOU ARE RESPONSIBLE
FOR ENFORCING UPLOAD LIMITS.**

## :spiral_notepad: Example

```rust
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

## :label: Versioning

Like all rust crates, this crate will follow semantic versioning guidelines. However, changing
the MSRV (minimum supported rust version) is not considered a breaking change.

## :page_with_curl: License

```
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
