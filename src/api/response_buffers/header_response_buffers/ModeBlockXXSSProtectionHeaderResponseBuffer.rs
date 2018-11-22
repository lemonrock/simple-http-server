// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Returns a header-value of `X-XSS-Protection:1;mode=block`.
///
/// Other forms are possible but not supported by this struct as they are rarely used or limited in implementation support:-
///
/// * `0`
/// * `1`
/// * `1;report=<reporting-URI>` only works on Chrome.
pub struct ModeBlockXXSSProtectionHeaderResponseBuffer;

impl HeaderResponseBuffer for ModeBlockXXSSProtectionHeaderResponseBuffer
{
	#[inline(always)]
	fn report_buffer(&self, buffer_index: usize, from_offset: usize) -> &[u8]
	{
		const Buffer: &'static [u8] = b"X-XSS-Protection:1;mode=block\r\n";

		debug_assert_eq!(buffer_index, 0, "buffer_index `{}` out of range", buffer_index);
		debug_assert!(from_offset < Buffer.len(), "from_offset `{}` out of range", from_offset);

		&Buffer[from_offset .. ]
	}
}
