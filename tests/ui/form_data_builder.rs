use gotham_formdata::FormData;
use serde::Deserialize;

#[derive(Deserialize, FormData)]
struct MyFormData;

struct MyFormDataFormDataBuilder;

fn main() {
}
