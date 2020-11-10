use super::Validator;
use thiserror::Error;

/// This error is emitted by the [MinLengthValidator] if the value was too short.
#[derive(Clone, Copy, Debug, Error)]
#[error("Value is shorter than minimum length of {0}")]
pub struct ValueTooShortError(usize);

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
	type Err = ValueTooShortError;

	fn validate(self, data: &T) -> Result<(), Self::Err> {
		if data.as_ref().len() < self.min_length {
			return Err(ValueTooShortError(self.min_length));
		}
		Ok(())
	}
}

/// This error is emitted by the [MaxLengthValidator] if the value was too long.
#[derive(Clone, Copy, Debug, Error)]
#[error("Value is longer than maximum length of {0}")]
pub struct ValueTooLongError(usize);

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
	type Err = ValueTooLongError;

	fn validate(self, data: &T) -> Result<(), Self::Err> {
		if data.as_ref().len() > self.max_length {
			return Err(ValueTooLongError(self.max_length));
		}
		Ok(())
	}
}
