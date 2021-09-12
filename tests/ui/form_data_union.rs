use gotham_formdata::FormData;
use validator::Validate;

#[derive(FormData, Validate)]
union MyFormData {
	foo: u8,
	bar: i8
}

fn main() {}
