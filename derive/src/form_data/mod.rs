use crate::util::*;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{spanned::Spanned, Data, DeriveInput, Error, Expr, Fields, Lit, Meta, NestedMeta, Result, Type};

mod builder;
use builder::FormDataBuilder;

mod validation_error;
use validation_error::ValidationError;

struct Field {
	ident: Ident,
	ty: Type,
	validator: Option<TokenStream>
}

impl Field {
	fn new(field: syn::Field) -> Result<Self> {
		let span = field.span();

		let mut ident = field
			.ident
			.ok_or(Error::new(span, "Fields without an ident are not supported"))?;
		ident.set_span(Span::call_site());

		let ty = field.ty;
		// unfortunately, we cannot change spans of types

		let mut validator: Option<TokenStream> = None;
		for attr in field.attrs {
			let meta = attr.parse_meta()?;
			if !meta.path().ends_with("validate") {
				continue;
			}
			let list = match meta {
				Meta::List(list) => list,
				_ => return Err(Error::new(meta.span(), "Illegal syntax for attribute validate"))
			};
			if list.nested.len() != 1 {
				return Err(Error::new(
					list.nested.span(),
					"Expected single key-value pair for attribute validate"
				));
			}
			let nested = list.nested.into_iter().next().unwrap();
			let validate = match nested {
				NestedMeta::Meta(Meta::NameValue(nv)) => nv,
				_ => return Err(Error::new(nested.span(), "Expected key-value pair for attribute validate"))
			};
			let path = validate.path;
			let value = match validate.lit {
				Lit::Str(value) => value,
				lit => return Err(Error::new(lit.span(), "Expected string literal for attribute validate"))
			};
			validator = Some(match path {
				path if path.ends_with("validator") => {
					let expr: Expr = syn::parse_str(&value.value())?;
					quote_spanned!(value.span() => #expr)
				},
				path => return Err(Error::new(path.span(), "Unknown key for attribute validate"))
			});
		}

		Ok(Self { ident, ty, validator })
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

	let fields = match strukt.fields {
		Fields::Named(named) => named.named.into_iter().map(|field| Field::new(field)).collect_to_result()?,
		Fields::Unnamed(_) => {
			return Err(Error::new(
				Span::call_site(),
				"#[derive(FormData)] cannot be used on tuple structs"
			))
		},
		Fields::Unit => Default::default()
	};

	let validation_error = ValidationError {
		name,
		vis: &input.vis,
		ident: format_ident!("{}VerificationError", name),
		fields: &fields
	};
	let err_ident = &validation_error.ident;

	let builder = FormDataBuilder {
		name,
		err_ident,
		ident: format_ident!("{}FormDataBuilder", name),
		generics: &input.generics,
		fields: &fields
	};
	let builder_ident = &builder.ident;

	let validation_error_struct = validation_error.gen_struct();
	let validation_error_display_impl = validation_error.gen_display_impl();

	let builder_struct = builder.gen_struct();
	let builder_default_impl = builder.gen_default_impl();
	let builder_builder_impl = builder.gen_builder_impl();

	let field_idents = fields.iter().map(|f| &f.ident);
	let field_validators = fields.iter().map(|f| {
		f.validator
			.as_ref()
			.map(|v| quote!(Some({ #v })))
			.unwrap_or_else(|| quote!(::std::option::Option::<()>::None))
	});

	Ok(quote! {
		#validation_error_struct
		#validation_error_display_impl

		#builder_struct
		#builder_default_impl
		#builder_builder_impl

		impl #impl_gen #name #ty_gen #were {
			#[doc(hidden)]
			fn __validate(&self) -> Result<(), #err_ident> {
				::log::debug!("Validating Form Data for type {}", stringify!(#name));

				#({
					let value = &self.#field_idents;
					let validator = #field_validators;
					if let Some(validator) = validator {
						::gotham_formdata::validate::Validator::validate(validator, value)
							.map_err(|err| #err_ident::invalid(stringify!(#field_idents), err))?;
					}
				})*

				Ok(())
			}
		}

		impl #impl_gen ::gotham_formdata::FormData for #name #ty_gen #were {
			type Err = ::gotham_formdata::Error<#err_ident>;

			fn parse_form_data(state: &mut ::gotham_formdata::export::State) -> ::gotham_formdata::FormDataFuture<Self> {
				use ::gotham_formdata::export::FutureExt;

				let content_type = ::gotham_formdata::internal::get_content_type(state);
				let body = ::gotham_formdata::internal::get_body(state);

				async move {
					let content_type = content_type?;
					::log::debug!("Parsing Form Data for type {} with Content-Type {}", stringify!(#name), content_type);

					let res: Self = match &content_type {
						ct if ::gotham_formdata::internal::is_urlencoded(ct) => {
							::gotham_formdata::internal::parse_urlencoded::<#name #ty_gen, _>(body).await
						},
						ct if ::gotham_formdata::internal::is_multipart(ct) => {
							::gotham_formdata::internal::parse_multipart::<#builder_ident #ty_gen>(body, ct).await
						},
						_ => Err(::gotham_formdata::Error::UnknownContentType(content_type))
					}?;
					res.__validate().map_err(|err| ::gotham_formdata::Error::InvalidData(err))?;
					Ok(res)
				}.boxed()
			}
		}
	})
}
