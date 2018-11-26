// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An iterator over the Rust immutable slices that make up a vectored buffer slice.
///
/// This is not a 'true' iterator, as it Rust's Iterator trait does not link the lifetime in `fn next()` with `Self::Item`.
///
/// See <https://stackoverflow.com/questions/30422177/how-do-i-write-an-iterator-that-returns-references-to-itself/30422716#30422716> and <https://www.reddit.com/r/rust/comments/6ffrbs/implementing_a_safe_mutable_iterator/>.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutableVectoredBufferSliceIterator<'vectored_buffer>
{
	parent: &'vectored_buffer mut MutableVectoredBufferSlice<'vectored_buffer>,
	is_at: Option<(BufferIndex, InclusiveFromOffset)>,
}

impl<'vectored_buffer> MutableVectoredBufferSliceIterator<'vectored_buffer>
{
	/// A method very similar to an Iterator's `next()` method but with lifetime annotation.
	#[inline(always)]
	pub fn next(&'vectored_buffer mut self) -> Option<&'vectored_buffer mut [u8]>
	{
		let (slice, next_is_at) = match self.is_at
		{
			None => return None,
			Some(is_at) => self.parent.next_like(is_at),
		};
		self.is_at = next_is_at;
		Some(slice)
	}

	/// A method very similar to an Iterator's `size_hint()` method.
	#[inline(always)]
	pub fn size_hint(&self) -> (usize, Option<usize>)
	{
		let bound = self.len();
		(bound, Some(bound))
	}

	/// A method very similar to an ExactSizeIterator's `len()` method.
	#[inline(always)]
	pub fn len(&self) -> usize
	{
		match self.is_at.as_ref()
		{
			None => 0,
			Some(&(is_at_buffer_index, _is_at_inclusive_from_offset)) => self.parent.ends_at_buffer_index() - is_at_buffer_index + 1,
		}
	}

	/// A method very similar to an ExactSizeIterator's `is_empty()` method.
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		self.len() == 0
	}
}
