use super::Validator;
use thiserror::Error;

/// This error is emitted by the [ExpectedValidator] if an unexpected value was found.
#[derive(Debug, Clone, Copy, Error)]
#[error("Value is not in list of expected values.")]
pub struct UnexpectedValueError;

/// A validator that checks that a value is in a list of accepted values.
#[derive(Clone, Copy, Debug)]
pub struct ExpectedValidator<'a, T> {
	expected: &'a [T]
}

impl<'a, T> ExpectedValidator<'a, T> {
	/// Create a new [ExpectedValidator].
	pub fn new(expected: &'a [T]) -> Self {
		Self { expected }
	}
}

impl<'a, D, T> Validator<D> for ExpectedValidator<'a, T>
where
	D: PartialEq<T>
{
	type Err = UnexpectedValueError;

	fn validate(self, data: &D) -> Result<(), Self::Err> {
		if !self.expected.iter().any(|expected| data == expected) {
			return Err(UnexpectedValueError);
		}
		Ok(())
	}
}
