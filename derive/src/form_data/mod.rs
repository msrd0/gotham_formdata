use crate::util::*;
use proc_macro2::{Span, TokenStream};
use syn::{Data, DeriveInput, Error, Fields, Result};

mod builder;
use builder::FormDataBuilder;

mod field;
use field::Field;

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

	let builder = FormDataBuilder {
		name,
		ident: format_ident!("{}FormDataBuilder", name),
		generics: &input.generics,
		fields: &fields
	};
	let builder_ident = &builder.ident;

	let builder_struct = builder.gen_struct();
	let builder_default_impl = builder.gen_default_impl();
	let builder_builder_impl = builder.gen_builder_impl();

	let mut dummy = format_ident!("_IMPL_FORMDATA_FOR_{}", name);
	dummy.set_span(Span::call_site());
	Ok(quote! {
		#[allow(non_upper_case_globals)]
		static #dummy: () = {
			#builder_struct
			#builder_default_impl
			#builder_builder_impl

			impl #impl_gen ::gotham_formdata::FormData for #name #ty_gen #were {
				type Err = ::gotham_formdata::Error;

				fn parse_form_data(state: &mut ::gotham_formdata::private::State) -> ::gotham_formdata::FormDataFuture<Self> {
					use ::gotham_formdata::private::FutureExt as _;

					let content_type = ::gotham_formdata::private::get_content_type(state);
					let body = ::gotham_formdata::private::get_body(state);

					async move {
						let content_type = content_type?;
						::log::debug!("Parsing Form Data for type {} with Content-Type {}", stringify!(#name), content_type);

						let res = ::gotham_formdata::private::parse::<#builder_ident #ty_gen>(body, content_type).await?;
						::gotham_formdata::private::Validate::validate(&res)?;
						Ok(res)
					}.boxed()
				}
			}
		};
	})
}
