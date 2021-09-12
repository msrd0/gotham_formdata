use gotham_formdata::FormData;
use validator::Validate;

#[derive(FormData, Validate)]
struct MyFormData(String);

fn main() {}
