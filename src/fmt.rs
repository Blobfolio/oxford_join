/*!
# Oxford Join: Format (Display) Wrappers.
*/

use crate::{
	Conjunction,
	OxfordJoin,
};
use alloc::borrow::Cow;
use core::fmt;



/// # [`Display`](fmt::Display)-Based Oxford Join Wrapper.
///
/// This struct offers a [`Display`](fmt::Display)-based alternative to the
/// main [`OxfordJoin`] trait.
///
/// In situations where joined output is only needed for the likes of
/// `format!`, `println!`, etc., this can help avoid unnecessary allocation.
///
/// Note that unlike the main trait, this does not require `T: AsRef<str>`. It
/// does, however, require a slice-based set.
///
/// ## Examples
///
/// ```
/// use oxford_join::{Conjunction, OxfordJoinFmt};
///
/// let set = ["Apples"];
/// assert_eq!(
///     OxfordJoinFmt::new(&set, Conjunction::And).to_string(),
///     "Apples",
/// );
///
/// let set = ["Apples", "Oranges"];
/// assert_eq!(
///     OxfordJoinFmt::new(&set, Conjunction::Or).to_string(),
///     "Apples or Oranges",
/// );
///
/// let set = ["Apples", "Oranges", "Bananas"];
/// assert_eq!(
///     OxfordJoinFmt::new(&set, Conjunction::AndOr).to_string(),
///     "Apples, Oranges, and/or Bananas",
/// );
/// ```
pub struct OxfordJoinFmt<'a, T: fmt::Display> {
	/// # The Set.
	inner: &'a [T],

	/// # The Glue.
	glue: Conjunction<'a>,
}

impl<'a, T: fmt::Display> fmt::Display for OxfordJoinFmt<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use core::cmp::Ordering;

		// Split off the last part, or quit because the set is empty.
		if let Some((last, rest)) = self.inner.split_last() {
			// If last is all we have, it's all we print!
			match rest.len().cmp(&1) {
				// Last is all there is.
				Ordering::Less => write!(f, "{last}"),

				// Just one thing.
				Ordering::Equal => write!(f, "{} {} {last}", rest[0], self.glue),

				// Many things.
				Ordering::Greater => {
					for v in rest { write!(f, "{v}, ")?; }
					write!(f, "{} {last}", self.glue)
				},
			}
		}
		else { Ok(()) }
	}
}

impl<'a, T: fmt::Display> OxfordJoinFmt<'a, T> {
	#[inline]
	/// # Oxford Join.
	///
	/// Return a wrapper for the set with the desired conjunction.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::{Conjunction, OxfordJoinFmt};
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(
	///     OxfordJoinFmt::new(set.as_slice(), Conjunction::Ampersand).to_string(),
	///     "Apples & Oranges",
	/// );
	/// ```
	pub const fn new(set: &'a [T], glue: Conjunction<'a>) -> Self {
		Self { inner: set, glue }
	}

	#[inline]
	/// # Oxford Join (and).
	///
	/// This is equivalent to passing [`Conjunction::And`] to
	/// [`OxfordJoinFmt::new`].
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::OxfordJoinFmt;
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(
	///     OxfordJoinFmt::and(set.as_slice()).to_string(),
	///     "Apples and Oranges",
	/// );
	/// ```
	pub const fn and(set: &'a [T]) -> Self { Self::new(set, Conjunction::And) }

	#[inline]
	/// # Oxford Join (and/or).
	///
	/// This is equivalent to passing [`Conjunction::AndOr`] to
	/// [`OxfordJoinFmt::new`].
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::OxfordJoinFmt;
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(
	///     OxfordJoinFmt::and_or(set.as_slice()).to_string(),
	///     "Apples and/or Oranges",
	/// );
	/// ```
	pub const fn and_or(set: &'a [T]) -> Self { Self::new(set, Conjunction::AndOr) }

	#[inline]
	/// # Oxford Join (nor).
	///
	/// This is equivalent to passing [`Conjunction::Nor`] to
	/// [`OxfordJoinFmt::new`].
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::OxfordJoinFmt;
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(
	///     OxfordJoinFmt::nor(set.as_slice()).to_string(),
	///     "Apples nor Oranges",
	/// );
	/// ```
	pub const fn nor(set: &'a [T]) -> Self { Self::new(set, Conjunction::Nor) }

	#[inline]
	/// # Oxford Join (or).
	///
	/// This is equivalent to passing [`Conjunction::Or`] to
	/// [`OxfordJoinFmt::new`].
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::OxfordJoinFmt;
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(
	///     OxfordJoinFmt::or(set.as_slice()).to_string(),
	///     "Apples or Oranges",
	/// );
	/// ```
	pub const fn or(set: &'a [T]) -> Self { Self::new(set, Conjunction::Or) }
}

impl<'a, T: AsRef<str> + fmt::Display> OxfordJoinFmt<'a, T> {
	#[must_use]
	/// # Join the Regular Way.
	///
	/// Join and return a string with commas in all the right places, same as
	/// calling [`OxfordJoin::oxford_join`] directly on the set, but without
	/// the need to re-specify the conjunction.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::OxfordJoinFmt;
	///
	/// let set = ["Apples", "Oranges"];
	/// assert_eq!(
	///     OxfordJoinFmt::or(set.as_slice()).join(),
	///     "Apples or Oranges",
	/// );
	/// ```
	pub fn join(&'a self) -> Cow<'a, str> {
		self.inner.oxford_join(self.glue)
	}
}
