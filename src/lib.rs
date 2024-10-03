/*!
# Oxford Join

[![docs.rs](https://img.shields.io/docsrs/oxford_join.svg?style=flat-square&label=docs.rs)](https://docs.rs/oxford_join/)
[![changelog](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/oxford_join/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=crates.io)](https://crates.io/crates/oxford_join)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/oxford_join/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/oxford_join/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/oxford_join/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/oxford_join)<br>
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

That's all, folks!
*/

#![deny(
	// TODO: clippy::allow_attributes_without_reason,
	clippy::correctness,
	unreachable_pub,
	unsafe_code,
)]

#![warn(
	clippy::complexity,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::style,

	// TODO: clippy::allow_attributes,
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

#![no_std]

extern crate alloc;

use alloc::{
	borrow::Cow,
	collections::{
		BTreeSet,
		BTreeMap,
	},
	string::String,
	vec::Vec,
};
use core::{
	borrow::Borrow,
	fmt,
	ops::Deref,
};



/// # Comma + Space.
const COMMASPACE: &[u8] = b", ";



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
	/// If your data type implements [`OxfordJoin`], you should use its
	/// [`OxfordJoin::oxford_join`] implementation instead as that will be
	/// faster — they're _specialized_ — but you'll get the same result either
	/// way.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::{Conjunction, OxfordJoin};
	/// const LIST: [&str; 3] = ["Apples", "Bananas", "Carrots"];
	///
	/// // Arrays, for example, implement OxfordJoin, so you should leverage
	/// // the trait method instead.
	/// assert_eq!(LIST.oxford_join(Conjunction::And), "Apples, Bananas, and Carrots");
	///
	/// // But if you use this method anyway, you'll get the same answer:
	/// assert_eq!(Conjunction::And.oxford_join(LIST), "Apples, Bananas, and Carrots");
	///
	/// // A more appropriate use case for this method would be something like
	/// // the following:
	/// assert_eq!(
	///     Conjunction::And.oxford_join("hello".chars().map(|c| c.to_string())),
	///     "h, e, l, l, and o"
	/// );
	/// ```
	pub fn oxford_join<I, T>(&self, iter: I) -> String
	where T: AsRef<str>, I: IntoIterator<Item=T> {
		iter.into_iter().collect::<Vec<_>>().oxford_join(*self).into_owned()
	}
}

impl Conjunction<'_> {
	/// # Append for Three+.
	///
	/// This writes the conjunction with a leading comma-space and trailing
	/// space to the buffer, e.g. `", and "`.
	fn append_to(&self, v: &mut Vec<u8>) {
		match self {
			Self::Ampersand => { v.extend_from_slice(b", & "); },
			Self::And => { v.extend_from_slice(b", and "); },
			Self::AndOr => { v.extend_from_slice(b", and/or "); },
			Self::Nor => { v.extend_from_slice(b", nor "); },
			Self::Or => { v.extend_from_slice(b", or "); },
			Self::Other(s) => {
				v.extend_from_slice(COMMASPACE);
				v.extend_from_slice(s.as_bytes());
				v.push(b' ');
			},
			Self::Plus => { v.extend_from_slice(b", + "); },
		}
	}

	/// # Append for Two.
	///
	/// This writes the conjunction with a leading and trailing space to the
	/// buffer, e.g. `" and "`.
	fn append_two(&self, v: &mut Vec<u8>) {
		match self {
			Self::Ampersand => { v.extend_from_slice(b" & "); },
			Self::And => { v.extend_from_slice(b" and "); },
			Self::AndOr => { v.extend_from_slice(b" and/or "); },
			Self::Nor => { v.extend_from_slice(b" nor "); },
			Self::Or => { v.extend_from_slice(b" or "); },
			Self::Other(s) => {
				v.push(b' ');
				v.extend_from_slice(s.as_bytes());
				v.push(b' ');
			},
			Self::Plus => { v.extend_from_slice(b" + "); },
		}
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
	#[allow(unsafe_code)] // Strings in, strings out.
	/// # Oxford Join.
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		// 2+ elements.
		if let [first, mid @ .., last] = self {
			let first = first.as_ref().as_bytes();
			let last = last.as_ref().as_bytes();

			// 2 elements.
			if mid.is_empty() {
				let len = first.len() + last.len() + 2 + glue.len();
				let mut v = Vec::with_capacity(len);
				v.extend_from_slice(first); // First.
				glue.append_two(&mut v);    // Conjunction.
				v.extend_from_slice(last);  // Last.

				// Safety: strings in, strings out.
				let out = unsafe { String::from_utf8_unchecked(v) };
				Cow::Owned(out)
			}
			// 3+ elements.
			else {
				let len =
					glue.len() + 1 +                                     // Glue length plus one trailing space.
					((mid.len() + 1) * 2) +                              // Commaspace (2) for all but last entry.
					first.len() + last.len() +                           // First and last item length.
					mid.iter().map(|x| x.as_ref().len()).sum::<usize>(); // All other item lengths.
				let mut v = Vec::with_capacity(len);

				// Write the first.
				v.extend_from_slice(first);

				// Write the middles.
				for s in mid {
					v.extend_from_slice(COMMASPACE);
					v.extend_from_slice(s.as_ref().as_bytes());
				}

				// Write the conjunction and last.
				glue.append_to(&mut v);
				v.extend_from_slice(last);

				// Safety: strings in, strings out.
				let out = unsafe { String::from_utf8_unchecked(v) };
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
	#[allow(unsafe_code)] // Strings in, strings out.
	#[inline]
	/// # Oxford Join.
	///
	/// This is a special case; it will always read "first CONJUNCTION last".
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		let a = self[0].as_ref().as_bytes();
		let b = self[1].as_ref().as_bytes();

		let len = a.len() + b.len() + 2 + glue.len();
		let mut v = Vec::with_capacity(len);
		v.extend_from_slice(a);  // First.
		glue.append_two(&mut v); // Conjunction.
		v.extend_from_slice(b);  // Last.

		// Safety: strings in, strings out.
		let out = unsafe { String::from_utf8_unchecked(v) };
		Cow::Owned(out)
	}
}

/// # Join Arrays (3+).
macro_rules! join_arrays {
	($($num:literal $pad:literal $last:literal),+ $(,)?) => ($(
		impl<T> OxfordJoin for [T; $num] where T: AsRef<str> {
			#[allow(unsafe_code)] // Strings in, strings out.
			/// # Oxford Join.
			fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
				let len = glue.len() + $pad + self.iter().map(|x| x.as_ref().len()).sum::<usize>();
				let [first, mid @ .., last] = self;
				let mut v = Vec::with_capacity(len);

				// Write the first.
				v.extend_from_slice(first.as_ref().as_bytes());

				// Write the middles.
				for s in mid {
					v.extend_from_slice(COMMASPACE);
					v.extend_from_slice(s.as_ref().as_bytes());
				}

				// Write the conjunction and last.
				glue.append_to(&mut v);
				v.extend_from_slice(last.as_ref().as_bytes());

				// Safety: strings in, strings out.
				let out = unsafe { String::from_utf8_unchecked(v) };
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
		#[allow(unsafe_code)] // Strings in, strings out.
		/// # Oxford Join.
		fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
			match self.len() {
				0 => Cow::Borrowed(""),
				1 => Cow::Borrowed(self.$iter().next().unwrap().as_ref()),
				2 => {
					let mut iter = self.$iter();
					let a = iter.next().unwrap().as_ref().as_bytes();
					let b = iter.next().unwrap().as_ref().as_bytes();

					let len = a.len() + b.len() + 2 + glue.len();
					let mut v = Vec::with_capacity(len);
					v.extend_from_slice(a);  // First.
					glue.append_two(&mut v); // Conjunction.
					v.extend_from_slice(b);  // Last.

					// Safety: strings in, strings out.
					let out = unsafe { String::from_utf8_unchecked(v) };
					Cow::Owned(out)
				},
				n => {
					let last = n - 1;
					let len = glue.len() + 1 + last * 2 + self.$iter().map(|x| x.as_ref().len()).sum::<usize>();

					let mut v = Vec::with_capacity(len);
					let mut iter = self.$iter();

					// Write the first.
					v.extend_from_slice(iter.next().unwrap().as_ref().as_bytes());

					// Write the middles. (Last is count minus one, but since
					// we already wrote an entry, we need to subtract one
					// again.)
					for s in iter.by_ref().take(last - 1) {
						v.extend_from_slice(COMMASPACE);
						v.extend_from_slice(s.as_ref().as_bytes());
					}

					// Write the conjunction and last.
					glue.append_to(&mut v);
					v.extend_from_slice(iter.next().unwrap().as_ref().as_bytes());

					// Safety: strings in, strings out.
					let out = unsafe { String::from_utf8_unchecked(v) };
					Cow::Owned(out)
				},
			}
		}
	);
}

impl<K, T> OxfordJoin for BTreeMap<K, T> where T: AsRef<str> { join_btrees!(values); }

impl<T> OxfordJoin for BTreeSet<T> where T: AsRef<str> { join_btrees!(iter); }



#[cfg(test)]
mod tests {
	use super::*;
	use brunch as _;

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
	#[allow(clippy::cognitive_complexity)] // It is what it is.
	fn t_fruit() {
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
		}

		assert!(Conjunction::Other("").is_empty());
	}

	#[test]
	fn conjunction_append() {
		for c in CTEST {
			// Two.
			let s = [" ", c.as_str(), " "].concat();
			let mut v = Vec::new();
			c.append_two(&mut v);
			assert_eq!(v, s.as_bytes());

			// Three+.
			let s = [", ", c.as_str(), " "].concat();
			v.truncate(0);
			c.append_to(&mut v);
			assert_eq!(v, s.as_bytes());
		}
	}
}
