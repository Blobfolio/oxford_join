/*!
# Oxford Join

[![Documentation](https://docs.rs/oxford_join/badge.svg)](https://docs.rs/oxford_join/)
[![crates.io](https://img.shields.io/crates/v/oxford_join.svg)](https://crates.io/crates/oxford_join)

Join a slice of strings with [Oxford Commas](https://en.wikipedia.org/wiki/Serial_comma) inserted as necessary, using the [`Conjunction`] of your choice.

(You know, as it should be. Haha.)

The return formatting depends on the size of the set:

```ignore,text
0: ""
1: "first"
2: "first <CONJUNCTION> last"
n: "first, second, …, <CONJUNCTION> last"
```

## Examples

The magic is accomplished with the [`OxfordJoin`] trait. Import that, and most
slice-y things holding `AsRef<str>` will inherit the [`OxfordJoin::oxford_join`]
method for joining.

```
use oxford_join::{Conjunction, OxfordJoin};

let set = ["Apples", "Oranges"];
assert_eq!(set.oxford_join(Conjunction::And), "Apples and Oranges");

let set = ["Apples", "Oranges", "Bananas"];
assert_eq!(set.oxford_join(Conjunction::And), "Apples, Oranges, and Bananas");

// There are also shorthand methods for and, or, and_or, and nor, allowing you
// to skip the Conjunction enum entirely.
assert_eq!(set.oxford_and(), "Apples, Oranges, and Bananas");
assert_eq!(set.oxford_and_or(), "Apples, Oranges, and/or Bananas");
assert_eq!(set.oxford_nor(), "Apples, Oranges, nor Bananas");
assert_eq!(set.oxford_or(), "Apples, Oranges, or Bananas");
```

That's all, folks!
*/

#![warn(clippy::filetype_is_file)]
#![warn(clippy::integer_division)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(macro_use_extern_crate)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

use std::{
	borrow::{
		Borrow,
		Cow,
	},
	fmt,
	ops::Deref,
};



#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
/// # Conjunction.
///
/// This is the glue used to bind the last entry in an [`oxford_join`]ed set.
///
/// If you're doing something weird and the preset entries aren't currint it
/// for you, you can use [`Conjunction::Other`], which wraps an `&str`. This
/// value should just be the word/symbol; any outer whitespace is provided
/// during join.
///
/// ## Examples.
///
/// If a set has exactly two items:
/// ```ignore,text
/// first <CONJUNCTION> last
/// ```
///
/// If a set has three or more items:
/// ```ignore,text
/// first, second, …, <CONJUNCTION> last
/// ```
///
/// If the set is empty or singular, there's nothing to conjunct.
pub enum Conjunction<'a> {
	/// # Ampersand (&).
	Ampersand,

	/// # And.
	And,

	/// # And/Or.
	AndOr,

	/// # Nor.
	Nor,

	/// # Or.
	Or,

	/// # Custom Entry (Trimmed).
	Other(&'a str),

	/// # Plus (+).
	Plus,
}

impl AsRef<str> for Conjunction<'_> {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl Borrow<str> for Conjunction<'_> {
	#[inline]
	fn borrow(&self) -> &str { self.as_str() }
}

impl Default for Conjunction<'_> {
	#[inline]
	fn default() -> Self { Self::And }
}

impl Deref for Conjunction<'_> {
	type Target = str;
	#[inline]
	fn deref(&self) -> &Self::Target { self.as_str() }
}

impl fmt::Display for Conjunction<'_> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl<'a> From<&'a str> for Conjunction<'a> {
	#[inline]
	fn from(src: &'a str) -> Self { Self::Other(src.trim()) }
}

impl Conjunction<'_> {
	#[must_use]
	/// # As Str.
	///
	/// Return the conjunction as a string slice.
	pub const fn as_str(&self) -> &str {
		match self {
			Self::Ampersand => "&",
			Self::And => "and",
			Self::AndOr => "and/or",
			Self::Nor => "nor",
			Self::Or => "or",
			Self::Other(s) => s,
			Self::Plus => "+",
		}
	}

	#[must_use]
	/// # Length.
	///
	/// Return the string length of the conjunction.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::Conjunction;
	///
	/// for i in [
	///     Conjunction::Ampersand, Conjunction::And,
	///     Conjunction::AndOr, Conjunction::Nor,
	///     Conjunction::Or, Conjunction::Plus
	/// ] {
	///     assert_eq!(i.as_str().len(), i.len());
	/// }
	/// ```
	pub const fn len(&self) -> usize {
		match self {
			Self::And | Self::Nor => 3,
			Self::Or => 2,
			Self::Ampersand | Self::Plus => 1,
			Self::AndOr => 6,
			Self::Other(s) => s.len(),
		}
	}

	#[must_use]
	/// # Is Empty.
	///
	/// An empty conjunction makes no sense, but because `Conjunction::Other`
	/// wraps arbitrary values, it is worth checking.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::Conjunction;
	///
	/// assert_eq!(Conjunction::And.is_empty(), false);
	/// assert_eq!(Conjunction::Other("foo").is_empty(), false);
	/// assert_eq!(Conjunction::Other("").is_empty(), true);
	/// ```
	pub const fn is_empty(&self) -> bool {
		match self {
			Self::Other(s) => s.is_empty(),
			_ => false,
		}
	}
}



/// # Oxford Join.
///
/// Join a slice of strings with Oxford Commas inserted as necessary.
///
/// The return formatting depends on the size of the set:
///
/// ```ignore,text
/// "" // Zero.
/// "first" // One.
/// "first <CONJUNCTION> last" // Two.
/// "first, second, …, <CONJUNCTION> last" // Three+.
/// ```
///
/// ## Examples
///
/// ```
/// use oxford_join::{Conjunction, OxfordJoin};
///
/// let set = ["Apples"];
/// assert_eq!(set.oxford_join(Conjunction::And), "Apples");
///
/// let set = ["Apples", "Oranges"];
/// assert_eq!(set.oxford_join(Conjunction::And), "Apples and Oranges");
///
/// let set = ["Apples", "Oranges", "Bananas"];
/// assert_eq!(set.oxford_join(Conjunction::And), "Apples, Oranges, and Bananas");
/// ```
pub trait OxfordJoin {
	/// # Oxford Join.
	///
	/// Join a slice of strings with Oxford Commas inserted as necessary.
	fn oxford_join(&self, glue: Conjunction) -> Cow<str>;

	#[inline]
	/// # Oxford Join (and).
	///
	/// This is equivalent to calling `oxford_join(Conjunction::And)`.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::{Conjunction, OxfordJoin};
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(set.oxford_join(Conjunction::And), set.oxford_and());
	/// ```
	fn oxford_and(&self) -> Cow<str> { self.oxford_join(Conjunction::And) }

	#[inline]
	/// # Oxford Join (and/or).
	///
	/// This is equivalent to calling `oxford_join(Conjunction::AndOr)`.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::{Conjunction, OxfordJoin};
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(set.oxford_join(Conjunction::AndOr), set.oxford_and_or());
	/// ```
	fn oxford_and_or(&self) -> Cow<str> { self.oxford_join(Conjunction::AndOr) }

	#[inline]
	/// # Oxford Join (nor).
	///
	/// This is equivalent to calling `oxford_join(Conjunction::Nor)`.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::{Conjunction, OxfordJoin};
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(set.oxford_join(Conjunction::Nor), set.oxford_nor());
	/// ```
	fn oxford_nor(&self) -> Cow<str> { self.oxford_join(Conjunction::Nor) }

	#[inline]
	/// # Oxford Join (or).
	///
	/// This is equivalent to calling `oxford_join(Conjunction::Or)`.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::{Conjunction, OxfordJoin};
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(set.oxford_join(Conjunction::Or), set.oxford_or());
	/// ```
	fn oxford_or(&self) -> Cow<str> { self.oxford_join(Conjunction::Or) }
}

impl<T> OxfordJoin for [T] where T: AsRef<str> {
	/// # Oxford Join.
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		match self.len() {
			0 => Cow::Borrowed(""),
			1 => Cow::Borrowed(self[0].as_ref()),
			2 => Cow::Owned(join_two(self[0].as_ref(), self[1].as_ref(), glue)),
			n => {
				let glue = glue.as_str();
				let len = glue.len() + 1 + ((n - 1) << 1) + slice_len(self);

				let (last, rest) = self.split_last().unwrap();
				let mut base: String = String::with_capacity(len);
				for s in rest {
					base.push_str(s.as_ref());
					base.push_str(", ");
				}

				base.push_str(glue);
				base.push(' ');

				base.push_str(last.as_ref());

				Cow::Owned(base)
			},
		}
	}
}

impl<T> OxfordJoin for [T; 1] where T: AsRef<str> {
	#[inline]
	/// # Oxford Join.
	///
	/// This is a special case; the only array entry will be returned as-is.
	fn oxford_join(&self, _glue: Conjunction) -> Cow<str> {
		Cow::Borrowed(self[0].as_ref())
	}
}

impl<T> OxfordJoin for [T; 2] where T: AsRef<str> {
	#[inline]
	/// # Oxford Join.
	///
	/// This is a special case; it will always read "first <CONJUNCTION> last".
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		Cow::Owned(join_two(self[0].as_ref(), self[1].as_ref(), glue))
	}
}

/// # Join Arrays.
macro_rules! join_arrays {
	($($num:literal),+) => ($(
		impl<T> OxfordJoin for [T; $num] where T: AsRef<str> {
			/// # Oxford Join.
			fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
				let len = glue.len() + 1_usize + ($num << 1) + slice_len(self.as_slice());

				let mut base: String = String::with_capacity(len);
				for s in self.iter().take($num - 1) {
					base.push_str(s.as_ref());
					base.push_str(", ");
				}

				base.push_str(glue.as_str());
				base.push(' ');

				base.push_str(self[$num - 1].as_ref());

				Cow::Owned(base)
			}
		}
	)+);
}

join_arrays!(
	3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
	23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);



/// # Slice Length.
fn slice_len<T>(src: &[T]) -> usize
where T: AsRef<str> {
	src.iter().map(|x| x.as_ref().len()).sum()
}

/// # Join Two.
fn join_two(a: &str, b: &str, glue: Conjunction) -> String {
	let a = a.as_bytes();
	let b = b.as_bytes();
	let glue = glue.as_str().as_bytes();

	let mut v: Vec<u8> = Vec::with_capacity(a.len() + b.len() + 2 + glue.len());
	v.extend_from_slice(a);
	v.push(b' ');
	v.extend_from_slice(glue);
	v.push(b' ');
	v.extend_from_slice(b);

	// Safety: all inputs were valid UTF-8, so the output is too.
	unsafe { String::from_utf8_unchecked(v) }
}



#[cfg(test)]
mod tests { use brunch as _; }
