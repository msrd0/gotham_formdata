use futures_executor::block_on;
use gotham::{
	hyper::{
		header::{HeaderMap, CONTENT_TYPE},
		Body
	},
	state::State
};
use gotham_formdata::{Error, FormData};
use mime::{Mime, APPLICATION_WWW_FORM_URLENCODED};
use serde::Deserialize;

fn custom_validator(value: &u8) -> Result<(), &'static str> {
	if *value >= 128 {
		return Err("Value out of range");
	}
	Ok(())
}

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
fn validate_custom_validator() {
	#[derive(Debug, Deserialize, FormData, PartialEq)]
	struct Data {
		#[validate(validator = "custom_validator")]
		data: u8
	}

	with_body(b"data=1", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: 1 })
	});

	with_body(b"data=128", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}
