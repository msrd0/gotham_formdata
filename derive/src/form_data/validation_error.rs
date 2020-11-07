use super::Field;
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use syn::{LitStr, Visibility};

pub(super) struct ValidationError<'a> {
	pub(super) name: &'a Ident,
	pub(super) vis: &'a Visibility,
	pub(super) ident: Ident,
	pub(super) fields: &'a [Field]
}

impl<'a> ValidationError<'a> {
	pub(super) fn gen_struct(&self) -> TokenStream {
		let name = &self.name;
		let vis = &self.vis;
		let ident = &self.ident;

		let doc = format!(
			"This error is returned when form data parsed for [{}] failed validation.",
			name
		);

		let field_names = self
			.fields
			.iter()
			.map(|f| LitStr::new(&f.ident.to_string(), Span::call_site()))
			.collect::<Vec<_>>();
		let variant_idents = self
			.fields
			.iter()
			.map(|f| format_ident!("{}Invalid", f.ident.to_string().to_camel_case()))
			.collect::<Vec<_>>();
		let invalid_idents = self
			.fields
			.iter()
			.map(|f| format_ident!("invalid_{}", f.ident.to_string().to_snake_case()))
			.collect::<Vec<_>>();
		// TODO variant_error_types

		quote! {
			#[doc = #doc]
			#[derive(Debug)]
			#vis enum #ident {
				#(
					#variant_idents(String)
				),*
			}

			impl #ident {
				#[doc(hidden)]
				fn invalid<Err: ::std::fmt::Display>(field: &str, err: Err) -> Self {
					match field {
						#(
							#field_names => Self::#invalid_idents(err),
						)*
						_ => panic!("Unknown field {}", field)
					}
				}

				#(
					fn #invalid_idents<Err: ::std::fmt::Display>(err: Err) -> Self {
						Self::#variant_idents(err.to_string())
					}
				)*

				#vis fn field_name(&self) -> &'static str {
					match self {
						#( Self::#variant_idents(_) => #field_names ),*
					}
				}
			}

			impl ::std::error::Error for #ident {}
		}
	}

	pub(super) fn gen_display_impl(&self) -> TokenStream {
		let ident = &self.ident;

		quote! {
			impl ::std::fmt::Display for #ident {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
					write!(f, "Invalid value for field {}", self.field_name())
				}
			}
		}
	}
}
