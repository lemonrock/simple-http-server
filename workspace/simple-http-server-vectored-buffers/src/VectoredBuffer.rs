// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A vectored buffer, ie one consisting of one or more individual buffers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VectoredBuffer(RingBuffer<Buffer>);

impl Default for VectoredBuffer
{
	#[inline(always)]
	fn default() -> Self
	{
		VectoredBuffer(RingBuffer::default())
	}
}

impl VectoredBuffer
{
	/// Use instead of `::std::io::Read.read()`.
	///
	/// Returns a tuple of `(bytes_read, next_offset)`.
	///
	/// If the `VectoredBuffer` is full, `next_offset` will be `None`.
	#[inline(always)]
	pub fn read_into(&mut self, read: &mut impl Read, starting_at: VectoredBufferOffset) -> io::Result<(usize, Option<VectoredBufferOffset>)>
	{
		let offset = starting_at.offset;
		let mut buffer_index = starting_at.buffer_index;

		let bytes_read =
		{
			let mutable_buffer = &mut self.get_mutable_buffer(buffer_index)[offset..];
			let bytes_read = read.read(mutable_buffer)?;
			if likely!(bytes_read != mutable_buffer.len())
			{
				return Ok((bytes_read, Some(VectoredBufferOffset::new(buffer_index, offset + bytes_read))))
			}
			bytes_read
		};


		buffer_index += 1;
		let mut total_bytes_read = bytes_read;
		while self.is_valid_buffer_index(buffer_index)
		{
			let mutable_buffer = self.get_mutable_buffer(buffer_index);

			let bytes_read = read.read(mutable_buffer)?;

			total_bytes_read += bytes_read;
			if likely!(bytes_read != mutable_buffer.len())
			{
				return Ok((total_bytes_read, Some(VectoredBufferOffset::new(buffer_index, bytes_read))))
			}
			buffer_index += 1;
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
		let offset = starting_at.offset;
		let mut buffer_index = starting_at.buffer_index;
		let immutable_buffer = &self.get_immutable_buffer(buffer_index)[offset .. ];

		let bytes_written = write.write(immutable_buffer)?;

		if likely!(bytes_written != immutable_buffer.len())
		{
			return Ok((bytes_written, Some(VectoredBufferOffset::new(buffer_index, offset + bytes_written))))
		}

		buffer_index += 1;
		let mut total_bytes_written = bytes_written;
		while self.is_valid_buffer_index(buffer_index)
		{
			let immutable_buffer = self.get_immutable_buffer(buffer_index);

			let bytes_written = write.write(immutable_buffer)?;

			total_bytes_written += bytes_written;
			if likely!(bytes_written != immutable_buffer.len())
			{
				return Ok((total_bytes_written, Some(VectoredBufferOffset::new(buffer_index, bytes_written))))
			}
			buffer_index += 1;
		}

		Ok((total_bytes_written, None))
	}

	#[inline(always)]
	pub(crate) fn is_valid_buffer_index(&self, buffer_index: usize) -> bool
	{
		buffer_index != self.number_of_buffers()
	}

	/// Number of buffers.
	#[inline(always)]
	pub fn number_of_buffers(&self) -> usize
	{
		self.0.length()
	}

	#[inline(always)]
	fn get_immutable_buffer(&self, buffer_index: usize) -> &[u8]
	{
		self.0.get(buffer_index)
	}

	#[inline(always)]
	fn get_mutable_buffer(&mut self, buffer_index: usize) -> &mut [u8]
	{
		self.0.get_mut(buffer_index)
	}
}
