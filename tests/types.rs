use futures_executor::block_on;
use gotham::{
	hyper::{
		body::Body,
		header::{HeaderMap, CONTENT_TYPE}
	},
	state::State
};
use gotham_formdata::FormData;
use mime::{Mime, APPLICATION_WWW_FORM_URLENCODED};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use validator::Validate;

fn with_body(body: &[u8], content_type: Mime, callback: impl Fn(&mut State)) {
	State::with_new(|state| {
		let body: Body = body.to_owned().into();
		state.put(body);

		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, content_type.to_string().parse().unwrap());
		state.put(headers);

		callback(state);
	});
}

fn with_body_foo(foo: &[u8], callback: impl Fn(&mut State)) {
	let urlencoded = format!("foo={}", percent_encode(foo, NON_ALPHANUMERIC));
	with_body(urlencoded.as_bytes(), APPLICATION_WWW_FORM_URLENCODED, &callback);

	let mut multipart = Vec::new();
	multipart.extend_from_slice(b"--GOTHAM-MULTIPART-BOUNDARY\r\nContent-Disposition: form-data; name=\"foo\"\r\n\r\n");
	multipart.extend_from_slice(foo);
	multipart.extend_from_slice(b"\r\n--GOTHAM-MULTIPART-BOUNDARY--");
	let mime = "multipart/form-data; boundary=GOTHAM-MULTIPART-BOUNDARY";
	with_body(&multipart, mime.parse().unwrap(), &callback);
}

#[test]
fn test_string() {
	#[derive(Debug, FormData, PartialEq, Validate)]
	struct Data {
		foo: String
	}

	with_body_foo(
		b"\xF0\x9F\x9A\xA2 DONAUDAMPFSCHIFFFAHRTSKAPIT\xC3\x84NSM\xC3\x9CTZE \xF0\x9F\x91\xAE",
		|state| {
			let data = block_on(Data::parse_form_data(state)).unwrap();
			assert_eq!(data, Data {
				foo: "🚢 DONAUDAMPFSCHIFFFAHRTSKAPITÄNSMÜTZE 👮".to_owned()
			});
		}
	);
}

#[test]
fn test_vec_u8() {
	#[derive(Debug, FormData, PartialEq, Validate)]
	struct Data {
		foo: Vec<u8>
	}

	with_body_foo(
		b"\xF0\x9F\x9A\xA2 DONAUDAMPFSCHIFFFAHRTSKAPIT\xC3\x84NSM\xC3\x9CTZE \xF0\x9F\x91\xAE",
		|state| {
			let data = block_on(Data::parse_form_data(state)).unwrap();
			assert_eq!(data, Data {
				foo: "🚢 DONAUDAMPFSCHIFFFAHRTSKAPITÄNSMÜTZE 👮".as_bytes().to_owned()
			});
		}
	);
}
