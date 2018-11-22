// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a list of tags for an `Content-Length` header.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ContentLengthHeaderResponseBuffer(Vec<u8>);

impl HeaderResponseBuffer for ContentLengthHeaderResponseBuffer
{
	#[inline(always)]
	fn report_buffer(&self, buffer_index: usize, from_offset: usize) -> &[u8]
	{
		debug_assert!(buffer_index < 3, "buffer_index `{}` out of range", buffer_index);

		let buffer = match buffer_index
		{
			0 => b"Content-Length:",

			1 => self.0.as_slice(),

			2 => b"\r\n",

			_ => unreachable!(),
		};

		debug_assert!(from_offset < buffer.len(), "from_offset `{}` out of range for buffer_index `{}`", from_offset, buffer_index);

		&buffer[from_offset .. ]
	}
}

impl ContentLengthHeaderResponseBuffer
{
	/// Create a new instance.
	pub fn new(length: usize) -> Self
	{
		let mut vec = Vec::with_capacity_for_usize(0, 0);
		vec.write_usize(length);
		ContentLengthHeaderResponseBuffer(vec)
	}
}
