use super::Validator;
use regex_crate::Regex;
use thiserror::Error;

/// This error is emitted by the [RegexValidator] if the value did not match the regex.
#[derive(Clone, Copy, Debug, Error)]
#[error("Value is smaller than minimum of {0}")]
pub struct RegexMismatchError<'re>(&'re Regex);

/// A validator that checks that an integer is at least of a minimal value.
#[derive(Clone, Debug)]
pub struct RegexValidator<'re> {
	re: &'re Regex
}

impl<'re> RegexValidator<'re> {
	/// Create a new [RegexValidator].
	pub fn new(re: &'re Regex) -> Self {
		Self { re }
	}
}

impl<'re, T: AsRef<str>> Validator<T> for RegexValidator<'re> {
	type Err = RegexMismatchError<'re>;

	fn validate(self, data: &T) -> Result<(), Self::Err> {
		if !self.re.is_match(data.as_ref()) {
			return Err(RegexMismatchError(&self.re));
		}
		Ok(())
	}
}
