use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate(min = 12345)]
	value: u8
}

fn main() {
}
