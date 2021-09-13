//! This crate implements derive macros for the `gotham_formdata` crate.
#![warn(missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Result};

#[macro_use]
mod util;

mod form_data;

#[inline]
fn print_tokens(tokens: TokenStream2) -> TokenStream {
	//eprintln!("{}", tokens);
	tokens.into()
}

#[inline]
fn expand_derive<F>(input: TokenStream, expand: F) -> TokenStream
where
	F: FnOnce(DeriveInput) -> Result<TokenStream2>
{
	print_tokens(expand(parse_macro_input!(input)).unwrap_or_else(|err| err.to_compile_error()))
}

/// This derive macro implements `FormData` for the struct it is invoked on. Enums, unions and
/// tuple structs are not supported.
#[proc_macro_derive(FormData, attributes(validate))]
pub fn derive_form_data(input: TokenStream) -> TokenStream {
	expand_derive(input, form_data::expand)
}
