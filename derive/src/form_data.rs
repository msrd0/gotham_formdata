use proc_macro2::{TokenStream, Span, Ident};
use syn::{DeriveInput, Result, Error, Data, Type, Fields, Generics, spanned::Spanned};

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
	
	fn gen_add_entry_impl(&self) -> TokenStream {
		let ident = &self.ident;
		let (impl_gen, ty_gen, were) = self.generics.split_for_impl();
		
		let field_names = self.fields.iter().map(|(name, _)| name);
		let field_types = self.fields.iter().map(|(_, ty)| ty);
		
		quote! {
			impl #impl_gen #ident #ty_gen #were {
				fn add_entry(&mut self, name: ::std::sync::Arc<str>, value: String) -> Result<(), ::gotham_multipart::Error> {
					let name: &str = &name;
					if false {
						unreachable!()
					}
					#( else if name == stringify!(#field_names) {
						let value_parsed = value.parse::<#field_types>()
							.map_err(|err| ::gotham_multipart::Error::IllegalField(name.to_owned(), err.into()))?;
						self.#field_names.replace(value_parsed);
						Ok(())
					} )*
					else {
						Err(::gotham_multipart::Error::UnknownField(name.to_owned()))
					}
				}
			}
		}
	}
	
	fn gen_build_impl(&self) -> TokenStream {
		let name = &self.name;
		let ident = &self.ident;
		let (impl_gen, ty_gen, were) = self.generics.split_for_impl();
		
		let field_names = self.fields.iter().map(|(name, _)| name);
		
		quote! {
			impl #impl_gen #ident #ty_gen #were {
				fn build(self) -> Result<#name #ty_gen, ::gotham_multipart::Error> {
					Ok(#name #ty_gen {
						#( #field_names: self.#field_names.ok_or(::gotham_multipart::Error::MissingField(stringify!(#field_names).to_owned()))? ),*
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
		_ => return Err(Error::new(Span::call_site(), "#[derive(FormData)] can only be used on structs"))
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
				let field_ident = field.ident.ok_or(Error::new(span, "Fields without an ident are not supported"))?;
				let field_type = field.ty;
				builder.fields.push((field_ident, field_type));
			}
		},
		Fields::Unnamed(_) => return Err(Error::new(Span::call_site(), "#[derive(FormData)] cannot be used on tuple structs")),
		Fields::Unit => {}
	};
	
	let builder_struct = builder.gen_struct();
	let builder_default_impl = builder.gen_default_impl();
	let builder_add_entry_impl = builder.gen_add_entry_impl();
	let builder_build_impl = builder.gen_build_impl();
	
	Ok(quote! {
		#builder_struct
		#builder_default_impl
		#builder_add_entry_impl
		#builder_build_impl
		
		impl #impl_gen ::gotham_multipart::FormData for #name #ty_gen #were {
			type Err = ::gotham_multipart::Error;
			
			fn parse_form_data(state: &mut ::gotham_multipart::export::State) -> ::gotham_multipart::FormDataFuture<Self> {
				use ::gotham_multipart::export::FutureExt;
				use ::std::io::Read;
				
				let content_type = ::gotham_multipart::internal::get_content_type(state);
				let body = ::gotham_multipart::internal::get_body(state);
				
				async move {
					let content_type = content_type?;
					let boundary = ::gotham_multipart::internal::get_boundary(&content_type)?;
					let mut multipart = ::gotham_multipart::internal::get_multipart(body, boundary).await?;
					
					let mut builder: #builder_ident #ty_gen = Default::default();
					while let Some(mut field) = multipart.read_entry()? {
						let name = field.headers.name;
						let mut value = String::new();
						field.data.read_to_string(&mut value);
						builder.add_entry(name, value)?;
					}
					builder.build()
				}.boxed()
			}
		} 
	})
}
