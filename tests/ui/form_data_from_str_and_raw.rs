use futures_util::future::FutureExt;
use gotham_formdata::FormData;
use gotham_formdata::conversion::*;
use std::convert::Infallible;
use std::str::FromStr;

struct CustomType(bool);

impl FromStr for CustomType {
	type Err = Infallible;
	
	fn from_str(_: &str) -> Result<Self, Infallible> {
		Ok(Self(false))
	}
}

impl<'a, E: 'a> ConvertRawBytes<'a, E> for CustomType {
	fn convert_byte_stream(_name: &'a str, mut stream: ByteStream<E>) -> ConversionFuture<'a, Self, E> {
		async move {
			Ok(Self(true))
		}.boxed()
	}
}

#[derive(FormData)]
struct Data {
	foo: CustomType
}

fn main() {
}
