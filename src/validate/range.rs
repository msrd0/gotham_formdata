use super::Validator;
use std::fmt::{Debug, Display};
use thiserror::Error;

/// This error is emitted by the [MinValidator] if the value was too small.
#[derive(Clone, Copy, Debug, Error)]
#[error("Value is smaller than minimum of {0}")]
pub struct ValueTooSmallError<I: Debug + Display>(I);

/// A validator that checks that an integer is at least of a minimal value.
#[derive(Clone, Copy, Debug)]
pub struct MinValidator<I> {
	min: I
}

impl<I> MinValidator<I> {
	/// Create a new [MinValidator].
	pub fn new(min: I) -> Self {
		Self { min }
	}
}

impl<I, T> Validator<T> for MinValidator<I>
where
	I: Debug + Display + PartialOrd,
	T: Clone + Into<I>
{
	type Err = ValueTooSmallError<I>;

	fn validate(self, data: &T) -> Result<(), Self::Err> {
		if data.clone().into() < self.min {
			return Err(ValueTooSmallError(self.min));
		}
		Ok(())
	}
}

/// This error is emitted by the [MaxValidator] if the value was too large.
#[derive(Clone, Copy, Debug, Error)]
#[error("Value is greater than minimum of {0}")]
pub struct ValueTooLargeError<I: Debug + Display>(I);

/// A validator that checks that an integer is at most of a maximal value.
#[derive(Clone, Copy, Debug)]
pub struct MaxValidator<I> {
	max: I
}

impl<I> MaxValidator<I> {
	/// Create a new [MaxValidator].
	pub fn new(max: I) -> Self {
		Self { max }
	}
}

impl<I, T> Validator<T> for MaxValidator<I>
where
	I: Debug + Display + PartialOrd,
	T: Clone + Into<I>
{
	type Err = ValueTooLargeError<I>;

	fn validate(self, data: &T) -> Result<(), Self::Err> {
		if data.clone().into() > self.max {
			return Err(ValueTooLargeError(self.max));
		}
		Ok(())
	}
}
