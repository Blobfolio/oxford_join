/*!
# Oxford Join

[![docs.rs](https://img.shields.io/docsrs/oxford_join.svg?style=flat-square&label=docs.rs)](https://docs.rs/oxford_join/)
[![changelog](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/oxford_join/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=crates.io)](https://crates.io/crates/oxford_join)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/oxford_join/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/oxford_join/actions)<br>
[![deps.rs](https://deps.rs/crate/oxford_join/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/oxford_join/)
[![no_std](https://img.shields.io/badge/env-%23![no_std]-00abc0?style=flat-square)](https://docs.rust-embedded.org/book/intro/no-std.html)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/oxford_join/issues)

This crate provides two joining macros for `format!`, `println!`, etc.,
formatting contexts:
* [`join`] combines the values of an iterator with the specified glue;
* [`oxford_join`] combines the values of an iterator with [Oxford commas](https://en.wikipedia.org/wiki/Serial_comma) and a conjunction;

```
# extern crate alloc;
use oxford_join::{join, oxford_join};

// Straight joins.
assert_eq!(
    format!("{} = 6", join!(" + ", 1_u8..=3_u8)),
    "1 + 2 + 3 = 6",
);

// The good stuff!
assert_eq!(
    oxford_join!("or", ["apples", "bananas"].iter()).to_string(),
    "apples or bananas",
);
assert_eq!(
    oxford_join!("or", ["apples", "bananas", "carrots"].iter()).to_string(),
    "apples, bananas, or carrots",
);
```

Formatting [parameters](https://doc.rust-lang.org/std/fmt/index.html#formatting-parameters) and [traits](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits)
are supported by both macros and passed through to each individual value.

```
# extern crate alloc;
# use oxford_join::oxford_join;
// Alternate, padding, HEX, oh my!
assert_eq!(
    format!(
        "Checksums: {:#010X}",
        oxford_join!(@UpperHex [30_000, 166, u32::MAX].iter()),
    ),
    "Checksums: 0x00007530, 0x000000A6, and 0xFFFFFFFF",
);
```

See the documentation for [`join`] and [`oxford_join`] for complete usage details.
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
	clippy::implicit_clone,
	clippy::lossy_float_literal,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::needless_raw_strings,
	clippy::panic_in_result_fn,
	clippy::pub_without_shorthand,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::semicolon_inside_block,
	clippy::str_to_string,
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

use core::fmt;

#[cfg(test)]
use brunch as _;



/// # Helper: Wrapper Structs.
///
/// These work just like [`FromFn`](fmt::FromFn), but map to individual
/// formatting traits.
macro_rules! wrappers {
	( $d:tt $( $struct:ident $trait:ident, )+ ) => (
		$(
			#[doc(hidden)]
			#[doc = concat!("# Format Type ([`", stringify!($trait), "`](fmt::", stringify!($trait), "))")]
			pub struct $struct<F>(F);

			#[doc(hidden)]
			impl<F> $struct<F>
			where F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result {
				#[inline]
				/// # New.
				pub const fn new(f: F) -> Self { Self(f) }
			}

			#[doc(hidden)]
			impl<F> fmt::$trait for $struct<F>
			where F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result {
				#[inline]
				fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
					(self.0)(f)
				}
			}
		)+

		#[macro_export]
		#[doc(hidden)]
		/// # Helper: Wrap Wrapper.
		///
		/// This macro is used to map a trait to its corresponding struct
		/// instantiation method.
		macro_rules! wrap {
			$(
				(@$trait $cb:expr) => ( $crate::$struct::new($cb) );
			)+

			// Catchall error message.
			( $d( $tt:tt )* ) => (
				::core::compile_error!(
					::core::concat!(
						"Supported traits are ",
						$( ::core::concat!("@", ::core::stringify!($trait), " ") ),+
					)
				)
			);
		}
	);
}
wrappers! {
	$ // TODO: remove when "meta-variable expressions" become stable.
	FmtBinary   Binary,
	FmtDebug    Debug,
	FmtDisplay  Display,
	FmtLowerExp LowerExp,
	FmtLowerHex LowerHex,
	FmtOctal    Octal,
	FmtUpperExp UpperExp,
	FmtUpperHex UpperHex,
}



#[macro_export]
/// # Oxford Join Formatter.
///
/// This macro joins the values of an iterator with [Oxford commas](https://en.wikipedia.org/wiki/Serial_comma)
/// and a conjunction for `format!`, `println!`, etc., formatting contexts,
/// without making any intermediary allocations.
///
/// ## Examples
///
/// ```
/// # extern crate alloc;
/// use oxford_join::oxford_join;
///
/// assert_eq!(
///     format!(
///         "This macro is {}!",
///         oxford_join!(["simple", "convenient", "niche"].iter()),
///     ),
///     "This macro is simple, convenient, and niche!",
/// );
///
/// // The default conjunction is "and". An alternative can be specified as
/// // the first argument.
/// assert_eq!(
///     format!("Pick {}.", oxford_join!("or", 1..=5_u8)),
///     "Pick 1, 2, 3, 4, or 5.",
/// );
///
/// // The output style depends on how many values there are.
/// let list = ["apples", "bananas", "carrots"];
/// assert_eq!(
///     oxford_join!(list.iter().take(0)).to_string(),
///     "",                             // Zero.
/// );
/// assert_eq!(
///     oxford_join!(list.iter().take(1)).to_string(),
///     "apples",                       // One.
/// );
/// assert_eq!(
///     oxford_join!(list.iter().take(2)).to_string(),
///     "apples and bananas",           // Two.
/// );
/// assert_eq!(
///     oxford_join!(list.iter().take(3)).to_string(),
///     "apples, bananas, and carrots", // Three+.
/// );
/// ```
///
/// [Formatting parameters](https://doc.rust-lang.org/std/fmt/index.html#formatting-parameters)
/// like alignment, padding, etc., are automatically passed through to each
/// individual value.
///
///```
/// # extern crate alloc;
/// # use core::f64::consts::{PI, GOLDEN_RATIO};
/// # use oxford_join::oxford_join;
/// // Alignment.
/// assert_eq!(
///     format!("{:>3}", oxford_join!("OR", ["no", "yes"].iter())),
///     " no OR yes",
/// );
///
/// // Padding.
/// assert_eq!(
///     format!("{:03}", oxford_join!(1..=3_u8)),
///     "001, 002, and 003",
/// );
///
/// // Precision.
/// assert_eq!(
///     format!("{:03.2}", oxford_join!([PI, GOLDEN_RATIO].iter())),
///     "3.14 and 1.62",
/// );
///```
///
/// Alternative [formatting traits](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits)
/// (except [`Pointer`](fmt::Pointer))
/// are also supported, but require the matching `@Trait` be manually added as the
/// first [`oxford_join`] argument.
///
/// ```
/// # extern crate alloc;
/// # use oxford_join::oxford_join;
/// // Slices and arrays require Debug.
/// let slice: &[[u8; 2]] = &[[0, 1], [2, 3], [4, 5]];
/// assert_eq!(
///     format!("{:?}", oxford_join!(@Debug slice.iter())),
///     "[0, 1], [2, 3], and [4, 5]",
/// );
/// #
/// # // Debug can also be useful for quoting.
/// # let list = ["apples", "bananas", "carrots"];
/// # assert_eq!(
/// #     format!("{:?}", oxford_join!(@Debug list.iter().take(0))),
/// #     "",
/// # );
/// # assert_eq!(
/// #     format!("{:?}", oxford_join!(@Debug list.iter().take(1))),
/// #     r#""apples""#,                           // One.
/// # );
/// # assert_eq!(
/// #     format!("{:?}", oxford_join!(@Debug list.iter().take(2))),
/// #     r#""apples" and "bananas""#,             // Two.
/// # );
/// # assert_eq!(
/// #     format!("{:?}", oxford_join!(@Debug list.iter().take(3))),
/// #     r#""apples", "bananas", and "carrots""#, // Three+.
/// # );
///
/// // Integer primitives implement all sorts of different traits!
/// assert_eq!(
///     format!("{:04b}", oxford_join!(@Binary 0..3_u8)),
///     "0000, 0001, and 0010",
/// );
/// assert_eq!(
///     format!("{:#04b}", oxford_join!(@Binary 0..3_u8)),
///     "0b00, 0b01, and 0b10",
/// );
///
/// assert_eq!(
///     format!("{:e}", oxford_join!(@LowerExp [1, 42, 3456].iter())),
///     "1e0, 4.2e1, and 3.456e3",
/// );
/// assert_eq!(
///     format!("{:E}", oxford_join!(@UpperExp [1, 42, 3456].iter())),
///     "1E0, 4.2E1, and 3.456E3",
/// );
///
/// assert_eq!(
///     format!("{:02x}", oxford_join!(@LowerHex 10..15_u8)),
///     "0a, 0b, 0c, 0d, and 0e",
/// );
/// assert_eq!(
///     format!("{:#04x}", oxford_join!(@LowerHex '&', 10..15_u8)),
///     "0x0a, 0x0b, 0x0c, 0x0d, & 0x0e",
/// );
/// assert_eq!(
///     format!("{:02X}", oxford_join!(@UpperHex 10..15_u8)),
///     "0A, 0B, 0C, 0D, and 0E",
/// );
/// assert_eq!(
///     format!("{:#04X}", oxford_join!(@UpperHex 10..15_u8)),
///     "0x0A, 0x0B, 0x0C, 0x0D, and 0x0E",
/// );
///
/// assert_eq!(
///     format!("{:03o}", oxford_join!(@Octal "and/or", 100..103_u8)),
///     "144, 145, and/or 146",
/// );
/// assert_eq!(
///     format!("{:#05o}", oxford_join!(@Octal "and/or", 100..103_u8)),
///     "0o144, 0o145, and/or 0o146",
/// );
/// ```
///
/// [`Display`](fmt::Display) is the default and can be omitted, but if doing
/// something else, the formatting type parameter and [`oxford_join`] `@Trait`
/// must both be present and match, otherwise you'll get a compile-time error.
///
/// ```compile_fail
/// # use oxford_join::oxford_join;
/// // error[E0277]: `oxford_join::FmtDisplay<{closure}>` doesn't implement
/// // `std::fmt::Debug`
/// println!(
///     "{:?}.", oxford_join!(u8::MIN..=u8::MAX),
/// //     ^Debug "type"     ^Needs @Debug
/// );
/// ```
///
/// Typos and unsupported traits will also produce a compile-time error.
///
/// ```compile_fail
/// # use oxford_join::oxford_join;
/// // error: Supported traits are @Binary @Debug @Display @LowerExp @LowerHex
/// // @Octal @UpperExp @UpperHex
/// println!(
///     "{:?}.", oxford_join!(@Degub u8::MIN..=u8::MAX),
/// //                        ^Oops.
/// );
/// ```
///
/// Value types, of course, must actually implement the relevant [formatting trait](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits)
/// too, or again, you'll get a compile-time error.
///
/// ```compile_fail
/// # use core::cmp::Ordering;
/// # use oxford_join::oxford_join;
/// // error[E0277]: `core::cmp::Ordering` doesn't implement `fmt::Display`
/// println!(
///     "Expected {}.",
///     oxford_join!(
///         "or",
///         [Ordering::Less, Ordering::Equal, Ordering::Greater].iter(),
///     ),
/// );
/// ```
macro_rules! oxford_join {
	(@$trait:ident $conjunction:literal, $list:expr $(,)?) => (
		$crate::wrap!(@$trait |f| {
			let mut list = $list;

			// Small lists require different formatting, so we need to pre-load
			// a few entries to know what we're dealing with.
			let Some(next) = list.next() else {
				return Ok(()); // Zero.
			};
			<_ as ::core::fmt::$trait>::fmt(&next, f)?; // The first value.
			let Some(next) = list.next() else {
				return Ok(()); // One.
			};
			let Some(mut dangle) = list.next() else {
				// Two.
				f.write_str(::core::concat!(" ", $conjunction, " "))?;
				return <_ as ::core::fmt::$trait>::fmt(&next, f);
			};

			// With three+ items, all but the last are written with trailing
			// commas. To that end, we need to catch up to the current "last".
			f.write_str(", ")?;
			<_ as ::core::fmt::$trait>::fmt(&next, f)?;
			f.write_str(", ")?;

			// Loop through the remainder, writing the previous value at each
			// step. (The current is moved to `dangle`.)
			for next in list {
				<_ as ::core::fmt::$trait>::fmt(
					&::core::mem::replace(&mut dangle, next),
					f,
				)?;
				f.write_str(", ")?;
			}

			// Write the conjunction and final value!
			f.write_str(::core::concat!($conjunction, " "))?;
			<_ as ::core::fmt::$trait>::fmt(&dangle, f)
		})
	);
	($conjunction:literal, $list:expr $(,)?) => ( $crate::oxford_join!(@Display $conjunction, $list) );

	(@$trait:ident $list:expr $(,)?) => ( $crate::oxford_join!(@$trait "and", $list) );
	($list:expr $(,)?) => ( $crate::oxford_join!(@Display $list) );

	// Catchall error message.
	( $( $tt:tt )* ) => (
		::core::compile_error!(
			::core::concat!(
				"Incorrect macro usage.\n",
				"oxford_join!(IteratorExpr)\n",
				"oxford_join!(ConjunctionLiteral, IteratorExpr)\n",
				"oxford_join!(@TraitIdent IteratorExpr)\n",
				"oxford_join!(@TraitIdent ConjunctionLiteral, IteratorExpr)",
			)
		)
	);
}



#[macro_export]
/// # Join Formatter.
///
/// This macro joins the values of an iterator with arbitrary glue for
/// `format!`, `println!`, etc., formatting contexts, without making any
/// intermediary allocations.
///
/// ## Examples
///
/// ```
/// # extern crate alloc;
/// # use core::iter;
/// use oxford_join::join;
///
/// // With just one thing, the "join" is moot.
/// assert_eq!(
///     join!(", ", iter::once("apples")).to_string(),
///     "apples",
/// );
///
/// // But with many things…
/// assert_eq!(
///     join!(" + ", 0..10_u8).to_string(),
///     "0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9",
/// );
///
/// // The glue is optional.
/// assert_eq!(
///     join!(['a', 'b', 'c'].iter()).to_string(),
///     "abc",
/// );
/// ```
///
/// [Formatting parameters](https://doc.rust-lang.org/std/fmt/index.html#formatting-parameters)
/// like alignment, padding, etc., are automatically passed through to each
/// individual value.
///
///```
/// # extern crate alloc;
/// # use core::f64::consts::{PI, GOLDEN_RATIO};
/// # use oxford_join::join;
/// // Alignment.
/// assert_eq!(
///     format!("{:>3}", join!("\n", ["no", "yes"].iter())),
///     " no\nyes",
/// );
///
/// // Padding.
/// assert_eq!(
///     format!("{:03}", join!(" + ", 1..=3_u8)),
///     "001 + 002 + 003",
/// );
///
/// // Precision.
/// assert_eq!(
///     format!("{:03.2}", join!(" / ", [PI, GOLDEN_RATIO].iter())),
///     "3.14 / 1.62",
/// );
///```
///
/// Alternative [formatting traits](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits)
/// (except [`Pointer`](fmt::Pointer))
/// are also supported, but require the matching `@Trait` be manually added as the
/// first [`join`] argument.
///
/// ```
/// # extern crate alloc;
/// # use oxford_join::join;
/// // Slices and arrays require Debug.
/// let slice: &[[u8; 2]] = &[[0, 1], [2, 3]];
/// assert_eq!(
///     format!("{:?}", join!(@Debug '\n', slice.iter())),
///     "[0, 1]\n[2, 3]",
/// );
///
/// // Glue is still technically optional.
/// assert_eq!(
///     format!("{:?}", join!(@Debug ['a', 'b', 'c'].iter())),
///     "'a''b''c'",
/// );
///
/// // Integer primitives implement all sorts of different traits!
/// assert_eq!(
///     format!("{:04b}", join!(@Binary ", ", 0..3_u8)),
///     "0000, 0001, 0010",
/// );
/// assert_eq!(
///     format!("{:#04b}", join!(@Binary ", ", 0..3_u8)),
///     "0b00, 0b01, 0b10",
/// );
///
/// assert_eq!(
///     format!("{:e}", join!(@LowerExp " ~ ", [1, 42, 3456].iter())),
///     "1e0 ~ 4.2e1 ~ 3.456e3",
/// );
/// assert_eq!(
///     format!("{:E}", join!(@UpperExp " ~ ", [1, 42, 3456].iter())),
///     "1E0 ~ 4.2E1 ~ 3.456E3",
/// );
///
/// assert_eq!(
///     format!("{:02x}", join!(@LowerHex " + ", 10..15_u8)),
///     "0a + 0b + 0c + 0d + 0e",
/// );
/// assert_eq!(
///     format!("{:#04x}", join!(@LowerHex " + ", 10..15_u8)),
///     "0x0a + 0x0b + 0x0c + 0x0d + 0x0e",
/// );
/// assert_eq!(
///     format!("{:02X}", join!(@UpperHex " + ", 10..15_u8)),
///     "0A + 0B + 0C + 0D + 0E",
/// );
/// assert_eq!(
///     format!("{:#04X}", join!(@UpperHex " + ", 10..15_u8)),
///     "0x0A + 0x0B + 0x0C + 0x0D + 0x0E",
/// );
///
/// assert_eq!(
///     format!("{:03o}", join!(@Octal ", ", 100..103_u8)),
///     "144, 145, 146",
/// );
/// assert_eq!(
///     format!("{:#05o}", join!(@Octal ", ", 100..103_u8)),
///     "0o144, 0o145, 0o146",
/// );
/// ```
///
/// [`Display`](fmt::Display) is the default and can be omitted, but if doing
/// something else, the formatting type parameter and [`join`] `@Trait`
/// must both be present and match, otherwise you'll get a compile-time error.
///
/// ```compile_fail
/// # use oxford_join::join;
/// // error[E0277]: `oxford_join::FmtDisplay<{closure}>` doesn't implement
/// // `std::fmt::LowerHex`
/// println!(
///     "{:#x}.",
/// //      ^LowerHex "type"
///     join!(", ", u8::MIN..=u8::MAX),
/// //        ^Missing @LowerHex
/// );
/// ```
///
/// Typos and unsupported traits will also produce a compile-time error.
///
/// ```compile_fail
/// # use oxford_join::join;
/// // error: Supported traits are @Binary @Debug @Display @LowerExp @LowerHex
/// // @Octal @UpperExp @UpperHex
/// println!(
///     "{:#x}.", join!(@hex ", ", u8::MIN..=u8::MAX),
/// //                  ^Oops.
/// );
/// ```
///
/// Value types, of course, must actually implement the relevant [formatting trait](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits)
/// too, or again, you'll get a compile-time error.
///
/// ```compile_fail
/// # use core::cmp::Ordering;
/// # use oxford_join::join;
/// // error[E0277]: `core::cmp::Ordering` doesn't implement `fmt::Display`
/// println!(
///     "Possible orderings: {}.",
///     join!(
///         ", ",
///         [Ordering::Less, Ordering::Equal, Ordering::Greater].iter(),
///     ),
/// );
///```
///
/// ## Advanced Glue
///
/// All of the above examples use string and char literals for glue, but
/// macro expressions producing the same are also supported. They just need to
/// be wrapped in parentheses.
///
/// ```
/// # extern crate alloc;
/// # use core::cmp::Ordering;
/// # use oxford_join::join;
/// /// Helper: ANSI dim.
/// macro_rules! dim {
///     ( $label:expr ) => ( concat!("\x1b[2m", $label, "\x1b[0m") );
/// }
///
/// assert_eq!(
///     join!((dim!(" + ")), 1..=3_u8).to_string(),
/// //        ^           ^ Wrapped to differentiate from literal literals.
///     "1\x1b[2m + \x1b[0m2\x1b[2m + \x1b[0m3",
/// );
///
/// assert_eq!(
///     format!(
///         "{:?}",
///         join!(
///             @Debug
///             (concat!(' ', dim!('<'), ' ')),
///             [Ordering::Less, Ordering::Greater].iter(),
///         ),
///     ),
///     "Less \x1b[2m<\x1b[0m Greater",
/// );
/// ```
macro_rules! join {
	(@$trait:ident ( $glue:expr ), $list:expr $(,)?) => (
		$crate::wrap!(@$trait |f| {
			let mut any = false;
			for next in $list {
				if any { f.write_str(::core::concat!($glue))?; }
				else { any = true; }
				<_ as ::core::fmt::$trait>::fmt(&next, f)?;
			}
			Ok(())
		})
	);
	(@$trait:ident $glue:literal, $list:expr $(,)?) => ( $crate::join!(@$trait ( $glue ), $list) );

	(( $glue:expr ), $list:expr $(,)?) => ( $crate::join!(@Display ( $glue ), $list) );
	($glue:literal, $list:expr $(,)?) =>  ( $crate::join!(@Display ( $glue ), $list) );

	(@$trait:ident $list:expr $(,)?) => (
		$crate::wrap!(@$trait |f| {
			for next in $list {
				<_ as ::core::fmt::$trait>::fmt(&next, f)?;
			}
			Ok(())
		})
	);
	($list:expr $(,)?) => ( $crate::join!(@Display $list) );

	// Catchall error message.
	( $( $tt:tt )* ) => (
		::core::compile_error!(
			::core::concat!(
				"Incorrect macro usage.\n",
				"join!(IteratorExpr)\n",
				"join!(GlueLiteral, IteratorExpr)\n",
				"join!((GlueLitExpr), IteratorExpr)\n",
				"join!(@TraitIdent IteratorExpr)\n",
				"join!(@TraitIdent GlueLiteral, IteratorExpr)\n",
				"join!(@TraitIdent (GlueLitExpr), IteratorExpr)",
			)
		)
	);
}
