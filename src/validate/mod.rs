//! This mod contains the [Validator] trait as well as pre-defined validation methods.

mod expected;
pub use expected::*;

mod length;
pub use length::*;

mod range;
pub use range::*;

mod validator;
pub use validator::*;

#[cfg(feature = "regex")]
mod regex;
#[cfg(feature = "regex")]
pub use regex::*;
