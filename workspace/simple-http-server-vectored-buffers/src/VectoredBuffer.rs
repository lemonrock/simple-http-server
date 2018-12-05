// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A vectored buffer, ie one consisting of one or more individual buffers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VectoredBuffer
{
	buffers: RingBuffer<BufferReference>,
	allocation_observer_identifier: AllocationObserverIdentifier,
}

impl<'b> VectoredBuffer
{
	/// Use instead of `::std::io::Read.read()`.
	///
	/// Returns a tuple of `(bytes_read, next_offset)`.
	///
	/// If the `VectoredBuffer` is full, `next_offset` will be `None`.
	#[inline(always)]
	pub fn read_into(&self, read: &mut impl Read, starting_at: VectoredBufferOffset) -> io::Result<(usize, Option<VectoredBufferOffset>)>
	{
		let mut buffer_index = starting_at.buffer_index;
		let offset = starting_at.offset;

		let bytes_read =
		{
			let bytes_read = read.read(self.get_mutable_buffer_with_offset(buffer_index, offset))?;
			if likely!(bytes_read != mutable_buffer.len())
			{
				return Ok((bytes_read, Some(VectoredBufferOffset::new(buffer_index, offset + bytes_read))))
			}
			bytes_read
		};


		buffer_index.increment_unchecked();
		let mut total_bytes_read = bytes_read;
		while self.is_valid_buffer_index(buffer_index)
		{
			let bytes_read = read.read(self.get_mutable_buffer(buffer_index))?;

			total_bytes_read += bytes_read;
			if likely!(bytes_read != mutable_buffer.len())
			{
				return Ok((total_bytes_read, Some(VectoredBufferOffset::new(buffer_index, bytes_read))))
			}
			buffer_index.increment_unchecked();
		}

		Ok((total_bytes_read, None))
	}

	/// Use instead of `::std::io::Write.write()`.
	///
	/// Returns a tuple of `(bytes_written, next_offset)`.
	///
	/// If the `VectoredBuffer` has nothing more to write, `next_offset` will be `None`.
	#[inline(always)]
	pub fn write_from(&self, write: &mut impl Write, starting_at: VectoredBufferOffset) -> io::Result<(usize, Option<VectoredBufferOffset>)>
	{
		let mut buffer_index = starting_at.buffer_index;
		let offset = starting_at.offset;

		let bytes_written = write.write(self.get_immutable_buffer_with_offset(buffer_index, offset))?;

		if likely!(bytes_written != immutable_buffer.len())
		{
			return Ok((bytes_written, Some(VectoredBufferOffset::new(buffer_index, offset + bytes_written))))
		}

		buffer_index.increment_unchecked();
		let mut total_bytes_written = bytes_written;
		while self.is_valid_buffer_index(buffer_index)
		{
			let bytes_written = write.write(self.get_immutable_buffer(buffer_index))?;

			total_bytes_written += bytes_written;
			if likely!(bytes_written != immutable_buffer.len())
			{
				return Ok((total_bytes_written, Some(VectoredBufferOffset::new(buffer_index, bytes_written))))
			}
			buffer_index.increment_unchecked();
		}

		Ok((total_bytes_written, None))
	}

	#[inline(always)]
	pub(crate) fn get_immutable_buffer_with_offset(&self, buffer_index: BufferIndex, offset: InclusiveFromOffset) -> Ref<'b, [u8]>
	{
		let immutable_buffer = self.get_immutable_buffer(buffer_index);
		Ref::map(immutable_buffer, |immutable_buffer| &immutable_buffer[ofset .. ])
	}

	#[inline(always)]
	fn get_immutable_buffer(&self, buffer_index: BufferIndex) -> Ref<'b, [u8]>
	{
		self.buffers.borrow(buffer_index)
	}

	#[inline(always)]
	fn get_mutable_buffer_with_offset(&self, buffer_index: BufferIndex, offset: InclusiveFromOffset) -> RefMut<'b, [u8]>
	{
		let mutable_buffer = self.get_mutable_buffer(buffer_index);
		RefMut::map(mutable_buffer, |mutable_buffer| &mut mutable_buffer[ofset .. ])
	}

	#[inline(always)]
	fn get_mutable_buffer(&self, buffer_index: BufferIndex) -> RefMut<'b, [u8]>
	{
		self.buffers.borrow_mut(buffer_index)
	}
}

impl VectoredBuffer
{
	/// Creates a new instance and gives it an initial buffer from `buffer_arena`.
	///
	/// The `buffer_arena` is not stored with the `VectoredBuffer`, allowing one to use buffers from different arenas.
	///
	/// The `allocation_observer_identifier` is usually a HTTP connection identifier, and can be used, along with the AllocationObserver in BufferArena, to prevent greedy use of buffers by a particular connection.
	///
	/// For instance `allocation_observer_identifier` could transmute into a reference to a struct which hold a count of how many buffers have been allocated for a connection.
	///
	/// It could also double as a `mio` `Token`.
	///
	/// On AMD 64-bit systems, the lowest 3 bits and highest 12 bits can also be used as a packed pointer.
	#[inline(always)]
	pub fn new(allocation_observer_identifier: AllocationObserverIdentifier, buffer_arena: &Arc<BufferArena<impl AllocationObserver>>) -> Result<(Self, BufferIndex), ()>
	{
		let mut this = Self
		{
			buffers: RingBuffer::new(),
			allocation_observer_identifier,
		};

		let buffer_index = this.allocate_another_buffer(buffer_arena)?;
		Ok((this, buffer_index))
	}

	/// Allocates another buffer.
	///
	/// Returns an Err if:-
	///
	/// * The underlying ring buffer for this VectoredBuffer is full;
	/// * There are no buffers left to allocate (effectively out of memory in the chosen `buffer_arena`);
	///
	/// Otherwise returns the `BufferIndex` of the allocated buffer.
	#[inline(always)]
	pub fn allocate_another_buffer(&mut self, buffer_arena: &Arc<BufferArena<impl AllocationObserver>>) -> Result<BufferIndex, ()>
	{
		if self.buffers.is_full()
		{
			return Err(())
		}

		match BufferArena::allocate(buffer_arena, self.allocation_observer_identifier)
		{
			None => Err(()),
			Some(buffer_reference) => Ok(self.buffers.insert_unchecked(buffer_reference)),
		}
	}

	/// Retires (drops) a buffer if there is more than one.
	///
	/// In practice, a buffer is only freed when all `BufferReference`s to it are dropped.
	///
	/// Retiring a buffer creates space in the VectoredBuffer for more buffers to be added.
	///
	/// If there is only one buffer remaining, it will not retire it.
	///
	/// Returns `true` while there are still more buffers that could be retired.
	pub fn retire_oldest_buffer(&mut self) -> bool
	{
		if unlikely!(self.buffers.length() <= 1)
		{
			return false
		}

		self.buffers.remove_unchecked();

		false
	}

	/// Recycles a buffer if there is more than one, taking the oldest buffer and making it the youngest.
	///
	/// Returns the buffer index of the inserted buffer.
	///
	/// This is equivalent to `retire_oldest_buffer()` followed by `allocate_another_buffer()` but much more efficient.
	///
	/// However, if some other code holds a reference to this buffer, and the contents are subsequently over-written, that code will now be looking at invalid memory.
	pub fn recycle_oldest_buffer(&mut self) -> Option<BufferIndex>
	{
		if unlikely!(self.buffers.length() <= 1)
		{
			return None
		}

		let buffer_reference = self.buffers.remove_unchecked_and_undropped();

		let buffer_index = self.buffers.insert_unchecked(buffer_reference);

		Some(buffer_index)
	}

	/// Is `buffer_index` valid?
	#[inline(always)]
	pub fn is_valid_buffer_index(&self, buffer_index: BufferIndex) -> bool
	{
		buffer_index != self.number_of_buffers()
	}

	/// Number of buffers.
	#[inline(always)]
	pub fn number_of_buffers(&self) -> usize
	{
		self.buffers.length()
	}

}
