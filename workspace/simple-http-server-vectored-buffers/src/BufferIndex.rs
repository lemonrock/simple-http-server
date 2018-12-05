// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A buffer index.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferIndex(usize);

impl BufferIndex
{
	/// Minimum index, inclusive.
	pub const Minimum: Self = BufferIndex(0);

	/// Maximum index, inclusive.
	pub const Maximum: Self = BufferIndex(15);

	pub(crate) const MaximumSize: usize = Self::Maximum.0 + 1;

	/// Converts an ever-increasing index to a known `BufferIndex` within (inclusively) `Self::Minimum` and `Self::Maximum`.
	#[inline(always)]
	pub const fn from_only_ever_increasing_index(only_ever_increasing_index: usize) -> Self
	{
		BufferIndex(only_ever_increasing_index % Self::MaximumSize)
	}

	/// Converts to a vector buffer offset, with an inclusive offset of 0 (zero).
	#[inline(always)]
	pub const fn to_vector_buffer_offset(self) -> Self
	{
		self.to_vector_buffer_offset_with_offset(0)
	}

	/// Converts to a vector buffer offset.
	#[inline(always)]
	pub const fn to_vector_buffer_offset_with_offset(self, offset: InclusiveFromOffset) -> Self
	{
		VectoredBufferOffset::new(self, offset)
	}

	/// Always get the next index, wrapping round as appropriate.
	#[inline(always)]
	pub fn next(self) -> Self
	{
		if unlikely!(self == Self::Maximum)
		{
			BufferIndex(0)
		}
		else
		{
			self.next_unchecked()
		}
	}

	#[inline(always)]
	pub(crate) fn next_unchecked(self) -> Self
	{
		BufferIndex(self.0 + 1)
	}

	#[inline(always)]
	pub(crate) fn increment_unchecked(&mut self)
	{
		self.0 += 1
	}

	#[inline(always)]
	pub(crate) fn decrement_unchecked(&mut self)
	{
		self.0 -= 1
	}
}
