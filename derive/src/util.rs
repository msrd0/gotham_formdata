use proc_macro2::Span;
use syn::{Error, Path};

macro_rules! path {
	(:: $($segment:ident)::*) => {
		path!(@private Some(Default::default()), $($segment),*)
	};
	($($segment:ident)::*) => {
		path!(@private None, $($segment),*)
	};
	(@private $leading_colon:expr, $($segment:ident),*) => {
		{
			#[allow(unused_mut)]
			let mut segments: ::syn::punctuated::Punctuated<::syn::PathSegment, _> = Default::default();
			$(
				segments.push(::syn::PathSegment {
					ident: ::proc_macro2::Ident::new(stringify!($segment), ::proc_macro2::Span::call_site()),
					arguments: Default::default()
				});
			)*
			::syn::Path {
				leading_colon: $leading_colon,
				segments
			}
		}
	};
}

pub(crate) trait CollectToResult {
	type Item;

	fn collect_to_result(self) -> Result<Vec<Self::Item>, Error>;
}

impl<Item, I> CollectToResult for I
where
	I: Iterator<Item = Result<Item, Error>>
{
	type Item = Item;

	fn collect_to_result(self) -> Result<Vec<Item>, Error> {
		self.fold(<Result<Vec<Item>, Error>>::Ok(Vec::new()), |res, code| match (code, res) {
			(Ok(code), Ok(mut codes)) => {
				codes.push(code);
				Ok(codes)
			},
			(Ok(_), Err(errors)) => Err(errors),
			(Err(err), Ok(_)) => Err(err),
			(Err(err), Err(mut errors)) => {
				errors.combine(err);
				Err(errors)
			}
		})
	}
}

pub(crate) trait PathEndsWith {
	fn ends_with(&self, s: &str) -> bool;
}

impl PathEndsWith for Path {
	fn ends_with(&self, s: &str) -> bool {
		self.segments.last().map(|segment| segment.ident.to_string()).as_deref() == Some(s)
	}
}

pub(crate) trait WithSpan {
	fn with_span(self, span: Span) -> Self;
}

impl WithSpan for Error {
	fn with_span(self, span: Span) -> Self {
		let mut err: Option<Self> = None;
		for old_err in self {
			let new_err = Error::new(span, old_err);
			err = match err {
				Some(mut err) => {
					err.combine(new_err);
					Some(err)
				},
				None => Some(new_err)
			};
		}
		err.unwrap()
	}
}
