use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate(validator = "struct CustomValidator;")]
	value: u8
}

fn main() {
}
