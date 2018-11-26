// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An iterator over the Rust immutable slices that make up a vectored buffer slice.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImmutableVectoredBufferSliceIterator<'vectored_buffer>
{
	parent: &'vectored_buffer ImmutableVectoredBufferSlice<'vectored_buffer>,
	is_at: Option<(BufferIndex, InclusiveFromOffset)>,
}

impl<'vectored_buffer> Iterator for ImmutableVectoredBufferSliceIterator<'vectored_buffer>
{
	type Item = &'vectored_buffer [u8];

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let (slice, next_is_at) = match self.is_at
		{
			None => return None,
			Some(is_at) => self.parent.next_like(is_at),
		};
		self.is_at = next_is_at;
		Some(slice)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		let bound = self.len();
		(bound, Some(bound))
	}
}

impl<'vectored_buffer> ExactSizeIterator for ImmutableVectoredBufferSliceIterator<'vectored_buffer>
{
	#[inline(always)]
	fn len(&self) -> usize
	{
		match self.is_at.as_ref()
		{
			None => 0,
			Some(&(is_at_buffer_index, _is_at_inclusive_from_offset)) => self.parent.ends_at_buffer_index() - is_at_buffer_index + 1,
		}
	}
}
