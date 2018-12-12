// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Read, write or read and write?
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum MessageQueueCreateReadOrWrite
{
	/// Only read.
	Read = O_RDONLY | O_CLOEXEC | O_NONBLOCK,

	/// Only write.
	Write = O_WRONLY | O_CLOEXEC | O_NONBLOCK,

	/// Read and write.
	ReadAndWrite = O_RDWR | O_CLOEXEC | O_NONBLOCK,
}

impl Default for MessageQueueCreateReadOrWrite
{
	#[inline(always)]
	fn default() -> Self
	{
		MessageQueueCreateReadOrWrite::ReadAndWrite
	}
}
