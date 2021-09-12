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
use validator::Validate;

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
fn validate_min_length() {
	#[derive(Debug, FormData, PartialEq, Validate)]
	struct Data {
		#[validate(length(min = 8))]
		data: String
	}

	with_body(b"data=verylong", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data {
			data: "verylong".to_owned()
		})
	});

	with_body(b"data=shorter", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state));
		assert!(matches!(data, Err(Error::InvalidData(_))));
	});
}

#[test]
fn validate_max_length() {
	#[derive(Debug, FormData, PartialEq, Validate)]
	struct Data {
		#[validate(length(max = 7))]
		data: String
	}

	with_body(b"data=shorter", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data {
			data: "shorter".to_owned()
		})
	});

	with_body(b"data=verylong", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state));
		assert!(matches!(data, Err(Error::InvalidData(_))));
	});
}

#[test]
fn validate_min() {
	#[derive(Debug, FormData, PartialEq, Validate)]
	struct Data {
		#[validate(range(min = 10))]
		data: u64
	}

	with_body(b"data=10", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: 10 })
	});

	with_body(b"data=9", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state));
		assert!(matches!(data, Err(Error::InvalidData(_))));
	});
}

#[test]
fn validate_max() {
	#[derive(Debug, FormData, PartialEq, Validate)]
	struct Data {
		#[validate(range(max = 10))]
		data: u64
	}

	with_body(b"data=10", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: 10 })
	});

	with_body(b"data=11", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state));
		assert!(matches!(data, Err(Error::InvalidData(_))));
	});
}
