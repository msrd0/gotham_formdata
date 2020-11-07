use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate = "custom_validator"]
	value: u8
}

fn main() {
}
