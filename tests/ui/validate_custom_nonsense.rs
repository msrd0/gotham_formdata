use gotham_formdata::FormData;
use serde::Deserialize;

#[derive(Deserialize, FormData)]
struct MyFormData {
	#[validate(validator = "struct CustomValidator;")]
	value: u8
}

fn main() {
}
