use gotham_formdata::FormData;

#[derive(FormData)]
struct MyFormData {
	#[validate(foo = "bar")]
	value: u8
}

fn main() {
}
