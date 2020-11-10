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
use std::convert::Infallible;

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
fn validate_custom_error() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(min = 1, error = "Too small.")]
		data: u8
	}

	// testing that we can define `.validate()` eventhough that name is also used in the derive code
	impl Data {
		fn validate(&self) -> bool {
			true
		}
	}

	with_body(b"data=1", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert!(data.validate()); // NOTE this is defined above, not generated
		assert_eq!(data, Data { data: 1 })
	});

	with_body(b"data=0", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		match data {
			Error::InvalidData(DataVerificationError::DataInvalid(err)) => assert_eq!(err, "Too small."),
			_ => panic!("Expected DataVerificationError::DataInvalid, got {:?}", data)
		};
	});
}

#[test]
fn validate_combined_validator() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(min = 1, max = 2)]
		data: u8
	}

	with_body(b"data=1", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: 1 })
	});

	with_body(b"data=2", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: 2 })
	});

	with_body(b"data=0", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});

	with_body(b"data=3", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}

#[test]
fn validate_custom_validator() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(validator = custom_validator)]
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

#[test]
fn validate_custom_validator_lambda() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(validator = |value: &u8| {
			if *value == 0 {
				return Err("Value must not be 0");
			}
			Ok(())
		})]
		data: u8
	}

	with_body(b"data=1", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: 1 })
	});

	with_body(b"data=0", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}

#[test]
fn validate_custom_validator_string_str() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(validator = |_: &str| Result::<(), Infallible>::Ok(()))]
		data: String
	}

	with_body(b"data=foo", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: "foo".to_owned() })
	});
}

#[test]
fn validate_min_length() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(min_length = 8)]
		data: String
	}

	with_body(b"data=verylong", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data {
			data: "verylong".to_owned()
		})
	});

	with_body(b"data=shorter", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}

#[test]
fn validate_max_length() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(max_length = 7)]
		data: String
	}

	with_body(b"data=shorter", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data {
			data: "shorter".to_owned()
		})
	});

	with_body(b"data=verylong", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}

#[test]
fn validate_min() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(min = 10)]
		data: u64
	}

	with_body(b"data=10", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: 10 })
	});

	with_body(b"data=9", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}

#[test]
fn validate_max() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(max = 10)]
		data: u64
	}

	with_body(b"data=10", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: 10 })
	});

	with_body(b"data=11", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}

#[test]
fn validate_regex() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(regex = "^[a-z]+$")]
		data: String
	}

	with_body(b"data=lower", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data {
			data: "lower".to_owned()
		})
	});

	with_body(b"data=UPPER", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}

#[test]
fn validate_expected() {
	#[derive(Debug, FormData, PartialEq)]
	struct Data {
		#[validate(expected = &["foo", "bar"])]
		data: String
	}

	with_body(b"data=foo", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: "foo".to_owned() })
	});

	with_body(b"data=bar", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { data: "bar".to_owned() })
	});

	with_body(b"data=other", APPLICATION_WWW_FORM_URLENCODED, |state| {
		let data = block_on(Data::parse_form_data(state)).unwrap_err();
		assert!(matches!(data, Error::InvalidData(_)));
	});
}
