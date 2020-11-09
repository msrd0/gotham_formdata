use std::convert::Infallible;

/// This trait allows data of type `T` to be verified against custom criteria.
///
/// Example:
///
/// ```rust
/// use gotham_formdata::FormData;
///
/// fn validate_password(password: &String) -> Result<(), &'static str> {
/// 	if password.len() < 8 {
/// 		return Err("Password is too short");
/// 		}
/// 	Ok(())
/// 	}
///
/// #[derive(FormData)]
/// struct LoginData {
/// 	username: String,
/// 	#[validate(validator = validate_password)]
/// 	password: String
/// 	}
/// ```
pub trait Validator<T> {
	/// The error returned when validation failed.
	type Err;

	/// Performs the validation.
	fn validate(self, data: &T) -> Result<(), Self::Err>;
}

/// Convert `()` into an allways accepting validator.
impl<T> Validator<T> for () {
	type Err = Infallible;

	fn validate(self, _: &T) -> Result<(), Infallible> {
		Ok(())
	}
}

/// Convert any function with the correct signature into a validator.
impl<F, Err, T> Validator<T> for F
where
	F: Fn(&T) -> Result<(), Err>
{
	type Err = Err;

	fn validate(self, data: &T) -> Result<(), Err> {
		self(data)
	}
}
