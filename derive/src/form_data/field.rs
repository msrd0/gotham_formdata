use proc_macro2::{Ident, Span};
use syn::{
	parse::{Parse, ParseStream},
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
	pub(super) ty: Type
}

impl Field {
	pub(super) fn new(field: syn::Field) -> Result<Self> {
		let span = field.span();

		let mut ident = field
			.ident
			.ok_or_else(|| Error::new(span, "Fields without an ident are not supported"))?;
		ident.set_span(Span::call_site());

		Ok(Self { ident, ty: field.ty })
	}
}
