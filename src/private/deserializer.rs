use paste::paste;
use serde::de::{self, Visitor};
use std::{
	borrow::{Borrow, Cow},
	fmt::{self, Display},
	ops::Deref
};

#[derive(Debug)]
pub struct Error(pub(super) String);

impl de::Error for Error {
	fn custom<T: Display>(msg: T) -> Self {
		Error(msg.to_string())
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Deserializer<'de> {
	Bytes(Vec<u8>),
	Text(Cow<'de, str>)
}

enum Borrowed<'short, 'long: 'short, T: ?Sized + ToOwned> {
	Short(&'short T),
	Long(&'long T),
	Owned(<T as ToOwned>::Owned)
}

impl<'short, 'long: 'short, T: ?Sized + ToOwned> Deref for Borrowed<'short, 'long, T> {
	type Target = T;

	fn deref(&self) -> &T {
		match *self {
			Self::Short(b) => b,
			Self::Long(b) => b,
			Self::Owned(ref o) => o.borrow()
		}
	}
}

impl<'de> Deserializer<'de> {
	fn text(&self) -> Borrowed<'_, 'de, str> {
		match &self {
			Self::Bytes(b) => match String::from_utf8_lossy(&b) {
				Cow::Borrowed(s) => Borrowed::Short(s),
				Cow::Owned(s) => Borrowed::Owned(s)
			},
			Self::Text(Cow::Borrowed(s)) => Borrowed::Long(s),
			Self::Text(Cow::Owned(s)) => Borrowed::Short(&s)
		}
	}

	fn bytes(&self) -> Borrowed<'_, 'de, [u8]> {
		match self {
			Self::Bytes(b) => Borrowed::Short(&b),
			Self::Text(Cow::Borrowed(s)) => Borrowed::Long(s.as_bytes()),
			Self::Text(Cow::Owned(s)) => Borrowed::Short(s.as_bytes())
		}
	}
}

macro_rules! impl_deserializer {
	(
		$(
			const $const:ident = |$this:ident, $visitor:ident| { $($body:tt)* };
		)*
		$(
			parse $parse:ty;
		)*
		$(
			forward $forward:ident to $to:ident;
		)*
		$(
			fn $fn:ident<$V:ident>($this0:ident $(, $arg:ident : $ty:ty)*) { $($body0:tt)* }
		)*
	) => {
		impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
			type Error = Error;
			paste! {
				$(
					fn $const<V>(self, $visitor: V) -> Result<V::Value>
					where
						V: Visitor<'de>
					{
						let $this: &'a mut Deserializer<'de> = self;
						$($body)*
					}
				)*

				$(
					fn [<deserialize_ $parse>]<V>(self, visitor: V) -> Result<V::Value>
					where
						V: Visitor<'de>
					{
						let value = self.text().parse::<$parse>().map_err(|e| Error(e.to_string()))?;
						visitor.[<visit_ $parse>](value)
					}
				)*

				$(
					fn $forward<V>(self, visitor: V) -> Result<V::Value>
					where
						V: Visitor<'de>
					{
						self.$to(visitor)
					}
				)*

				$(
					fn $fn<$V>(self$(, $arg : $ty)*) -> Result<$V::Value>
					where
						$V: Visitor<'de>
					{
						let $this0 = self;
						$($body0)*
					}
				)*
			}
		}
	};
}

impl_deserializer! {
	const deserialize_any = |this, visitor| {
		match this {
			Deserializer::Bytes(b) => visitor.visit_bytes(&b),
			Deserializer::Text(Cow::Borrowed(s)) => visitor.visit_borrowed_str(s),
			Deserializer::Text(Cow::Owned(s)) => visitor.visit_str(&s)
		}
	};

	const deserialize_bool = |this, visitor| {
		let text = this.text().to_ascii_lowercase();
		let value = text == "true" || text == "y" || text == "on" || text == "1";
		visitor.visit_bool(value)
	};

	const deserialize_str = |this, visitor| {
		match this.text() {
			Borrowed::Short(s) => visitor.visit_str(s),
			Borrowed::Long(s) => visitor.visit_borrowed_str(s),
			Borrowed::Owned(s) => visitor.visit_string(s)
		}
	};

	const deserialize_bytes = |this, visitor| {
		match this.bytes() {
			Borrowed::Short(b) => visitor.visit_bytes(b),
			Borrowed::Long(b) => visitor.visit_borrowed_bytes(b),
			Borrowed::Owned(b) => visitor.visit_byte_buf(b)
		}
	};

	parse i8;
	parse i16;
	parse i32;
	parse i64;
	parse i128;
	parse u8;
	parse u16;
	parse u32;
	parse u64;
	parse u128;
	parse f32;
	parse f64;

	forward deserialize_char to deserialize_any;
	forward deserialize_string to deserialize_str;
	forward deserialize_byte_buf to deserialize_bytes;
	forward deserialize_option to deserialize_any;
	forward deserialize_unit to deserialize_any;
	forward deserialize_seq to deserialize_any;
	forward deserialize_map to deserialize_any;
	forward deserialize_identifier to deserialize_any;
	forward deserialize_ignored_any to deserialize_any;

	fn deserialize_unit_struct<V>(this, _name: &'static str, visitor: V) {
		this.deserialize_any(visitor)
	}

	fn deserialize_newtype_struct<V>(this, _name: &'static str, visitor: V) {
		this.deserialize_any(visitor)
	}

	fn deserialize_tuple<V>(this, _len: usize, visitor: V) {
		this.deserialize_any(visitor)
	}

	fn deserialize_tuple_struct<V>(this, _name: &'static str, _len: usize, visitor: V) {
		this.deserialize_any(visitor)
	}

	fn deserialize_struct<V>(this, _name: &'static str, _fields: &'static [&'static str], visitor: V) {
		this.deserialize_any(visitor)
	}

	fn deserialize_enum<V>(this, _name: &'static str, _variants: &'static [&'static str], visitor: V) {
		this.deserialize_any(visitor)
	}
}
