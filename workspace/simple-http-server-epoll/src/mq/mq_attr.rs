// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A `mq_attr` structure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct mq_attr
{
	// x32 compatibility (See <https://sourceware.org/bugzilla/show_bug.cgi?id=21279>)
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] mq_flags: i64,
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] mq_maxmsg: i64,
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] mq_msgsize: i64,
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] mq_curmsgs: i64,
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] pad: [i64; 4],

	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] mq_flags: isize,
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] mq_maxmsg: isize,
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] mq_msgsize: isize,
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] mq_curmsgs: isize,
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] pad: [isize; 4],
}

impl mq_attr
{
	#[inline(always)]
	pub(crate) fn for_create(optional_message_queue_create_settings: &OptionalMessageQueueCreateSettings) -> Self
	{
		Self
		{
			mq_flags: unsafe { uninitialized() },
			mq_maxmsg: optional_message_queue_create_settings.maximum_number_of_enqueued_messages,
			mq_msgsize: optional_message_queue_create_settings.maximum_message_size_in_bytes,
			mq_curmsgs: unsafe { uninitialized() },
			pad: unsafe { uninitialized() },
		}
	}
}
