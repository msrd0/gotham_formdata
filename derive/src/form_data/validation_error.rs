use super::Field;
use proc_macro2::{Ident, TokenStream};

pub(super) struct ValidationError<'a> {
	pub(super) name: &'a Ident,
	pub(super) ident: Ident,
	pub(super) fields: &'a [Field]
}

impl<'a> ValidationError<'a> {
	pub(super) fn gen_struct(&self) -> TokenStream {
		let name = &self.name;
		let ident = &self.ident;

		let doc = format!(
			"This error is returned when form data parsed for [{}] failed validation.",
			name
		);

		quote! {
			#[doc = #doc]
			#[derive(Debug)]
			enum #ident {

			}

			impl ::std::error::Error for #ident {}
		}
	}

	pub(super) fn gen_display_impl(&self) -> TokenStream {
		let ident = &self.ident;

		quote! {
			impl ::std::fmt::Display for #ident {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
					unimplemented!()
				}
			}
		}
	}
}
