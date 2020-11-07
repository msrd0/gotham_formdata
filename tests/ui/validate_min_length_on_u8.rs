use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate(min_length = 42)]
	value: u8
}

fn main() {
}
