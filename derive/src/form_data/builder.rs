use super::Field;
use proc_macro2::{Ident, TokenStream};
use syn::{ImplGenerics, TypeGenerics, WhereClause};

pub(super) struct FormDataBuilder<'a> {
	pub(super) name: &'a Ident,
	pub(super) ident: Ident,
	pub(super) impl_generics: &'a ImplGenerics<'a>,
	pub(super) ty_generics: &'a TypeGenerics<'a>,
	pub(super) where_clause: &'a WhereClause,
	pub(super) fields: &'a [Field]
}

impl<'a> FormDataBuilder<'a> {
	pub(super) fn gen_struct(&self) -> TokenStream {
		let ident = &self.ident;
		let ty_generics = &self.ty_generics;

		let field_names = self.fields.iter().map(|f| &f.ident);
		let field_types = self.fields.iter().map(|f| &f.ty);

		quote! {
			#[doc(hidden)]
			struct #ident #ty_generics {
				#( #field_names: ::core::option::Option<#field_types> ),*
			}
		}
	}

	pub(super) fn gen_default_impl(&self) -> TokenStream {
		let ident = &self.ident;
		let ty_generics = &self.ty_generics;

		let field_names = self.fields.iter().map(|f| &f.ident);

		quote! {
			impl #ty_generics ::core::default::Default for #ident #ty_generics {
				fn default() -> Self {
					Self {
						#( #field_names: ::core::option::Option::None ),*
					}
				}
			}
		}
	}

	pub(super) fn gen_builder_impl(&self) -> TokenStream {
		let name = &self.name;
		let ident = &self.ident;
		let impl_generics = &self.impl_generics;
		let ty_generics = &self.ty_generics;
		let where_clause = &self.where_clause;

		let field_names = self.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
		let field_types = self.fields.iter().map(|f| &f.ty);

		quote! {
			impl #impl_generics ::gotham_formdata::private::FormDataBuilder for #ident #ty_generics
			#where_clause
			{
				type Data = #name #ty_generics;

				fn add_entry<'a>(
						&'a mut self,
						name: ::std::borrow::Cow<'a, str>,
						value: ::gotham_formdata::value::Value<'a, ::gotham_formdata::Error>
				) -> ::gotham_formdata::private::FormDataBuilderFuture<'a> {
					#[allow(unused_imports)]
					use ::gotham_formdata::private::{FutureExt as _, StreamExt as _};

					async move {
						let name: &::core::primitive::str = &name;
						match name {
							#(stringify!(#field_names) => {
								::gotham_formdata::private::debug!("Found value for field {}", name);
								let value_parsed = ::gotham_formdata::private::Parse::<#field_types>::parse(value)
									.await
									.map_err(|err| ::gotham_formdata::Error::IllegalField(name.to_owned(), err.into()))?;
								self.#field_names.replace(value_parsed);
								Ok(())
							},)*
							_ => {
								::gotham_formdata::private::debug!("Found an unknown field: {}", name);
								Err(::gotham_formdata::Error::UnknownField(name.to_string()))
							}
						}
					}.boxed()
				}

				fn build(self) -> ::core::result::Result<Self::Data, ::gotham_formdata::Error> {
					::core::result::Result::Ok(Self::Data {
						#( #field_names: self.#field_names
							.ok_or_else(|| ::gotham_formdata::Error::MissingField(stringify!(#field_names).to_owned()))? ),*
					})
				}
			}
		}
	}
}
