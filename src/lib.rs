/*!
# Oxford Join

[![docs.rs](https://img.shields.io/docsrs/oxford_join.svg?style=flat-square&label=docs.rs)](https://docs.rs/oxford_join/)
[![changelog](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/oxford_join/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=crates.io)](https://crates.io/crates/oxford_join)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/oxford_join/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/oxford_join/actions)
[![deps.rs](https://deps.rs/crate/oxford_join/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/oxford_join/)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/oxford_join/issues)

Join a slice of strings with [Oxford Commas](https://en.wikipedia.org/wiki/Serial_comma) inserted as necessary, using the [`Conjunction`] of your choice.

(You know, as it should be. Haha.)

The return formatting depends on the size of the set:

```text
0: ""
1: "first"
2: "first <CONJUNCTION> last"
n: "first, second, …, <CONJUNCTION> last"
```

This crate is `#![no_std]`-compatible.

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

There is also a [`Display`](core::fmt::Display)-based [`OxfordJoinFmt`] wrapper
that can be more efficient for `format!`-type use cases, or types which
implement `Display` but not `AsRef<str>`.

```
use oxford_join::OxfordJoinFmt;
let set = ["Apples", "Oranges", "Bananas"];
assert_eq!(
    format!("I eat {}.", OxfordJoinFmt::and(&set)),
    "I eat Apples, Oranges, and Bananas.",
);
```

That's all, folks!
*/

#![forbid(unsafe_code)]

#![deny(
	clippy::allow_attributes_without_reason,
	clippy::correctness,
	unreachable_pub,
)]

#![warn(
	clippy::complexity,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::style,

	clippy::allow_attributes,
	clippy::clone_on_ref_ptr,
	clippy::create_dir,
	clippy::filetype_is_file,
	clippy::format_push_string,
	clippy::get_unwrap,
	clippy::impl_trait_in_params,
	clippy::lossy_float_literal,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::needless_raw_strings,
	clippy::panic_in_result_fn,
	clippy::pub_without_shorthand,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::semicolon_inside_block,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::todo,
	clippy::undocumented_unsafe_blocks,
	clippy::unneeded_field_pattern,
	clippy::unseparated_literal_suffix,
	clippy::unwrap_in_result,

	macro_use_extern_crate,
	missing_copy_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]

#![allow(clippy::module_name_repetitions, reason = "Repetition is preferred.")]

#![no_std]

extern crate alloc;

mod fmt;

// Re-export.
pub use fmt::{
	JoinFmt,
	OxfordJoinFmt,
};

use alloc::{
	borrow::Cow,
	collections::{
		BTreeSet,
		BTreeMap,
	},
	string::String,
};
use core::borrow::Borrow;



#[derive(Debug, Copy, Clone, Default, Eq, Hash, PartialEq)]
/// # Conjunction.
///
/// This is the glue used to bind the last entry in an [`oxford_join`](OxfordJoin::oxford_join)ed set.
///
/// If you're doing something weird and the preset entries aren't currint it
/// for you, you can use [`Conjunction::Other`], which wraps an `&str`. This
/// value should just be the word/symbol; surrounding whitespace and
/// punctuation are added during the join as needed.
///
/// ## Examples.
///
/// If a set has exactly two items:
/// ```text
/// first <CONJUNCTION> last
/// ```
///
/// If a set has three or more items:
/// ```text
/// first, second, …, <CONJUNCTION> last
/// ```
///
/// If the set is empty or singular, there's nothing to conjunct.
pub enum Conjunction<'a> {
	/// # Ampersand (&).
	Ampersand,

	#[default]
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

impl core::fmt::Display for Conjunction<'_> {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		<str as core::fmt::Display>::fmt(self.as_str(), f)
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

	/// # As Str w/ Padding.
	///
	/// Return the conjunction as a string slice with spaces on either end,
	/// unless custom, which passes through as an error.
	const fn as_str_2(&self) -> Result<&'static str, &str> {
		match self {
			Self::Ampersand => Ok(" & "),
			Self::And => Ok(" and "),
			Self::AndOr => Ok(" and/or "),
			Self::Nor => Ok(" nor "),
			Self::Or => Ok(" or "),
			Self::Other(s) => Err(s),
			Self::Plus => Ok(" + "),
		}
	}

	/// # As Str w/ Comma and Padding.
	///
	/// Return the conjunction as a string slice starting with a ", " and
	/// ending with a space, unless custom.
	const fn as_str_n(&self) -> Result<&'static str, &str> {
		match self {
			Self::Ampersand => Ok(", & "),
			Self::And => Ok(", and "),
			Self::AndOr => Ok(", and/or "),
			Self::Nor => Ok(", nor "),
			Self::Or => Ok(", or "),
			Self::Other(s) => Err(s),
			Self::Plus => Ok(", + "),
		}
	}

	#[must_use]
	/// # Length.
	///
	/// Return the string length of the conjunction.
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
	pub const fn is_empty(&self) -> bool {
		match self {
			Self::Other(s) => s.is_empty(),
			_ => false,
		}
	}
}

impl Conjunction<'_> {
	/// # Oxford Join (Generic).
	///
	/// This convenience method allows you to Oxford-join _any_ iterable data
	/// source that yields `AsRef<str>`.
	///
	/// For types that implement [`OxfordJoin`] directly, the trait methods
	/// should be preferred as they're specialized, but you'll get the same
	/// answer either way.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::{Conjunction, OxfordJoin};
	/// const LIST: [&str; 3] = ["Apples", "Bananas", "Carrots"];
	///
	/// // A contrived example to spell it out…
	/// assert_eq!(
	///     Conjunction::And.oxford_join("hello".chars().map(String::from)),
	///     "h, e, l, l, and o"
	/// );
	/// ```
	pub fn oxford_join<I, T>(&self, iter: I) -> String
	where T: AsRef<str>, I: IntoIterator<Item=T> {
		// Pull the first value, ensuring there actually is one.
		let mut iter = iter.into_iter();
		let Some(next) = iter.next() else { return String::new(); };

		// MAGIC NUMBER: one fuzzy preallocation improves collection times a
		// lot compared to separate item-by-item reserves.
		let mut out = String::with_capacity(64);
		out.push_str(next.as_ref());

		// We have a second item!
		if let Some(mut buf) = iter.next() {
			// Can we get an Nth?!
			let mut many = false;
			for next in iter.map(|n| core::mem::replace(&mut buf, n)) {
				// Add the _previous_ value to the output. (The "current" value
				// is now in the buffer.)
				out.push_str(", ");
				out.push_str(next.as_ref());
				many = true;
			}

			// Add the final punctuation and conjunction.
			if many { out.push_str(", "); } else { out.push(' '); }
			out.push_str(self.as_str());
			out.push(' ');

			// Cap it off with the last item.
			out.push_str(buf.as_ref());
		}

		out
	}
}



/// # Oxford Join.
///
/// Join a slice of strings with Oxford Commas inserted as necessary.
///
/// The return formatting depends on the size of the set:
///
/// ```text
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
/// assert_eq!(set.oxford_join(Conjunction::Or), "Apples or Oranges");
///
/// let set = ["Apples", "Oranges", "Bananas"];
/// assert_eq!(set.oxford_join(Conjunction::AndOr), "Apples, Oranges, and/or Bananas");
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
		// 2+ elements.
		if let [first, mid @ .., last] = self {
			let first = first.as_ref();
			let last = last.as_ref();

			// 2 elements.
			if mid.is_empty() { Cow::Owned(two_and_glue(first, last, glue)) }
			// 3+ elements.
			else {
				let len =
					glue.len() + 1 +                                     // Glue length plus one trailing space.
					((mid.len() + 1) * 2) +                              // Commaspace (2) for all but last entry.
					first.len() + last.len() +                           // First and last item length.
					mid.iter().map(|x| x.as_ref().len()).sum::<usize>(); // All other item lengths.
				let mut out = String::with_capacity(len);

				// Write the first.
				out.push_str(first);

				// Write the middles.
				for s in mid {
					out.push_str(", ");
					out.push_str(s.as_ref());
				}

				// Final glue.
				match glue.as_str_n() {
					Ok(s) => { out.push_str(s); },
					Err(s) => {
						out.push_str(", ");
						out.push_str(s);
						out.push(' ');
					}
				}

				// Write the last.
				out.push_str(last.as_ref());

				Cow::Owned(out)
			}
		}
		// One element.
		else if self.len() == 1 { Cow::Borrowed(self[0].as_ref()) }
		// No elements.
		else { Cow::Borrowed("") }
	}
}

impl<T> OxfordJoin for [T; 0] where T: AsRef<str> {
	#[inline]
	/// # Oxford Join.
	///
	/// This is a special case; the result is always empty.
	fn oxford_join(&self, _glue: Conjunction) -> Cow<str> { Cow::Borrowed("") }
}

impl<T> OxfordJoin for [T; 1] where T: AsRef<str> {
	#[inline]
	/// # Oxford Join.
	///
	/// This is a special case; the sole entry will be returned as-is.
	fn oxford_join(&self, _glue: Conjunction) -> Cow<str> {
		Cow::Borrowed(self[0].as_ref())
	}
}

impl<T> OxfordJoin for [T; 2] where T: AsRef<str> {
	#[inline]
	/// # Oxford Join.
	///
	/// This is a special case; it will always read "first CONJUNCTION last".
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		Cow::Owned(two_and_glue(self[0].as_ref(), self[1].as_ref(), glue))
	}
}

/// # Join Arrays (3+).
macro_rules! join_arrays {
	($($num:literal $pad:literal $last:literal),+ $(,)?) => ($(
		impl<T> OxfordJoin for [T; $num] where T: AsRef<str> {
			/// # Oxford Join.
			fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
				let len = glue.len() + $pad + self.iter().map(|x| x.as_ref().len()).sum::<usize>();
				let [first, mid @ .., last] = self;
				let mut out = String::with_capacity(len);

				// Write the first.
				out.push_str(first.as_ref());

				// Write the middles.
				for s in mid {
					out.push_str(", ");
					out.push_str(s.as_ref());
				}

				// Final glue.
				match glue.as_str_n() {
					Ok(s) => { out.push_str(s); },
					Err(s) => {
						out.push_str(", ");
						out.push_str(s);
						out.push(' ');
					}
				}

				// Write the last.
				out.push_str(last.as_ref());

				Cow::Owned(out)
			}
		}
	)+);
}

join_arrays!(
	 3  5  2,
	 4  7  3,
	 5  9  4,
	 6 11  5,
	 7 13  6,
	 8 15  7,
	 9 17  8,
	10 19  9,
	11 21 10,
	12 23 11,
	13 25 12,
	14 27 13,
	15 29 14,
	16 31 15,
	17 33 16,
	18 35 17,
	19 37 18,
	20 39 19,
	21 41 20,
	22 43 21,
	23 45 22,
	24 47 23,
	25 49 24,
	26 51 25,
	27 53 26,
	28 55 27,
	29 57 28,
	30 59 29,
	31 61 30,
	32 63 31,
);

/// # Helper: Binary Tree Joins.
macro_rules! join_btrees {
	($iter:ident) => (
		/// # Oxford Join.
		fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
			let mut iter = self.$iter();

			// Do we have a first?
			let Some(a) = iter.next() else { return Cow::Borrowed(""); };
			let a: &str = a.as_ref();

			// Can we get a second?
			let Some(b) = iter.next() else { return Cow::Borrowed(a); };
			let b: &str = b.as_ref();

			// How about a third?
			let Some(c) = iter.next() else {
				return Cow::Owned(two_and_glue(a, b, glue));
			};

			// We'll have N to deal with.
			let len = glue.len() + 1 + (self.len() - 1) * 2 + self.$iter().map(|x| x.as_ref().len()).sum::<usize>();
			let mut out = String::with_capacity(len);

			// Start with what we already know.
			out.push_str(a);
			out.push_str(", ");
			out.push_str(b);

			// Loop through the remainder, saving the last for last.
			let mut buf = c;
			for next in iter.map(|n| core::mem::replace(&mut buf, n)) {
				// Add the _previous_ value to the output. (The "current" value
				// is now in the buffer.)
				out.push_str(", ");
				out.push_str(next.as_ref());
			}

			// Final glue.
			match glue.as_str_n() {
				Ok(s) => { out.push_str(s); },
				Err(s) => {
					out.push_str(", ");
					out.push_str(s);
					out.push(' ');
				}
			}

			// Write the last.
			out.push_str(buf.as_ref());

			Cow::Owned(out)
		}
	);
}

impl<K, T> OxfordJoin for BTreeMap<K, T> where T: AsRef<str> { join_btrees!(values); }

impl<T> OxfordJoin for BTreeSet<T> where T: AsRef<str> { join_btrees!(iter); }



/// # Two and Glue!
///
/// Join two elements and some glue into a new string.
fn two_and_glue(a: &str, b: &str, glue: Conjunction<'_>) -> String {
	let len = a.len() + b.len() + 2 + glue.len();
	let mut out = String::with_capacity(len);
	out.push_str(a);
	match glue.as_str_2() {
		Ok(s) => { out.push_str(s); }
		Err(s) => {
			out.push(' ');
			out.push_str(s);
			out.push(' ');
		}
	}
	out.push_str(b);
	out
}



#[cfg(test)]
mod tests {
	use super::*;
	use brunch as _;
	use alloc::format;

	const CTEST: [Conjunction; 7] = [
		Conjunction::Ampersand,
		Conjunction::And,
		Conjunction::AndOr,
		Conjunction::Nor,
		Conjunction::Or,
		Conjunction::Other("Boo"),
		Conjunction::Plus,
	];

	#[test]
	#[expect(clippy::cognitive_complexity, reason = "It is what it is.")]
	fn t_fruit() {
		use alloc::string::ToString;

		// Make sure arrays, slices, vecs, boxes, etc., all work out the same
		// way.
		macro_rules! compare {
			($($arr:ident, $expected:literal),+ $(,)?) => ($(
				assert_eq!($arr.oxford_and(), $expected, "Array.");
				assert_eq!($arr.as_slice().oxford_and(), $expected, "Slice.");

				let v = $arr.to_vec();
				assert_eq!(v.oxford_and(), $expected, "Vec.");
				assert_eq!(v.into_boxed_slice().oxford_and(), $expected, "Box.");

				let v: BTreeMap<usize, &str> = $arr.into_iter().enumerate().collect();
				assert_eq!(v.oxford_and(), $expected, "BTreeMap.");

				let v = BTreeSet::from($arr);
				assert_eq!(v.oxford_and(), $expected, "BTreeSet.");

				assert_eq!(
					OxfordJoinFmt::and($arr.as_slice()).to_string(),
					$expected,
					"OxfordJoinFmt::to_string",
				);
			)+);
		}

		const ARR0: [&str; 0] = [];
		const ARR1: [&str; 1] = ["Apples"];
		const ARR2: [&str; 2] = ["Apples", "Bananas"];
		const ARR3: [&str; 3] = ["Apples", "Bananas", "Carrots"];
		const ARR4: [&str; 4] = ["Apples", "Bananas", "Carrots", "Dates"];
		const ARR5: [&str; 5] = ["Apples", "Bananas", "Carrots", "Dates", "Eggplant"];
		const ARR32: [&str; 32] = [
			"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
			"G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V",
		];

		compare!(
			ARR0, "",
			ARR1, "Apples",
			ARR2, "Apples and Bananas",
			ARR3, "Apples, Bananas, and Carrots",
			ARR4, "Apples, Bananas, Carrots, and Dates",
			ARR5, "Apples, Bananas, Carrots, Dates, and Eggplant",
			ARR32, "0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, and V",
		);
	}

	#[test]
	fn conjunction_len() {
		for c in CTEST {
			assert_eq!(c.len(), c.as_str().len());
			assert!(! c.is_empty());

			// Version two.
			match c.as_str_2() {
				// Should match if spaces are added.
				Ok(s) => {
					assert_eq!(
						format!(" {} ", c.as_str()),
						s,
					);
				},
				// Should match exactly because custom.
				Err(s) => { assert_eq!(c.as_str(), s); },
			}

			// Version N.
			match c.as_str_n() {
				// Should match with stuff on the ends.
				Ok(s) => {
					assert_eq!(
						format!(", {} ", c.as_str()),
						s,
					);
				},
				// Should match exactly because custom.
				Err(s) => { assert_eq!(c.as_str(), s); },
			}
		}

		assert!(Conjunction::Other("").is_empty());
	}
}
