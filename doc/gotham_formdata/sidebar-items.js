initSidebarItems({"derive":[["FormData","This derive macro implements `FormData` for the struct it is invoked on. Enums, unions and tuple structs are not supported."]],"enum":[["Error","This error type is used when parsing form data from a request body was unsuccessful."]],"mod":[["conversion","This mod contains conversion traits for common used types, that allows them to be created from a stream of bytes. Furthermore, it allows every type that implements [FromStr] plus some other common types to be converted."],["validate","This mod contains the [Validator] trait as well as pre-defined validation methods."],["value","This mod defines the [Value] used for parsing the form data."]],"trait":[["FormData","This is the trait implemented by `#[derive(FormData)]`. It provides a method to parse the struct it is implemented for to be parsed from the request body contained in gotham's state."],["FormDataFromState","This is the equivalent of [FormData] from the state's perspective. Use this if you prefer `state.parse_form_data::<MyData>()?` over `MyData::parse_form_data(&mut state)?`."]],"type":[["FormDataFuture","This is the return type of [FormData::parse_form_data]."]]});