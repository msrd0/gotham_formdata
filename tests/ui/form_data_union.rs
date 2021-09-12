use gotham_formdata::FormData;

#[derive(FormData)]
union MyFormData {
	foo: u8,
	bar: i8
}

fn main() {}
