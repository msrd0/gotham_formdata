use futures_util::FutureExt;
use gotham_formdata::{
	conversion::{ConversionFuture, ConvertFromStr},
	value::Value
};

struct MyType;

impl<E> ConvertFromStr<E> for MyType {
	fn convert_value<'a>(name: &'a str, value: Value<'a, E>) -> ConversionFuture<'a, Self, E> {
		async move { Ok(MyType) }.boxed()
	}
}

fn main() {}
