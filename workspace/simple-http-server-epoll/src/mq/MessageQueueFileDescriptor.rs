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
	/// Removes and destroys a queue.
	///
	/// The message queue name `name` is removed immediately.
	/// The queue itself is destroyed once any other processes that have the queue open close their descriptors referring to the queue.
	///
	/// Failure is caused by the queue not existing or by not having permission.
	#[inline(always)]
	pub fn unlink(name: &CStr) -> Result<(), MessageQueueUnlinkError>
	{
		Self::guard_name(name);

		let result = unsafe { mq_unlink(name.as_ptr()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::MessageQueueUnlinkError::*;

			Err
			(
				match errno().0
				{
					EACCES => PermissionDenied,

					ENOENT => DoesNotExist,

					ENAMETOOLONG => panic!("`name` was too long"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// Creates a new instance.
	#[inline(always)]
	pub fn new(name: &CStr, read_or_write: MessageQueueCreateReadOrWrite, open_or_create: &OpenOrCreateMessageQueue) -> Result<Self, CreationError>
	{
		open_or_create.invoke_mq_open(read_or_write, name)
	}

	/// Obtains queue attributes.
	///
	/// Not particularly useful.
	#[inline(always)]
	pub fn queue_attributes(&self) -> mq_attr
	{
		let mut attributes = unsafe { zeroed() };
		let result = unsafe { mq_getattr(self.0, &mut attributes) };

		if likely!(result == 0)
		{
			attributes
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("The message queue descriptor specified in `mqdes` is invalid"),
				EINVAL => panic!("`newattr.mq_flags` contained set bits other than `O_NONBLOCK`"),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}

	pub(crate) fn guard_name(name: &CStr)
	{
		if cfg!(debug_assertions)
		{
			let bytes = name.to_bytes();
			let length = bytes.len();
			debug_assert!(length > 1, "name must be 2 bytes or more long (excluding the trailing NUL)");
			debug_assert!(length < 256, "name must be 255 bytes or less long (excluding the trailing NUL)");

			debug_assert_eq!(bytes[0], b'/', "name must start with a slash");
			for byte in name.to_bytes()[ 1 .. ].iter()
			{
				debug_assert_ne!(byte, &b'/', "name contains more than one slash");
			}
		}
	}
}

/*

ssize_t mq_receive(mqd_t, char *, size_t, unsigned *);
ssize_t mq_timedreceive(mqd_t, char *__restrict, size_t, unsigned *__restrict, const struct timespec *__restrict);

int mq_send(mqd_t, const char *, size_t, unsigned);
int mq_timedsend(mqd_t, const char *, size_t, unsigned, const struct timespec *);

?mq_notify?

int mq_unlink(const char *);
*/
