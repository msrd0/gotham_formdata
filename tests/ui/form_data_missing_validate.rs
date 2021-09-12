use gotham_formdata::FormData;

struct MyType;

#[derive(FormData)]
struct MyFormData {
	foo: String
}

fn main() {}
