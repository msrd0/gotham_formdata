use syn::{Error, Path};

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
