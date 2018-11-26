// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Manages buffers in the header response.
pub trait HeaderResponseBuffer
{
	/// Must be at least one, but can be more.
	#[inline(always)]
	fn number_of_buffers(&self) -> usize
	{
		1
	}

	/// * `buffer_index` is between 0 and `self.number_of_buffers() - 1` inclusive.
	/// * `from_offset` is between 0 and the length of the buffer from `buffer_index` less one.
	fn report_buffer(&self, buffer_index: usize, from_offset: usize) -> &[u8];
}
