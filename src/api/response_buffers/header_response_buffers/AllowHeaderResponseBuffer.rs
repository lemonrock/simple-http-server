// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a list of tags for an `Allow` header.
pub struct AllowHeaderResponseBuffer(ArrayVec<[&'static [u8]; 11]>);

impl HeaderResponseBuffer for AllowHeaderResponseBuffer
{
	#[inline(always)]
	fn report_buffer(&self, buffer_index: usize, from_offset: usize) -> &[u8]
	{
		debug_assert!(buffer_index < self.0.len(), "buffer_index `{}` out of range", buffer_index);

		let buffer = (unsafe { self.0.get_unchecked(buffer_index) }).borrow();

		debug_assert!(from_offset < buffer.len(), "from_offset `{}` out of range for buffer_index `{}`", from_offset, buffer_index);

		&buffer[from_offset .. ]
	}
}

impl AllowHeaderResponseBuffer
{
	/// Create a new instance.
	///
	/// `request_methods` can be empty.
	pub fn new(mut request_methods: BTreeSet<ResponseCacheControlDirective>) -> Self
	{
		let mut buffers = ArrayVec::new();

		buffers.push((b"Allow:"));

		let mut request_methods = request_methods.iter();
		let first_request_method = request_methods.next.unwrap();
		buffers.push(first_request_method.buffer());

		for request_method in request_methods
		{
			buffers.push(request_method.with_leading_comma_buffer());
		}
		buffers.push(b"\r\n");

		AllowHeaderResponseBuffer(buffers)
	}
}
