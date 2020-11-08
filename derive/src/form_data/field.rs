use crate::util::*;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{spanned::Spanned, Error, Expr, Lit, Meta, NestedMeta, Result, Type};

pub(super) struct Field {
	pub(super) ident: Ident,
	pub(super) ty: Type,
	pub(super) validator: Option<TokenStream>
}

impl Field {
	pub(super) fn new(field: syn::Field) -> Result<Self> {
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

			validator = Some(match path {
				// custom validator
				path if path.ends_with("validator") => {
					let value = match validate.lit {
						Lit::Str(value) => value,
						lit => {
							return Err(Error::new(
								lit.span(),
								"Expected string literal containing validator expression"
							))
						},
					};
					let expr: Expr = syn::parse_str(&value.value()).map_err(|err| err.with_span(value.span()))?;
					quote_spanned!(value.span() => #expr)
				},

				// min_length validator
				path if path.ends_with("min_length") => {
					let value = match validate.lit {
						Lit::Int(value) => value,
						lit => return Err(Error::new(lit.span(), "Expected integer literal for min_length validator"))
					};
					let min_length: usize = value.base10_parse().map_err(|err| err.with_span(value.span()))?;
					quote!(::gotham_formdata::validate::MinLengthValidator::new(#min_length))
				},

				// max_length validator
				path if path.ends_with("max_length") => {
					let value = match validate.lit {
						Lit::Int(value) => value,
						lit => return Err(Error::new(lit.span(), "Expected integer literal for max_length validator"))
					};
					let max_length: usize = value.base10_parse().map_err(|err| err.with_span(value.span()))?;
					quote!(::gotham_formdata::validate::MaxLengthValidator::new(#max_length))
				},

				// min validator
				path if path.ends_with("min") => {
					let value = match validate.lit {
						Lit::Int(value) => value,
						lit => return Err(Error::new(lit.span(), "Expected integer literal for min validator"))
					};
					quote!(::gotham_formdata::validate::MinValidator::<#ty>::new(#value))
				},

				// max validator
				path if path.ends_with("max") => {
					let value = match validate.lit {
						Lit::Int(value) => value,
						lit => return Err(Error::new(lit.span(), "Expected integer literal for max validator"))
					};
					quote!(::gotham_formdata::validate::MaxValidator::<#ty>::new(#value))
				},

				path => return Err(Error::new(path.span(), "Unknown key for attribute validate"))
			});
		}

		Ok(Self { ident, ty, validator })
	}
}
