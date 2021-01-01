use crate::util::*;
use proc_macro2::{Span, TokenStream};
use syn::{Data, DeriveInput, Error, Fields, Result};

mod builder;
use builder::FormDataBuilder;

mod field;
use field::Field;

mod validation_error;
use validation_error::ValidationError;

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
		Fields::Unit => Vec::new()
	};

	let validation_error = ValidationError {
		name,
		vis: &input.vis,
		ident: format_ident!("{}ValidationError", name),
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
	let field_validation_errors = fields.iter().map(|f| {
		f.validation_error
			.as_ref()
			.map(|err| quote!(Some(#err)))
			.unwrap_or_else(|| quote!(::std::option::Option::<String>::None))
	});

	// We're using this weird way of calling the validator to make type errors appear on the
	// validator's span, not the call site.
	let field_validate_call = fields.iter().map(|f| {
		let span = f.validator_span.unwrap_or_else(Span::call_site);
		quote_spanned! { span =>
			{
				fn validate_field_asserting_type<T, V>(validator: V, value: &T)
					-> ::std::result::Result<(), <V as ::gotham_formdata::validate::Validator<T>>::Err>
				where
					T: ?Sized,
					V: ::gotham_formdata::validate::Validator<T>
				{
					validator.validate(value)
				}
				validate_field_asserting_type(validator, value)
			}
		}
	});

	let validate_trait = format_ident!("Validate{}FormData", name);

	let mut dummy = format_ident!("_IMPL_FORMDATA_FOR_{}", name);
	dummy.set_span(Span::call_site());
	Ok(quote! {
		#validation_error_struct

		#[allow(non_upper_case_globals)]
		static #dummy: () = {
			#builder_struct
			#builder_default_impl
			#builder_builder_impl

			#[doc(hidden)]
			trait #validate_trait {
				fn validate(&self) -> Result<(), #err_ident>;
			}

			impl #impl_gen #validate_trait for #name #ty_gen #were {
				#[doc(hidden)]
				fn validate(&self) -> Result<(), #err_ident> {
					::log::debug!("Validating Form Data for type {}", stringify!(#name));

					#({
						const name: &str = stringify!(#field_idents);
						let value = &self.#field_idents;
						let validator = #field_validators;
						let validation_error = #field_validation_errors;
						if let Some(validator) = validator {
							#field_validate_call
								.map_err(|err| {
									match validation_error {
										Some(ve) => #err_ident::invalid(name, ve),
										None     => #err_ident::invalid(name, err)
									}
								})?;
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

						let res = ::gotham_formdata::internal::parse::<#builder_ident #ty_gen>(body, content_type).await?;
						#validate_trait::validate(&res).map_err(|err| ::gotham_formdata::Error::InvalidData(err))?;
						Ok(res)
					}.boxed()
				}
			}
		};
	})
}
