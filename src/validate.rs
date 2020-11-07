//! This mod contains the `Validator` trait as well as pre-defined validation methods.

/// This trait allows data of type `T` to be verified against custom criteria.
///
/// Example:
///
/// ```rust
/// // TODO
/// ```
pub trait Validator<T> {
	/// The error returned when validation failed.
	type Err;

	/// Performs the validation.
	fn validate(self, data: &T) -> Result<(), Self::Err>;
}

impl<F, Err, T> Validator<T> for F
where
	F: Fn(&T) -> Result<(), Err>
{
	type Err = Err;

	fn validate(self, data: &T) -> Result<(), Err> {
		self(data)
	}
}
