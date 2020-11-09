use super::Validator;

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
