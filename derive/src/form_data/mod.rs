use crate::util::*;
use proc_macro2::{Span, TokenStream};
use std::iter;
use syn::{
	AngleBracketedGenericArguments, BoundLifetimes, Data, DeriveInput, Error, Fields, GenericArgument, Lifetime,
	LifetimeDef, PathArguments, PredicateType, Result, TraitBound, TraitBoundModifier, Type, TypeParamBound, TypePath,
	WhereClause, WherePredicate
};

mod builder;
use builder::FormDataBuilder;

mod field;
use field::Field;

pub(super) fn expand(input: DeriveInput) -> Result<TokenStream> {
	let name = &input.ident;

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	let mut where_clause = where_clause.cloned().unwrap_or_else(|| WhereClause {
		where_token: Default::default(),
		predicates: Default::default()
	});

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

	for f in &fields {
		// T: Send
		where_clause.predicates.push(WherePredicate::Type(PredicateType {
			lifetimes: None,
			bounded_ty: f.ty.clone(),
			colon_token: Default::default(),
			bounds: iter::once(TypeParamBound::Trait(TraitBound {
				paren_token: None,
				modifier: TraitBoundModifier::None,
				lifetimes: None,
				path: path!(::std::marker::Send)
			}))
			.collect()
		}));

		// for<'a> Value<'a>: Parse<T>
		let lt = format_ident!("gotham_formdata_value");
		where_clause.predicates.push(WherePredicate::Type(PredicateType {
			lifetimes: Some(BoundLifetimes {
				for_token: Default::default(),
				lt_token: Default::default(),
				lifetimes: iter::once(LifetimeDef {
					attrs: Vec::new(),
					lifetime: Lifetime {
						apostrophe: Span::call_site(),
						ident: lt.clone()
					},
					colon_token: None,
					bounds: Default::default()
				})
				.collect(),
				gt_token: Default::default()
			}),
			bounded_ty: {
				let mut path = path!(::gotham_formdata::private::Value);
				path.segments.last_mut().unwrap().arguments =
					PathArguments::AngleBracketed(AngleBracketedGenericArguments {
						colon2_token: None,
						lt_token: Default::default(),
						args: iter::once(GenericArgument::Lifetime(Lifetime {
							apostrophe: Span::call_site(),
							ident: lt
						}))
						.collect(),
						gt_token: Default::default()
					});
				Type::Path(TypePath { qself: None, path })
			},
			colon_token: Default::default(),
			bounds: iter::once(TypeParamBound::Trait(TraitBound {
				paren_token: None,
				modifier: TraitBoundModifier::None,
				lifetimes: None,
				path: {
					let mut path = path!(::gotham_formdata::private::Parse);
					path.segments.last_mut().unwrap().arguments =
						PathArguments::AngleBracketed(AngleBracketedGenericArguments {
							colon2_token: None,
							lt_token: Default::default(),
							args: iter::once(GenericArgument::Type(f.ty.clone())).collect(),
							gt_token: Default::default()
						});
					path
				}
			}))
			.collect()
		}));
	}

	let builder = FormDataBuilder {
		name,
		ident: format_ident!("{}FormDataBuilder", name),
		impl_generics: &impl_generics,
		ty_generics: &ty_generics,
		where_clause: &where_clause,
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

			impl #impl_generics ::gotham_formdata::FormData for #name #ty_generics
			#where_clause
			{
				type Err = ::gotham_formdata::Error;

				fn parse_form_data(state: &mut ::gotham_formdata::private::State) -> ::gotham_formdata::FormDataFuture<Self> {
					use ::gotham_formdata::private::FutureExt as _;

					let content_type = ::gotham_formdata::private::get_content_type(state);
					let body = ::gotham_formdata::private::get_body(state);

					async move {
						let content_type = content_type?;
						::gotham_formdata::private::debug!("Parsing Form Data for type {} with Content-Type {}", stringify!(#name), content_type);

						let res = ::gotham_formdata::private::parse::<#builder_ident #ty_generics>(body, content_type).await?;
						::gotham_formdata::private::Validate::validate(&res)?;
						Ok(res)
					}.boxed()
				}
			}
		};
	})
}
