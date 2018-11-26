// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a list of tags for a `Cache-Control` header.
pub struct CacheControlHeaderResponseBuffer(ArrayVec<[Cow<'static, [u8]>; 14]>);

impl HeaderResponseBuffer for CacheControlHeaderResponseBuffer
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

impl CacheControlHeaderResponseBuffer
{
	/// Create a new instance.
	pub fn new(mut response_cache_control_directives: BTreeSet<ResponseCacheControlDirective>) -> Self
	{
		debug_assert_ne!(response_cache_control_directives.len(), 0, "response_cache_control_directives should always contain at least one tag");

		let mut buffers = ArrayVec::new();

		use self::Cow::Borrowed;

		buffers.push(Borrowed(b"Cache-Control:"));

		let mut response_cache_control_directives = response_cache_control_directives.iter();
		let first_response_cache_control_directive = response_cache_control_directives.next.unwrap();
		buffers.push(first_response_cache_control_directive.buffer());

		for response_cache_control_directive in response_cache_control_directives
		{
			buffers.push(response_cache_control_directive.with_leading_comma_buffer());
		}
		buffers.push(Borrowed(b"\r\n"));

		CacheControlHeaderResponseBuffer(buffers)
	}
}
