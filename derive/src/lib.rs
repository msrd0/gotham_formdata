#![warn(rust_2018_idioms)]
#![deny(unreachable_pub)]

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Result};

mod form_data;
mod util;

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

#[proc_macro_derive(FormData, attributes(validate))]
pub fn derive_form_data(input: TokenStream) -> TokenStream {
	expand_derive(input, form_data::expand)
}
