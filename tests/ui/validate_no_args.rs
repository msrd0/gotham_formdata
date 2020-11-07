use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate]
	value: u8
}

fn main() {
}
