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

fn with_body(body: &'static [u8], content_type: Mime, callback: impl Fn(&mut State)) {
	State::with_new(|state| {
		let body: Body = body.into();
		state.put(body);

		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, content_type.to_string().parse().unwrap());
		state.put(headers);

		callback(state);
	});
}

#[test]
fn test_string() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		foo: String
	}

	with_body(
		b"foo=%F0%9F%9A%A2+DONAUDAMPFSCHIFFFAHRTSKAPIT%C3%84NSM%C3%9CTZE+%F0%9F%91%AE",
		APPLICATION_WWW_FORM_URLENCODED,
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
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		foo: Vec<u8>
	}

	with_body(
		b"foo=%F0%9F%9A%A2+DONAUDAMPFSCHIFFFAHRTSKAPIT%C3%84NSM%C3%9CTZE+%F0%9F%91%AE",
		APPLICATION_WWW_FORM_URLENCODED,
		|state| {
			let data = block_on(Data::parse_form_data(state)).unwrap();
			assert_eq!(data, Data {
				foo: "🚢 DONAUDAMPFSCHIFFFAHRTSKAPITÄNSMÜTZE 👮".as_bytes().to_owned()
			});
		}
	);
}
