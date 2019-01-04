// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Optional message queue creation settings.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptionalPosixMessageQueueCreateSettings
{
	/// Must not be zero or negative.
	///
	/// Default (if this structure is omitted during creation) is in `/proc/sys/fs/mqueue/msg_default` and is usually `10` since Linux 3.5.
	///
	/// The maximum value is in `/proc/sys/fs/mqueue/msg_max`, and is usually `10` since Linux 3.5.
	///
	/// Processes with the capability `CAP_SYS_RESOURCE` may exceed this maximum, up to `65,536`.
	pub maximum_number_of_enqueued_messages: isize,

	/// Must not be zero or negative.
	///
	/// Default () is in `/proc/sys/fs/mqueue/msgsize_default` and is usually `8,192` since Linux 3.5.
	///
	/// The maximum value is in `/proc/sys/fs/mqueue/msgsize_max`, and is usually `8,192` since Linux 3.5.
	///
	/// Processes with the capability `CAP_SYS_RESOURCE` may exceed this maximum, up to `16,777,216`.
	pub maximum_message_size_in_bytes: isize,
}
