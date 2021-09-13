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

#[test]
fn test_generic() {
	#[derive(Debug, PartialEq, Validate)]
	struct Data<T> {
		foo: T
	}

	#[allow(non_upper_case_globals)]
	static _IMPL_FORMDATA_FOR_Data: () = {
		#[doc(hidden)]
		struct DataFormDataBuilder<T> {
			foo: ::core::option::Option<T>
		}
		impl<T> ::core::default::Default for DataFormDataBuilder<T> {
			fn default() -> Self {
				Self {
					foo: ::core::option::Option::None
				}
			}
		}
		impl<T> ::gotham_formdata::private::FormDataBuilder for DataFormDataBuilder<T>
		where
			T: ::std::marker::Send,
			for<'a> ::gotham_formdata::private::Value<'a>: ::gotham_formdata::private::Parse<T>
		{
			type Data = Data<T>;
			fn add_entry<'a>(
				&'a mut self,
				name: ::std::borrow::Cow<'a, str>,
				value: ::gotham_formdata::value::Value<'a, ::gotham_formdata::Error>
			) -> ::gotham_formdata::private::FormDataBuilderFuture<'a> {
				#[allow(unused_imports)]
				use ::gotham_formdata::private::{FutureExt as _, StreamExt as _};
				async move {
					let name: &::core::primitive::str = &name;
					match name {
						"foo" => {
							let value_parsed = ::gotham_formdata::private::Parse::<T>::parse(value)
								.await
								.map_err(|err| ::gotham_formdata::Error::IllegalField(name.to_owned(), err.into()))?;
							self.foo.replace(value_parsed);
							Ok(())
						},
						_ => Err(::gotham_formdata::Error::UnknownField(name.to_string()))
					}
				}
				.boxed()
			}
			fn build(self) -> ::core::result::Result<Self::Data, ::gotham_formdata::Error> {
				::core::result::Result::Ok(Self::Data {
					foo: self.foo.ok_or(::gotham_formdata::Error::MissingField("foo".to_owned()))?
				})
			}
		}
		impl<T> ::gotham_formdata::FormData for Data<T>
		where
			T: ::std::marker::Send,
			for<'a> ::gotham_formdata::private::Value<'a>: ::gotham_formdata::private::Parse<T>
		{
			type Err = ::gotham_formdata::Error;
			fn parse_form_data(state: &mut ::gotham_formdata::private::State) -> ::gotham_formdata::FormDataFuture<Self> {
				use ::gotham_formdata::private::FutureExt as _;
				let content_type = ::gotham_formdata::private::get_content_type(state);
				let body = ::gotham_formdata::private::get_body(state);
				async move {
					let content_type = content_type?;
					let res = ::gotham_formdata::private::parse::<DataFormDataBuilder<T>>(body, content_type).await?;
					::gotham_formdata::private::Validate::validate(&res)?;
					Ok(res)
				}
				.boxed()
			}
		}
	};

	with_body_foo(b"bar", |state| {
		let data = block_on(Data::<String>::parse_form_data(state)).unwrap();
		assert_eq!(data, Data { foo: "bar".to_owned() })
	});
}
