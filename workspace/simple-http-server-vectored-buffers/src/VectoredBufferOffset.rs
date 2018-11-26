// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An offset into a vectored buffer.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VectoredBufferOffset
{
	/// Buffer index, 0 to 15 inclusive.
	///
	/// Zero-based.
	///
	/// Must never be the same as `RingBufferMaximumLength` (or greater).
	pub buffer_index: usize,

	/// Offset (in bytes) within the buffer at `buffer_index` from 0 inclusive.
	///
	/// Zero-based.
	///
	/// Must never be the same as the buffer length (or greater) of the buffer at `buffer_index`.
	pub offset: usize,
}

impl VectoredBufferOffset
{
	/// Convenience constructor.
	#[inline(always)]
	pub const fn new(buffer_index: usize, offset: usize) -> Self
	{
		Self
		{
			buffer_index,
			offset,
		}
	}

	/// Increments to next buffer, zero offset.
	#[inline(always)]
	pub fn next(&self) -> Self
	{
		Self
		{
			buffer_index: self.buffer_index + 1,
			offset: 0,
		}
	}
}
