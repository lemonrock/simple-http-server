// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


struct Bytes
{
	vectored_buffer: VectoredBuffer,
	current_position: VectoredBufferOffset,
	previous_position: VectoredBufferOffset,

	current_pointer: NonNull<u8>,
	current_slice_end_pointer: NonNull<u8>,
}

impl Bytes
{
	// TODO: Adding or removing a buffer affects increment() and previous_position() and if_has_more_return_current_value_and_increment()

	pub fn retire_oldest_buffer(&mut self) -> bool;

	pub fn recycle_oldest_buffer(&mut self) -> Option<BufferIndex>;

	#[inline(always)]
	pub(crate) fn new(vectored_buffer: VectoredBuffer) -> Self
	{
		let mut this = Self
		{
			vectored_buffer,
			current_position: VectoredBufferOffset::new(BufferIndex::Minimum, 0),
			previous_position: VectoredBufferOffset::new(BufferIndex::Minimum, 0),

			current_pointer: unsafe { uninitialized() },
			current_slice_end_pointer: unsafe { uninitialized() },
		};

		this.set();
		this
	}

	#[inline(always)]
	pub(crate) fn reset(&mut self, reset_to_position: VectoredBufferOffset)
	{
		self.current_position = reset_to_position;

		if likely!(reset_to_position.offset != 0)
		{
			self.previous_position = VectoredBufferOffset::new(reset_to_position.buffer_index, reset_to_position.offset - 1)
		}
		else if unlikely!(reset_to_position.buffer_index == 0)
		{
			self.previous_position = reset_to_position;
		}
		else
		{
			let offset = self.vectored_buffer.get_immutable_buffer(self.current_position.buffer_index).len() - 1;

			self.previous_position = VectoredBufferOffset::new(reset_to_position.buffer_index.decrement_unchecked(), offset)
		}

		self.set()
	}

	#[inline(always)]
	pub(crate) fn set(&mut self)
	{
		let current_slice = self.get_current_slice();
		let current_pointer = unsafe { NonNull::new_unchecked(current_slice.as_ptr()) };

		self.current_pointer = current_pointer;
		self.current_slice_end_pointer = unsafe { current_pointer.add(current_slice.len()) };
	}

	#[inline(always)]
	pub(crate) fn previous_position(&self) -> VectoredBufferOffset
	{
		self.previous_position
	}

	#[inline(always)]
	pub(crate) fn current_position(&self) -> VectoredBufferOffset
	{
		self.current_position
	}

	#[inline(always)]
	pub(crate) fn is_slice<ReentryPoint, InvalidReason, R>(&mut self, compare_to: &[u8], reentry_point: ReentryPoint, invalid_reason: InvalidReason, result: R) -> Result<R, Status<ReentryPoint, InvalidReason>>
	{
		trait NonNullExt<T>
		{
			fn value(self) -> T;

			fn increment_in_place(&mut self);

			fn difference(self, other: Self) -> usize;
		}

		impl<T> NonNullExt<T> for NonNull<T>
		{
			#[inline(always)]
			fn value(self) -> T
			{
				unsafe { * self.as_ptr() }
			}

			#[inline(always)]
			fn increment_in_place(&mut self)
			{
				self = unsafe { NonNull::new_unchecked(self.as_ptr().add(1)) }
			}

			#[inline(always)]
			fn difference(self, other: Self) -> usize
			{
				(self.as_ptr() as usize) - (other.as_ptr() as usize)
			}
		}

		let length = compare_to.len();
		debug_assert!(length > 1, "compare_to.len() must be more than 1");

		use self::Status::*;

		let potential_slice_end_pointer = unsafe { self.current_pointer.add(length) };
		let can_be_efficiently_done_within_one_slice = potential_slice_end_pointer < self.current_slice_end_pointer;
		if can_be_efficiently_done_within_one_slice
		{
			let mut compare_to_pointer = unsafe { NonNull::new_unchecked(compare_to.as_ptr()) };
			let mut pointer = self.current_pointer;
			while
			{
				if compare_to_pointer.value() != pointer.value()
				{
					return Err(Invalid(invalid_reason))
				}

				compare_to_pointer.increment_in_place();
				pointer.increment_in_place();
				pointer != end_pointer
			}
			{
			}

			self.current_pointer = end_pointer;
			self.current_position = self.current_position.increment_offset_by(length);
			self.previous_position = self.current_pointer.decrement_offset();
			Ok(result)
		}
		else
		{
			// TODO: These scenarios are possible and are not efficiently coded for:-
			//
			// (a) can be done in one slice but ends on current_slice_end_pointer, or
			// (b) covers multiple slices.
			// (c) not enough bytes.
			// (d) self.current_slice_end_pointer == self.current_pointer ie final buffer and no space.
			for item in compare_to
			{
				let compare_to_pointer_value = *item;
				let pointer_value = self.if_has_more_return_current_value_and_increment()?;
				if compare_to_pointer_value != pointer_value
				{
					return Err(Invalid(invalid_reason))
				}
			}
			Ok(result)
		}
	}

	#[inline(always)]
	pub(crate) fn if_has_more_return_current_value_and_increment<ReentryPoint, InvalidReason>(&mut self, reentry_point: ReentryPoint) -> Result<u8, Status<ReentryPoint, InvalidReason>>
	{
		if self.current_pointer == self.current_slice_end_pointer
		{
			return Err(Status::RanOutOfBytes(reentry_point))
		}

		let current_value = self.current_value();

		let current_position = self.current_position;
		self.previous_position = current_position;

		let next_pointer = unsafe { self.current_pointer.add(1) };

		let would_be_end_pointer = next_pointer == self.current_slice_end_pointer;
		if unlikely!(would_be_end_pointer)
		{
			let is_final_buffer_index = current_position.buffer_index.0 == self.vectored_buffer.number_of_buffers() - 1;
			if unlikely!(is_final_buffer_index)
			{
				self.current_position = current_position.increment_offset();
				self.current_pointer = next_pointer;
			}
			else
			{
				self.current_position = current_position.next();
				self.set()
			}
		}
		else
		{
			self.current_position = current_position.increment_offset();
			self.current_pointer = next_pointer;
		}

		Ok(current_value)
	}

	#[inline(always)]
	fn current_value(&self) -> u8
	{
		unsafe { *self.current_pointer.as_ptr() }
	}

	#[inline(always)]
	fn get_current_slice<'b>(&'b self) -> Ref<'b, [u8]>
	{
		self.vectored_buffer.get_immutable_buffer_with_offset(self.current_position.buffer_index, self.current_position.offset)
	}
}
