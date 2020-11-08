use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate(min_length = -1 as i32)]
	value: String
}

fn main() {
}
