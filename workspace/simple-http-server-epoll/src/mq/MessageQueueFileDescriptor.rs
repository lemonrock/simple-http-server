// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a POSIX message queue instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageQueueFileDescriptor(mqd_t);

impl Drop for MessageQueueFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for MessageQueueFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for MessageQueueFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl MessageQueueFileDescriptor
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(name: &CStr, read_or_write: MessageQueueCreateReadOrWrite, open_or_create: &OpenOrCreateMessageQueue) -> Result<Self, CreationError>
	{
		open_or_create.invoke_mq_open(read_or_write, name)
	}
}
