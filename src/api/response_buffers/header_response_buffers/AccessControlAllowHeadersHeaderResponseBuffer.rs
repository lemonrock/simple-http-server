// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a list of tags for an `Access-Control-Allow-Methods` header.
pub struct AccessControlAllowHeadersHeaderResponseBuffer(Vec<&'static [u8]>);

impl HeaderResponseBuffer for AccessControlAllowHeadersHeaderResponseBuffer
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

impl AccessControlAllowHeadersHeaderResponseBuffer
{
	/// Create a new instance.x
	pub fn new(mut headers: BTreeSet<HeaderName>) -> Self
	{
		debug_assert_ne!(headers.len(), 0, "headers should always contain at least one header name");

		let mut buffers = ArrayVec::new();

		buffers.push((b"Access-Control-Allow-Headers:"));

		let mut headers = headers.iter();
		let first_header = headers.next.unwrap();
		buffers.push(first_header.name);

		for header in headers
		{
			buffers.push(header.leading_comma_then_name);
		}
		buffers.push(b"\r\n");

		AccessControlAllowHeadersHeaderResponseBuffer(buffers)
	}
}
