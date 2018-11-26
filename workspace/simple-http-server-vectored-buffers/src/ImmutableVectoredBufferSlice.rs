// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An immutable vectored buffer slice.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImmutableVectoredBufferSlice<'vectored_buffer>
{
	vectored_buffer: &'vectored_buffer VectoredBuffer,
	is_at: (BufferIndex, InclusiveFromOffset),
	ends_at: (BufferIndex, ExclusiveToOffset),
}

impl<'vectored_buffer> ImmutableVectoredBufferSlice<'vectored_buffer>
{
	#[inline(always)]
	fn next_like(&'vectored_buffer self, (is_at_buffer_index, is_at_inclusive_from_offset): (BufferIndex, InclusiveFromOffset)) -> (&'vectored_buffer [u8], Option<(BufferIndex, InclusiveFromOffset)>)
	{
		let ends_at_buffer_index = self.ends_at_buffer_index();
		let ends_at_exclusive_to_offset = self.ends_at_exclusive_to_offset();

		let immmutable_buffer = self.vectored_buffer.get_immutable_buffer(is_at_buffer_index);

		if is_at_buffer_index == ends_at_buffer_index
		{
			let slice = &immmutable_buffer[is_at_inclusive_from_offset ..ends_at_exclusive_to_offset];
			(slice, None)
		}
		else
		{
			let slice = &immmutable_buffer[is_at_inclusive_from_offset .. ];
			(slice, Some((is_at_buffer_index + 1, 0)))
		}
	}

	/// Use the Rust immutable slices that make up this slice.
	#[inline(always)]
	pub fn use_slices<Failure>(&self, slice_user: impl Fn(&[u8]) -> Result<(), Failure>) -> Result<(), Failure>
	{
		let (mut is_at_buffer_index, is_at_inclusive_from_offset) = self.is_at;

		if likely!(is_at_buffer_index == self.ends_at_buffer_index())
		{
			slice_user(&self.vectored_buffer.get_immutable_buffer(is_at_buffer_index)[is_at_inclusive_from_offset .. self.ends_at_exclusive_to_offset()])?;
		}
		else
		{
			slice_user(&self.vectored_buffer.get_immutable_buffer(is_at_buffer_index)[is_at_inclusive_from_offset .. ])?;
			is_at_buffer_index += 1;

			while is_at_buffer_index < self.ends_at_buffer_index()
			{
				slice_user(self.vectored_buffer.get_immutable_buffer(is_at_buffer_index))?;
				is_at_buffer_index += 1;
			}

			slice_user(&self.vectored_buffer.get_immutable_buffer(is_at_buffer_index)[ .. self.ends_at_exclusive_to_offset()])?;
		}
		Ok(())
	}

	#[inline(always)]
	fn ends_at_buffer_index(&self) -> usize
	{
		self.ends_at.0
	}

	#[inline(always)]
	fn ends_at_exclusive_to_offset(&self) -> usize
	{
		self.ends_at.1
	}
}
