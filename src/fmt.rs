/*!
# Oxford Join: Format (Display) Wrappers.
*/

use crate::Conjunction;
use core::{
	cell::Cell,
	fmt,
};



/// # [`Display`](fmt::Display)-Based Join Wrapper.
///
/// This wrapper provides a symmetrical — _non-Oxford!_ — alternative to
/// [`OxfordJoinFmt`].
///
/// It works just like `Vec::join` or any interspersing iterator; the chosen
/// separator is inserted between _every_ pair of values, plain and simple.
///
/// In situations where joined output is only needed for the likes of
/// `format!`, `println!`, etc., this can help avoid unnecessary intermediary
/// allocation.
///
/// ## Examples
///
/// ```
/// use oxford_join::JoinFmt;
///
/// let set = ["one", "two", "three"];
///
/// // Joining the slice directly gets the job done, but creates an extra
/// // string in the process.
/// assert_eq!(
///     format!("numbers: {}", set.join(", ")),
///     "numbers: one, two, three",
/// );
///
/// // JoinFmt implements display directly, avoiding that overhead.
/// assert_eq!(
///     format!("numbers: {}", JoinFmt::new(set.iter(), ", ")),
///     "numbers: one, two, three",
/// );
/// ```
///
/// ## Errors
///
/// [`Display::fmt`](fmt::Display::fmt) necessarily consumes the backing iterator
/// when invoked so can only be called **_once_**; any attempted reuse will trigger
/// an error and/or panic.
///
/// ```should_panic
/// use oxford_join::JoinFmt;
///
/// let set = ["one", "two", "three"];
///
/// // Saving it to a variable won't save you; double-use will panic!
/// let wrapped = JoinFmt::new(set.iter(), " + ");
/// let nope = format!("{wrapped} + {wrapped}");
/// ```
pub struct JoinFmt<'a, I: Iterator>
where <I as Iterator>::Item: fmt::Display {
	/// # Wrapped Iterator.
	iter: Cell<Option<I>>,

	/// # The Glue.
	glue: &'a str,
}

impl<'a, I: Iterator> JoinFmt<'a, I>
where <I as Iterator>::Item: fmt::Display {
	#[inline]
	/// # Join.
	///
	/// Return a wrapper around the iterator and desired separator (glue), if
	/// any.
	///
	/// ## Examples
	///
	/// ```
	/// use oxford_join::JoinFmt;
	///
	/// let set = ["Apples", "Oranges", "Bananas"];
	/// assert_eq!(
	///     format!("{}", JoinFmt::new(set.iter(), " & ")),
	///     "Apples & Oranges & Bananas",
	/// );
	/// ```
	pub const fn new(iter: I, glue: &'a str) -> Self {
		Self {
			iter: Cell::new(Some(iter)),
			glue,
		}
	}
}

impl<I: Iterator> fmt::Display for JoinFmt<'_, I>
where <I as Iterator>::Item: fmt::Display {
	#[inline]
	#[track_caller]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// The iterator is consumed during invocation so we can only do this
		// once!
		let mut iter = self.iter.take().ok_or(fmt::Error)?;

		// If the glue is empty, just run through everything in one go.
		if self.glue.is_empty() {
			for v in iter { <I::Item as fmt::Display>::fmt(&v, f)?; }
		}
		// Otherwise start with the first first, then loop through the rest,
		// adding the glue at the start of each pass.
		else if let Some(v) = iter.next() {
			<I::Item as fmt::Display>::fmt(&v, f)?;

			// Finish it!
			for v in iter {
				f.write_str(self.glue)?;
				<I::Item as fmt::Display>::fmt(&v, f)?;
			}
		}

		Ok(())
	}
}



/// # [`Display`](fmt::Display)-Based Oxford Join Wrapper.
///
/// This struct offers a [`Display`](fmt::Display)-based alternative to the
/// main [`OxfordJoin`](crate::OxfordJoin) trait.
///
/// In situations where joined output is only needed for the likes of
/// `format!`, `println!`, etc., this can help avoid unnecessary intermediary
/// allocation.
///
/// Note that unlike the main trait, this does not require `T: AsRef<str>`. It
/// does, however, require a slice-based set to start with.
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



#[cfg(test)]
mod test {
	use super::*;
	use alloc::format;

	#[test]
	fn t_join() {
		// With just one item, the glue is irrelevant.
		assert_eq!(
			format!("{}", JoinFmt::new(core::iter::once("hi"), "-")),
			"hi",
		);

		// Now the glue matters.
		assert_eq!(
			format!("{}", JoinFmt::new(["hi", "ho"].iter(), "-")),
			"hi-ho",
		);

		// Empty-glue cases are specialized, so let's quickly verify
		// concatenation works as expected.
		assert_eq!(
			format!("{}", JoinFmt::new(["hi", "ho"].iter(), "")),
			"hiho",
		);
	}
}
