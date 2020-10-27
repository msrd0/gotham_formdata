use gotham_formdata::FormData;
use serde::Deserialize;

#[derive(Deserialize)]
struct MyType;

#[derive(Deserialize, FormData)]
struct MyFormData {
	foo: MyType
}

fn main() {
}
