use super::Field;
use proc_macro2::{Ident, TokenStream};
use syn::Generics;

pub(super) struct FormDataBuilder<'a> {
	pub(super) name: &'a Ident,
	pub(super) err_ident: &'a Ident,
	pub(super) ident: Ident,
	pub(super) generics: &'a Generics,
	pub(super) fields: &'a [Field]
}

impl<'a> FormDataBuilder<'a> {
	pub(super) fn gen_struct(&self) -> TokenStream {
		let ident = &self.ident;
		let (impl_gen, _, were) = self.generics.split_for_impl();

		let field_names = self.fields.iter().map(|f| &f.ident);
		let field_types = self.fields.iter().map(|f| &f.ty);

		quote! {
			#[doc(hidden)]
			struct #ident #impl_gen #were {
				#( #field_names: Option<#field_types> ),*
			}
		}
	}

	pub(super) fn gen_default_impl(&self) -> TokenStream {
		let ident = &self.ident;
		let (impl_gen, ty_gen, were) = self.generics.split_for_impl();

		let field_names = self.fields.iter().map(|f| &f.ident);

		quote! {
			impl #impl_gen Default for #ident #ty_gen #were {
				fn default() -> Self {
					Self {
						#( #field_names: None ),*
					}
				}
			}
		}
	}

	pub(super) fn gen_builder_impl(&self) -> TokenStream {
		let name = &self.name;
		let err_ident = &self.err_ident;
		let ident = &self.ident;
		let (impl_gen, ty_gen, were) = self.generics.split_for_impl();

		let field_names = self.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
		let field_types = self.fields.iter().map(|f| &f.ty);

		quote! {
			impl #impl_gen ::gotham_formdata::private::FormDataBuilder for #ident #ty_gen #were {
				type Data = #name #ty_gen;
				type Err = #err_ident;

				fn add_entry<'a>(
						&'a mut self,
						name: ::std::borrow::Cow<'a, str>,
						value: ::gotham_formdata::value::Value<'a, ::gotham_formdata::Error<Self::Err>>
				) -> ::gotham_formdata::private::FormDataBuilderFuture<'a, Self::Err> {
					#[allow(unused_imports)]
					use ::gotham_formdata::{conversion::prelude::*, private::{FutureExt, StreamExt}};

					async move {
						let name: &str = &name;
						match name {
							#(stringify!(#field_names) => {
								log::debug!("Found value for field {}", name);
								let value_parsed = <#field_types>::convert_value(name, value).await?;
								self.#field_names.replace(value_parsed);
								Ok(())
							},)*
							_ => {
								log::debug!("Found an unknown field: {}", name);
								Err(::gotham_formdata::Error::UnknownField(name.to_string()))
							}
						}
					}.boxed()
				}

				fn build(self) -> Result<#name #ty_gen, ::gotham_formdata::Error<Self::Err>> {
					Ok(#name #ty_gen {
						#( #field_names: self.#field_names.ok_or(::gotham_formdata::Error::MissingField(stringify!(#field_names).to_owned()))? ),*
					})
				}
			}
		}
	}
}
