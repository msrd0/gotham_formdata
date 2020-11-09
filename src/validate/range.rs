use super::Validator;
use std::fmt::Display;

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
	I: Display + PartialOrd,
	T: Clone + Into<I>
{
	type Err = String;

	fn validate(self, data: &T) -> Result<(), String> {
		if data.clone().into() < self.min {
			return Err(format!("Value is smaller than minimum of {}", self.min));
		}
		Ok(())
	}
}

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
	I: Display + PartialOrd,
	T: Clone + Into<I>
{
	type Err = String;

	fn validate(self, data: &T) -> Result<(), String> {
		if data.clone().into() > self.max {
			return Err(format!("Value is greater than maximum of {}", self.max));
		}
		Ok(())
	}
}
