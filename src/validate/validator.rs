use either::Either;
use std::{convert::Infallible, marker::PhantomData};

/**
This trait allows data of type `T` to be verified against custom criteria.

Example:

```rust
use gotham_formdata::FormData;

fn validate_password(password: &String) -> Result<(), &'static str> {
	if password.len() < 8 {
		return Err("Password is too short");
		}
	Ok(())
}

#[derive(FormData)]
struct LoginData {
	username: String,
	#[validate(validator = validate_password)]
	password: String
}
```
*/
pub trait Validator<T: ?Sized> {
	/// The error returned when validation failed.
	type Err;

	/// Performs the validation.
	fn validate(self, data: &T) -> Result<(), Self::Err>;
}

/// Convert `()` into an allways accepting validator.
impl<T: ?Sized> Validator<T> for () {
	type Err = Infallible;

	fn validate(self, _: &T) -> Result<(), Infallible> {
		Ok(())
	}
}

/// Convert any function with the correct signature into a validator.
impl<F, Err, T: ?Sized> Validator<T> for F
where
	F: Fn(&T) -> Result<(), Err>
{
	type Err = Err;

	fn validate(self, data: &T) -> Result<(), Err> {
		self(data)
	}
}

/// This struct combines two validators and only validates its input if both validators pass it.
#[derive(Debug, Clone)]
pub struct CombinedValidator<T: ?Sized, V: Validator<T>, W: Validator<T>>(V, W, PhantomData<T>);

impl<T: ?Sized, V: Validator<T>, W: Validator<T>> CombinedValidator<T, V, W> {
	pub fn new(first_validator: V, second_validator: W) -> Self {
		Self(first_validator, second_validator, Default::default())
	}
}

impl<T: ?Sized, V: Validator<T>, W: Validator<T>> Validator<T> for CombinedValidator<T, V, W> {
	type Err = Either<V::Err, W::Err>;

	fn validate(self, data: &T) -> Result<(), Self::Err> {
		self.0.validate(data).map_err(Either::Left)?;
		self.1.validate(data).map_err(Either::Right)
	}
}
