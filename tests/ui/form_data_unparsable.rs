use gotham_formdata::FormData;
use validator::Validate;

struct MyType;

#[derive(FormData, Validate)]
struct MyFormData {
	foo: MyType
}

fn assert_formdata<T: FormData>() {}

fn main() {
	assert_formdata::<MyFormData>();
}
