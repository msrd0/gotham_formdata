use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate(min = 1, error = "Minimum value is 1", error = "The value must be at least 1")]
	value: u8
}

fn main() {
}
