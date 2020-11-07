use gotham_formdata::FormData;

struct MyType;

#[derive(FormData)]
struct MyFormData {
	foo: MyType
}

fn main() {
}
