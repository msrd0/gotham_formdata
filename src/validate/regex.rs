use super::Validator;
use regex_crate::Regex;
use std::{cell::Cell, mem::MaybeUninit, sync::Once};
use thiserror::Error;

/// Lazy Regex creation, similar to `lazy_static`. NOT PUBLIC API.
#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct LazyRegex {
	raw: &'static str,
	re: Cell<MaybeUninit<Result<Regex, regex_crate::Error>>>,
	once: Once
}

impl LazyRegex {
	pub const fn new(raw: &'static str) -> Self {
		Self {
			raw,
			re: Cell::new(MaybeUninit::uninit()),
			once: Once::new()
		}
	}

	pub fn get(&'static self) -> Result<&'static Regex, regex_crate::Error> {
		self.once.call_once(|| self.re.set(MaybeUninit::new(Regex::new(self.raw))));

		// self.re is guaranteed to be initialized at this point
		let re = unsafe { &*(*self.re.as_ptr()).as_ptr() };

		match re {
			Ok(re) => Ok(&re),
			Err(err) => Err(err.clone())
		}
	}
}

// regex::Regex and regex::Error are both Sync, so this should be fine
unsafe impl Sync for LazyRegex {}

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
