use futures_util::FutureExt;
use gotham_formdata::{
	conversion::{ConvertRawBytes, ConversionFuture},
	value::Value
};

struct MyType;

impl<'a, E: 'a> ConvertRawBytes<'a, E> for MyType {
	fn convert_value(_name: &'a str, _value: Value<'a, E>) -> ConversionFuture<'a, Self, E> {
		async move {
			Ok(MyType)
		}.boxed()
	}
}

fn main() {
}
