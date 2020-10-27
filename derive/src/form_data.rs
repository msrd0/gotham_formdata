use proc_macro2::{Ident, Span, TokenStream};
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields, Generics, Result, Type};

struct FormDataBuilder<'a> {
	name: &'a Ident,
	ident: Ident,
	generics: &'a Generics,
	fields: Vec<(Ident, Type)>
}

impl<'a> FormDataBuilder<'a> {
	fn gen_struct(&self) -> TokenStream {
		let ident = &self.ident;
		let (impl_gen, _, were) = self.generics.split_for_impl();

		let field_names = self.fields.iter().map(|(name, _)| name);
		let field_types = self.fields.iter().map(|(_, ty)| ty);

		quote! {
			#[doc(hidden)]
			struct #ident #impl_gen #were {
				#( #field_names: Option<#field_types> ),*
			}
		}
	}

	fn gen_default_impl(&self) -> TokenStream {
		let ident = &self.ident;
		let (impl_gen, ty_gen, were) = self.generics.split_for_impl();

		let field_names = self.fields.iter().map(|(name, _)| name);

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

	fn gen_builder_impl(&self) -> TokenStream {
		let name = &self.name;
		let ident = &self.ident;
		let (impl_gen, ty_gen, were) = self.generics.split_for_impl();

		let field_names = self.fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
		let field_types = self.fields.iter().map(|(_, ty)| ty);

		quote! {
			impl #impl_gen ::gotham_formdata::internal::FormDataBuilder for #ident #ty_gen #were {
				type Data = #name #ty_gen;

				fn add_entry(&mut self, name: ::std::sync::Arc<str>, value: String) -> Result<(), ::gotham_formdata::Error> {
					let name: &str = &name;
					if false {
						unreachable!()
					}
					#( else if name == stringify!(#field_names) {
						let value_parsed = value.parse::<#field_types>()
							.map_err(|err| ::gotham_formdata::Error::IllegalField(name.to_owned(), err.into()))?;
						log::debug!("Found value for field {}", name);
						self.#field_names.replace(value_parsed);
						Ok(())
					} )*
					else {
						log::debug!("Found an unknown field: {}", name);
						Err(::gotham_formdata::Error::UnknownField(name.to_owned()))
					}
				}

				fn build(self) -> Result<#name #ty_gen, ::gotham_formdata::Error> {
					Ok(#name #ty_gen {
						#( #field_names: self.#field_names.ok_or(::gotham_formdata::Error::MissingField(stringify!(#field_names).to_owned()))? ),*
					})
				}
			}
		}
	}
}

pub(super) fn expand(input: DeriveInput) -> Result<TokenStream> {
	let name = &input.ident;
	let (impl_gen, ty_gen, were) = input.generics.split_for_impl();
	let strukt = match input.data {
		Data::Struct(strukt) => strukt,
		_ => {
			return Err(Error::new(
				Span::call_site(),
				"#[derive(FormData)] can only be used on structs"
			))
		},
	};

	let mut builder = FormDataBuilder {
		name,
		ident: format_ident!("{}FormDataBuilder", name),
		generics: &input.generics,
		fields: Vec::new()
	};
	let builder_ident = &builder.ident;
	match strukt.fields {
		Fields::Named(named) => {
			for field in named.named {
				let span = field.span();
				let field_ident = field
					.ident
					.ok_or(Error::new(span, "Fields without an ident are not supported"))?;
				let field_type = field.ty;
				builder.fields.push((field_ident, field_type));
			}
		},
		Fields::Unnamed(_) => {
			return Err(Error::new(
				Span::call_site(),
				"#[derive(FormData)] cannot be used on tuple structs"
			))
		},
		Fields::Unit => {}
	};

	let builder_struct = builder.gen_struct();
	let builder_default_impl = builder.gen_default_impl();
	let builder_builder_impl = builder.gen_builder_impl();

	Ok(quote! {
		#builder_struct
		#builder_default_impl
		#builder_builder_impl

		impl #impl_gen ::gotham_formdata::FormData for #name #ty_gen #were {
			type Err = ::gotham_formdata::Error;

			fn parse_form_data(state: &mut ::gotham_formdata::export::State) -> ::gotham_formdata::FormDataFuture<Self> {
				use ::gotham_formdata::export::FutureExt;

				let content_type = ::gotham_formdata::internal::get_content_type(state);
				let body = ::gotham_formdata::internal::get_body(state);

				async move {
					let content_type = content_type?;
					log::debug!("Parsing Form Data for type {} with Content-Type {}", stringify!(#name), content_type);

					if ::gotham_formdata::internal::is_urlencoded(&content_type) {
						::gotham_formdata::internal::parse_urlencoded::<#name #ty_gen>(body).await
					}

					else if ::gotham_formdata::internal::is_multipart(&content_type) {
						::gotham_formdata::internal::parse_multipart::<#builder_ident #ty_gen>(body, &content_type).await
					}

					else {
						Err(::gotham_formdata::Error::UnknownContentType(content_type))
					}
				}.boxed()
			}
		}
	})
}
