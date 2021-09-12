use gotham_formdata::FormData;
use validator::Validate;

#[derive(FormData, Validate)]
enum MyFormData {}

fn main() {}
