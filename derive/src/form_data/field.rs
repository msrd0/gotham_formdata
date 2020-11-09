use crate::util::*;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	spanned::Spanned,
	Error, Expr, Result, Token, Type
};

#[allow(dead_code)]
struct FieldMeta {
	ident: Ident,
	eq_token: Token![=],
	expr: Expr
}

impl Parse for FieldMeta {
	fn parse(input: ParseStream<'_>) -> Result<Self> {
		Ok(Self {
			ident: input.parse()?,
			eq_token: input.parse()?,
			expr: input.parse()?
		})
	}
}

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
			if !attr.path.ends_with("validate") {
				continue;
			}
			let attr_span = attr.tokens.span();
			let list = attr.parse_args_with(Punctuated::<FieldMeta, Token![,]>::parse_separated_nonempty)?;
			// parse_separated_nonempty guarantees that there is at least one element in the list
			if list.len() != 1 {
				return Err(Error::new(attr_span, "Expected single key-value pair for attribute validate"));
			}
			let validate = list.into_iter().next().unwrap();
			let name = validate.ident;
			let expr = validate.expr;

			validator = Some(match name.to_string().as_ref() {
				// custom validator
				"validator" => {
					// TODO this code makes sure to emit an error message pointing to the macro's
					// input, but there should be a better way to do this
					quote_spanned! { expr.span() =>
						{
							let validator = #expr;
							::gotham_formdata::internal::assert_validator::<_, _>(&validator);
							validator
						}
					}
				},

				// min_length validator
				"min_length" => quote!(::gotham_formdata::validate::MinLengthValidator::new(#expr)),

				// max_length validator
				"max_length" => quote!(::gotham_formdata::validate::MaxLengthValidator::new(#expr)),

				// min validator
				"min" => quote!(::gotham_formdata::validate::MinValidator::<#ty>::new(#expr)),

				// max validator
				"max" => quote!(::gotham_formdata::validate::MaxValidator::<#ty>::new(#expr)),

				// regex validator
				"regex" => {
					if cfg!(not(feature = "regex")) {
						return Err(Error::new(
							name.span(),
							"You need to enable the 'regex' feature of gotham_formdata to enable the regex validator"
						));
					}

					let regex_ident = format_ident!("{}_validation_regex", ident.to_string());
					quote!({
						static #regex_ident: ::gotham_formdata::validate::LazyRegex = ::gotham_formdata::validate::LazyRegex::new(#expr);
						::gotham_formdata::validate::RegexValidator::new(#regex_ident.get().expect("Invalid Regex"))
					})
				},

				// expected validator
				"expected" => quote!(::gotham_formdata::validate::ExpectedValidator::new(#expr)),

				_ => return Err(Error::new(name.span(), "Unknown key for attribute validate"))
			});
		}

		Ok(Self { ident, ty, validator })
	}
}
