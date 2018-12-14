// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


pub(crate) struct MutableMessageHeadersIterator<'a>
{
	parent: &'a msghdr,
	next: Option<&'a mut cmsghdr>,
}

impl<'a> Iterator for MutableMessageHeadersIterator<'a>
{
	type Item = &'a mut cmsghdr;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let next_message_header = self.next.take();
		if let Some(next) = next_message_header
		{
			self.next = next_message_header.next_mut(self.parent);
		}

		next_message_header
	}
}
