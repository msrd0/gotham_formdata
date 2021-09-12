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
	pub(super) validator: Option<TokenStream>,
	pub(super) validator_span: Option<Span>,
	pub(super) validation_error: Option<Expr>
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
		let mut validator_span: Option<Span> = None;
		let mut validation_error: Option<Expr> = None;
		for attr in field.attrs {
			if !attr.path.ends_with("validate") {
				continue;
			}
			let list = attr.parse_args_with(Punctuated::<FieldMeta, Token![,]>::parse_separated_nonempty)?;
			// parse_separated_nonempty guarantees that there is at least one element in the list
			for meta in list.into_iter() {
				let name = meta.ident;
				let expr = meta.expr;

				let (new_validator, new_span) = match name.to_string().as_ref() {
					// custom error message
					"error" => {
						if validation_error.is_some() {
							return Err(Error::new(name.span(), "'error' must not appear more than once"));
						}
						validation_error = Some(expr);
						continue;
					},

					// custom validator
					"validator" => {
						// TODO this code makes sure to emit an error message pointing to the macro's
						// input, but there should be a better way to do this
						(
							quote_spanned! { expr.span() =>
								{
									let validator = #expr;
									::gotham_formdata::private::assert_validator::<_, _>(&validator);
									validator
								}
							},
							expr.span()
						)
					},

					// min_length validator
					"min_length" => (
						quote!(::gotham_formdata::validate::MinLengthValidator::new(#expr)),
						name.span()
					),

					// max_length validator
					"max_length" => (
						quote!(::gotham_formdata::validate::MaxLengthValidator::new(#expr)),
						name.span()
					),

					// min validator
					"min" => (
						quote!(::gotham_formdata::validate::MinValidator::<#ty>::new(#expr)),
						name.span()
					),

					// max validator
					"max" => (
						quote!(::gotham_formdata::validate::MaxValidator::<#ty>::new(#expr)),
						name.span()
					),

					// regex validator
					"regex" => {
						if cfg!(not(feature = "regex")) {
							return Err(Error::new(
								name.span(),
								"You need to enable the 'regex-validation' feature of gotham_formdata to enable the regex validator"
							));
						}

						let regex_ident = format_ident!("{}_validation_regex", ident.to_string());
						(
							quote!({
								static #regex_ident: ::gotham_formdata::private::LazyRegex =
										::gotham_formdata::private::LazyRegex::new(|| {
											::gotham_formdata::private::Regex::new(#expr).expect("Invalid Regex")
										});
								::gotham_formdata::validate::RegexValidator::new(&#regex_ident)
							}),
							name.span()
						)
					},

					// expected validator
					"expected" => (
						quote!(::gotham_formdata::validate::ExpectedValidator::new(#expr)),
						name.span()
					),

					_ => return Err(Error::new(name.span(), "Unknown key for attribute validate"))
				};

				validator = match validator {
					Some(old_validator) => Some(quote! {
						{
							let first_validator = #old_validator;
							let second_validator = #new_validator;
							::gotham_formdata::validate::CombinedValidator::new(first_validator, second_validator)
						}
					}),
					None => Some(new_validator)
				};

				validator_span = match validator_span {
					Some(old_span) => Some(old_span.join(new_span).unwrap_or(old_span)),
					None => Some(new_span)
				};
			}
		}

		Ok(Self {
			ident,
			ty,
			validator,
			validator_span,
			validation_error
		})
	}
}
