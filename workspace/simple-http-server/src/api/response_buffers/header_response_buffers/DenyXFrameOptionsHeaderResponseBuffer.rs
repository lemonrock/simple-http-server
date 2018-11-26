// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Returns a header-value of `X-Frame-Options:deny`.
///
/// Whilst the values `sameorigin` and `allow-from https://example.com/` are also possible values, they are of limited benefit and limited browser support.
pub struct DenyXFrameOptionsHeaderResponseBuffer;

impl HeaderResponseBuffer for DenyXFrameOptionsHeaderResponseBuffer
{
	#[inline(always)]
	fn report_buffer(&self, buffer_index: usize, from_offset: usize) -> &[u8]
	{
		const Buffer: &'static [u8] = b"X-Frame-Options:deny\r\n";

		debug_assert_eq!(buffer_index, 0, "buffer_index `{}` out of range", buffer_index);
		debug_assert!(from_offset < Buffer.len(), "from_offset `{}` out of range", from_offset);

		&Buffer[from_offset .. ]
	}
}
