use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate(validator = "foobar")]
	value: u8
}

fn main() {
}
