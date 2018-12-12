// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// How to open or create (or both) a message queue.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OpenOrCreateMessageQueue
{
	/// Opens the queue if it already exists; fails if it does not.
	OpenIfAlreadyExistsOrFail,

	/// Opens the queue if it already exists; creates (and implicitly opens) it is it does not.
	OpenOrCreateIfDoesNotExist(MessageQueueCreateSettings),

	/// Creates (and implicitly opens) the queue if it does not already exist; fails if it does exist.
	CreateIfItDoesNotExistOrFail(MessageQueueCreateSettings),
}

impl OpenOrCreateMessageQueue
{
	#[inline(always)]
	pub(crate) fn invoke_mq_open(&self, read_or_write: MessageQueueCreateReadOrWrite, name: &CStr) -> Result<MessageQueueFileDescriptor, CreationError>
	{
		MessageQueueFileDescriptor::guard_name(name);

		use self::OpenOrCreateMessageQueue::*;

		let oflag = read_or_write as i32;

		let name_pointer = name.as_ptr();

		use self::CreationError::*;

		match self
		{
			&OpenIfAlreadyExistsOrFail =>
			{
				let result = unsafe { mq_open(name_pointer, oflag) };

				if likely!(result >= 0)
				{
					Ok(MessageQueueFileDescriptor(result))
				}
				else if likely!(result == 0)
				{
					Err
					(
						match errno().0
						{
							EACCES => PermissionDenied,

							EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,

							ENFILE | ENOSPC => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,

							ENOMEM => KernelWouldBeOutOfMemory,

							ENOENT => panic!("No queue with this name exists"),

							ENAMETOOLONG => panic!("`name` is too long"),

							EINVAL => panic!("`name` is invalid in some way"),

							_ => unreachable!(),
						}
					)
				}
				else
				{
					unreachable!();
				}
			}

			&OpenOrCreateIfDoesNotExist(ref create_settings) =>
			{
				let result = create_settings.invoke_mq_open(name_pointer, oflag | O_CREAT);

				if likely!(result >= 0)
				{
					Ok(MessageQueueFileDescriptor(result))
				}
				else if likely!(result == 0)
				{
					Err
					(
						match errno().0
						{
							EACCES => PermissionDenied,

							EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,

							ENFILE | ENOSPC => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,

							ENOMEM => KernelWouldBeOutOfMemory,

							EINVAL => PermissionDenied,

							ENOENT => panic!("`name` was just \"/\" followed by no other characters"),

							ENAMETOOLONG => panic!("`name` is too long"),

							_ => unreachable!(),
						}
					)
				}
				else
				{
					unreachable!();
				}
			}

			&CreateIfItDoesNotExistOrFail(ref create_settings) =>
			{
				let result = create_settings.invoke_mq_open(name_pointer, oflag | O_CREAT | O_EXCL);

				if likely!(result >= 0)
				{
					Ok(MessageQueueFileDescriptor(result))
				}
				else if likely!(result == 0)
				{
					Err
					(
						match errno().0
						{
							EACCES => PermissionDenied,

							EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,

							ENFILE | ENOSPC => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,

							ENOMEM => KernelWouldBeOutOfMemory,

							EINVAL => PermissionDenied,

							ENOENT => panic!("`name` was just \"/\" followed by no other characters"),

							ENAMETOOLONG => panic!("`name` is too long"),

							EEXIST => panic!("queue already exists"),

							_ => unreachable!(),
						}
					)
				}
				else
				{
					unreachable!();
				}
			}
		}
	}
}
