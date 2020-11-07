//! This mod contains the `Validator` trait as well as pre-defined validation methods.

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
/// 	#[validate(validator = "validate_password")]
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

/// A validator that checks that a string has a minimum length.
#[derive(Clone, Copy, Debug)]
pub struct MinLengthValidator {
	min_length: usize
}

impl MinLengthValidator {
	/// Create a new [MinLengthValidator].
	pub fn new(min_length: usize) -> Self {
		Self { min_length }
	}
}

impl<T: AsRef<str>> Validator<T> for MinLengthValidator {
	type Err = String;

	fn validate(self, data: &T) -> Result<(), String> {
		if data.as_ref().len() < self.min_length {
			return Err(format!("Value is shorter than minimum length of {}", self.min_length));
		}
		Ok(())
	}
}

/// A validator that checks that a string has a maximum length.
#[derive(Clone, Copy, Debug)]
pub struct MaxLengthValidator {
	max_length: usize
}

impl MaxLengthValidator {
	/// Create a new [MaxLengthValidator].
	pub fn new(max_length: usize) -> Self {
		Self { max_length }
	}
}

impl<T: AsRef<str>> Validator<T> for MaxLengthValidator {
	type Err = String;

	fn validate(self, data: &T) -> Result<(), String> {
		if data.as_ref().len() > self.max_length {
			return Err(format!("Value is longer than maximum length of {}", self.max_length));
		}
		Ok(())
	}
}
