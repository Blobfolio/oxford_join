/*!
# Oxford Join

[![Documentation](https://docs.rs/oxford_join/badge.svg)](https://docs.rs/oxford_join/)
[![Changelog](https://img.shields.io/crates/v/oxford_join.svg?label=Changelog&color=9cf)](https://github.com/Blobfolio/oxford_join/blob/master/CHANGELOG.md)
[![crates.io](https://img.shields.io/crates/v/oxford_join.svg)](https://crates.io/crates/oxford_join)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/Blobfolio/oxford_join)

Join a slice of strings with [Oxford Commas](https://en.wikipedia.org/wiki/Serial_comma) inserted as necessary, using the [`Conjunction`] of your choice.

(You know, as it should be. Haha.)

The return formatting depends on the size of the set:

```ignore,text
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

#![deny(unsafe_code)]

#![warn(
	clippy::filetype_is_file,
	clippy::integer_division,
	clippy::needless_borrow,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::suboptimal_flops,
	clippy::unneeded_field_pattern,
	macro_use_extern_crate,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
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

/// # Join 3+.
///
/// Sets with three or more items follow the general pattern of:
/// `"first" + [", mid"] + ", CONJUNCTION " + "last"`
macro_rules! join_split {
	// Arrays and slices can be pre-split, avoiding the need to iterate.
	($first:ident, $mid:ident, $last:ident, $len:ident, $glue:ident) => {{
		let mut v: Vec<u8> = Vec::with_capacity($len);
		unsafe {
			// Write the first.
			let mut dst = write_advance($first.as_ref().as_bytes(), v.as_mut_ptr());

			// Write the middle.
			for s in $mid {
				dst = write_advance(COMMASPACE, dst);
				dst = write_advance(s.as_ref().as_bytes(), dst);
			}

			// Write the glue.
			dst = $glue.write_to(dst);

			// Write the last.
			dst = write_advance($last.as_ref().as_bytes(), dst);

			// Set the length.
			let offset = dst.offset_from(v.as_ptr()) as usize;
			assert_eq!(offset, $len, "The length of the joined string changed?! Invalid AsRef<str> impl?");
			v.set_len(offset);

			// Build and return the string.
			String::from_utf8_unchecked(v)
		}
	}};

	// The BTree collections have to iterate, but we can still do some things
	// to help them out.
	($iter:expr, $last:ident, $len:ident, $glue:ident) => {{
		let mut v: Vec<u8> = Vec::with_capacity($len);
		let mut iter = $iter;
		unsafe {
			// Write the first.
			let mut dst = write_advance(
				iter.next().unwrap().as_ref().as_bytes(),
				v.as_mut_ptr()
			);

			// Write the middle. ("last" is the count minus one, so we need to
			// subtract one more to account for the (first) entry we already
			// took.)
			for s in iter.by_ref().take($last - 1) {
				dst = write_advance(COMMASPACE, dst);
				dst = write_advance(s.as_ref().as_bytes(), dst);
			}

			// Write the glue.
			dst = $glue.write_to(dst);

			// Write the last.
			dst = write_advance(iter.next().unwrap().as_ref().as_bytes(), dst);

			// Set the length.
			let offset = dst.offset_from(v.as_ptr()) as usize;
			assert_eq!(offset, $len, "The length of the joined string changed?! Invalid AsRef<str> impl?");
			v.set_len(offset);

			// Build and return the string.
			String::from_utf8_unchecked(v)
		}
	}};
}



#[derive(Debug, Copy, Clone, Default, Eq, Hash, PartialEq)]
/// # Conjunction.
///
/// This is the glue used to bind the last entry in an [`oxford_join`]ed set.
///
/// If you're doing something weird and the preset entries aren't currint it
/// for you, you can use [`Conjunction::Other`], which wraps an `&str`. This
/// value should just be the word/symbol; surrounding whitespace and
/// punctuation are added during the join as needed.
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
	#[allow(unsafe_code)]
	/// # Write To.
	///
	/// This writes the conjunction with a leading comma-space and trailing
	/// space to the raw pointer, then returns that pointer advanced the
	/// appropriate length.
	///
	/// ## Safety
	///
	/// This is unsafe because it deals with pointers, but space has been
	/// pre-allocated for the length being written, so it's fine. ;)
	unsafe fn write_to(&self, v: *mut u8) -> *mut u8 {
		match self {
			Self::Ampersand => write_advance(b", & ", v),
			Self::And => write_advance(b", and ", v),
			Self::AndOr => write_advance(b", and/or ", v),
			Self::Nor => write_advance(b", nor ", v),
			Self::Or => write_advance(b", or ", v),
			Self::Other(s) => {
				let mut v = write_advance(COMMASPACE, v);
				v = write_advance(s.as_bytes(), v);
				core::ptr::write(v, b' ');
				v.add(1)
			},
			Self::Plus => write_advance(b", + ", v),
		}
	}

	#[allow(unsafe_code)]
	#[allow(clippy::cast_sign_loss)] // We're subtracting zero.
	#[inline]
	/// # Join Two.
	///
	/// Build a string that is "A <CONJUNCTION> B".
	fn join_two(&self, a: &str, b: &str) -> String {
		let len = a.len() + b.len() + 2 + self.len();
		let mut v: Vec<u8> = Vec::with_capacity(len);
		unsafe {
			let mut dst = write_advance(a.as_bytes(), v.as_mut_ptr());

			dst = match self {
				Self::Ampersand => write_advance(b" & ", dst),
				Self::And => write_advance(b" and ", dst),
				Self::AndOr => write_advance(b" and/or ", dst),
				Self::Nor => write_advance(b" nor ", dst),
				Self::Or => write_advance(b" or ", dst),
				Self::Other(s) => {
					core::ptr::write(dst, b' ');
					dst = write_advance(s.as_bytes(), dst.add(1));
					core::ptr::write(dst, b' ');
					dst.add(1)
				},
				Self::Plus => write_advance(b" + ", dst),
			};

			dst = write_advance(b.as_bytes(), dst);

			let offset = dst.offset_from(v.as_ptr()) as usize;
			debug_assert_eq!(offset, len, "BUG: The length of the joined string changed?!");
			v.set_len(offset);

			String::from_utf8_unchecked(v)
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
	#[allow(unsafe_code)]
	/// # Oxford Join.
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		match self.len() {
			0 => Cow::Borrowed(""),
			1 => Cow::Borrowed(self[0].as_ref()),
			2 => Cow::Owned(glue.join_two(self[0].as_ref(), self[1].as_ref())),
			n => {
				let last = n - 1;
				let len = glue.len() + 1 + (last << 1) + self.iter().map(|x| x.as_ref().len()).sum::<usize>();

				// We can't pattern this split like we can with arrays, but
				// this will do.
				let first = &self[0];
				let mid = &self[1..last];
				let last = &self[last];

				let out = join_split!(first, mid, last, len, glue);
				Cow::Owned(out)
			},
		}
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
	/// This is a special case; it will always read "first <CONJUNCTION> last".
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		Cow::Owned(glue.join_two(self[0].as_ref(), self[1].as_ref()))
	}
}

/// # Join Arrays (3+).
macro_rules! join_arrays {
	($($num:literal $pad:literal $last:literal),+ $(,)?) => ($(
		impl<T> OxfordJoin for [T; $num] where T: AsRef<str> {
			#[allow(unsafe_code)]
			/// # Oxford Join.
			fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
				let len = glue.len() + $pad + self.iter().map(|x| x.as_ref().len()).sum::<usize>();
				let [first, mid @ .., last] = self;
				let out = join_split!(first, mid, last, len, glue);
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

impl<K, T> OxfordJoin for BTreeMap<K, T> where T: AsRef<str> {
	#[allow(unsafe_code)]
	/// # Oxford Join.
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		match self.len() {
			0 => Cow::Borrowed(""),
			1 => {
				let a = self.values().next().unwrap();
				Cow::Borrowed(a.as_ref())
			},
			2 => {
				let mut iter = self.values();
				let a = iter.next().unwrap();
				let b = iter.next().unwrap();
				Cow::Owned(glue.join_two(a.as_ref(), b.as_ref()))
			},
			n => {
				let last = n - 1;
				let len = glue.len() + 1 + (last << 1) + self.values().map(|x| x.as_ref().len()).sum::<usize>();
				let out = join_split!(self.values(), last, len, glue);
				Cow::Owned(out)
			},
		}
	}
}

impl<T> OxfordJoin for BTreeSet<T> where T: AsRef<str> {
	#[allow(unsafe_code)]
	/// # Oxford Join.
	fn oxford_join(&self, glue: Conjunction) -> Cow<str> {
		match self.len() {
			0 => Cow::Borrowed(""),
			1 => {
				let a = self.iter().next().unwrap();
				Cow::Borrowed(a.as_ref())
			},
			2 => {
				let mut iter = self.iter();
				let a = iter.next().unwrap();
				let b = iter.next().unwrap();
				Cow::Owned(glue.join_two(a.as_ref(), b.as_ref()))
			},
			n => {
				let last = n - 1;
				let len = glue.len() + 1 + (last << 1) + self.iter().map(|x| x.as_ref().len()).sum::<usize>();
				let out = join_split!(self.iter(), last, len, glue);
				Cow::Owned(out)
			},
		}
	}
}



#[allow(unsafe_code)]
#[inline]
/// # Write/Advance.
const unsafe fn write_advance(src: &[u8], dst: *mut u8) -> *mut u8 {
	core::ptr::copy_nonoverlapping(src.as_ptr(), dst, src.len());
	dst.add(src.len())
}



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
	#[allow(unsafe_code)]
	fn conjunction_append_to() {
		for c in CTEST {
			// What we expect.
			let s = [", ", c.as_str(), " "].concat();

			// Check the pointer version too.
			let mut v = Vec::with_capacity(s.len());
			unsafe {
				let ptr = c.write_to(v.as_mut_ptr());

				// The pointer should have the same length as the original.
				assert_eq!(
					ptr.offset_from(v.as_ptr()) as usize,
					s.len(),
					"Wrong ptr length: {}.", c.as_str()
				);

				v.set_len(s.len());
			}

			// It should equal the manual version.
			assert_eq!(v, s.as_bytes());
		}
	}

	#[test]
	fn join_two_three() {
		for c in CTEST {
			let tmp = ["one ", c.as_str(), " two"].concat();
			assert_eq!(c.join_two("one", "two"), tmp);

			// We test collections of various sizes elsewhere, but only with
			// one conjunction. This makes sure each individual conjunction
			// gets written correctly.
			let tmp = ["one, two, ", c.as_str(), " three"].concat();
			assert_eq!(["one", "two", "three"].oxford_join(c), tmp);
		}

		assert!(Conjunction::Other("").is_empty());
	}
}
