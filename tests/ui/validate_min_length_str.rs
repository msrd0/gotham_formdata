use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate(min_length = "foobar")]
	value: String
}

fn main() {
}
