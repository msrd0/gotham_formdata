use gotham_formdata::FormData;
use serde::Deserialize;

#[derive(Deserialize, FormData)]
struct MyFormData {
	#[validate]
	value: u8
}

fn main() {
}
