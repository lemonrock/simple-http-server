// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a fanotify instance.
///
/// fanotify requires the `CAP_SYS_ADMIN` capability, so is only suitable for priveleged processes or those running as root.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FanotifyFileDescriptor(RawFd);

impl Drop for FanotifyFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for FanotifyFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for FanotifyFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FanotifyFileDescriptor
{
	/// Creates a new instance.
	///
	/// The `Notification` class is always enabled.
	#[inline(always)]
	pub fn new(use_precontent_class: bool, use_content_class: bool, file_status_flags: FileStatusFlags) -> Result<Self, CreationError>
	{
		let mut classes = FAN_CLASS_NOTIF;
		if likely!(use_precontent_class)
		{
			classes |= FAN_CLASS_PRE_CONTENT;
		}
		if likely!(use_content_class)
		{
			classes |= FAN_CLASS_CONTENT;
		}

		let result = unsafe { fanotify_init(FAN_CLOEXEC | FAN_NONBLOCK | classes | FAN_UNLIMITED_QUEUE | FAN_UNLIMITED_MARKS, file_status_flags.bits) };
		if likely!(result >= 0)
		{
			Ok(FanotifyFileDescriptor(result))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			Err
			(
				match errno().0
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOMEM => KernelWouldBeOutOfMemory,
					EPERM => PermissionDenied,
					EINVAL => panic!("Invalid arguments"),
					ENOSYS => panic!("The fanotify API is available only if the kernel was configured with `CONFIG_FANOTIFY`"),
					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!();
		}
	}

	/// Adds a mark.
	#[inline(always)]
	pub fn add_mark<'a>(&self, mark_flags: MarkFlags, mark_event_flags: MarkEventFlags, mark_path: &MarkPath<'a>) -> Result<(), FanotifyMarkError>
	{
		let (dirfd, pathname) = mark_path.to_dirfd_and_pathname();

		let result = unsafe { fanotify_mark(self.0, mark_flags.bits | FAN_MARK_ADD, mark_event_flags.bits, dirfd, pathname) };

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::FanotifyMarkError::*;

			Err
			(
				match errno().0
				{
					EBADF => panic!("An invalid file descriptor was passed in `fanotify_fd`"),
					EINVAL => panic!("An invalid value was passed in `flags` or `mask`, or `fanotify_fd` was not an fanotify file descriptor, or the fanotify file descriptor was opened with `FAN_CLASS_NOTIF` and mask contains a flag for permission events (`FAN_OPEN_PERM` or `FAN_ACCESS_PERM`)"),
					ENOENT | ENOTDIR => FilePathInvalid,
					ENOMEM | ENOSPC => KernelWouldBeOutOfMemory,
					ENOSYS => panic!("This kernel does not implement `fanotify_mark()`. The fanotify API is available only if the kernel was configured with `CONFIG_FANOTIFY`"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!();
		}
	}

	/// Remove a mark.
	#[inline(always)]
	pub fn remove_mark<'a>(&self, mark_flags: MarkFlags, mark_event_flags: MarkEventFlags, mark_path: &MarkPath<'a>) -> Result<(), FanotifyMarkError>
	{
		let (dirfd, pathname) = mark_path.to_dirfd_and_pathname();

		let result = unsafe { fanotify_mark(self.0, mark_flags.bits | FAN_MARK_REMOVE, mark_event_flags.bits, dirfd, pathname) };

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::FanotifyMarkError::*;

			Err
			(
				match errno().0
				{
					EBADF => panic!("An invalid file descriptor was passed in `fanotify_fd`"),
					EINVAL => panic!("An invalid value was passed in `flags` or `mask`, or `fanotify_fd` was not an fanotify file descriptor, or the fanotify file descriptor was opened with `FAN_CLASS_NOTIF` and mask contains a flag for permission events (`FAN_OPEN_PERM` or `FAN_ACCESS_PERM`)"),
					ENOENT | ENOTDIR => FilePathInvalid,
					ENOMEM | ENOSPC => KernelWouldBeOutOfMemory,
					ENOSYS => panic!("This kernel does not implement `fanotify_mark()`. The fanotify API is available only if the kernel was configured with `CONFIG_FANOTIFY`"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!();
		}
	}

	/// Remove all.
	#[inline(always)]
	pub fn remove_all_mount_or_non_mount_marks<'a>(&self, remove_mount: bool, mark_event_flags: MarkEventFlags, mark_path: &MarkPath<'a>) -> Result<(), FanotifyMarkError>
	{
		let mark_flags = if remove_mount
		{
			FAN_MARK_MOUNT
		}
		else
		{
			0
		};

		let (dirfd, pathname) = mark_path.to_dirfd_and_pathname();

		let result = unsafe { fanotify_mark(self.0, mark_flags | FAN_MARK_FLUSH, mark_event_flags.bits, dirfd, pathname) };

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::FanotifyMarkError::*;

			Err
			(
				match errno().0
				{
					EBADF => panic!("An invalid file descriptor was passed in `fanotify_fd`"),
					EINVAL => panic!("An invalid value was passed in `flags` or `mask`, or `fanotify_fd` was not an fanotify file descriptor, or the fanotify file descriptor was opened with `FAN_CLASS_NOTIF` and mask contains a flag for permission events (`FAN_OPEN_PERM` or `FAN_ACCESS_PERM`)"),
					ENOENT | ENOTDIR => FilePathInvalid,
					ENOMEM | ENOSPC => KernelWouldBeOutOfMemory,
					ENOSYS => panic!("This kernel does not implement `fanotify_mark()`. The fanotify API is available only if the kernel was configured with `CONFIG_FANOTIFY`"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!();
		}
	}

	/// Reads a fanotify event.
	///
	/// Use this only after a read-ready event notification is received (using edge-triggered events).
	///
	/// Returns the number of items read; adds read items starting at `read_into.len()` up to `read_into.capacity()`.
	#[inline(always)]
	pub fn read(&self, read_into: &mut Vec<fanotify_event_metadata>) -> Result<usize, StructReadError>
	{
		use self::StructReadError::*;

		const SizeOfRead: usize = size_of::<fanotify_event_metadata>();

		let length = read_into.len();

		let starting_at = unsafe { read_into.as_mut_ptr().add(length) };
		let extra_items = read_into.capacity() - length;

		if unlikely!(extra_items == 0)
		{
			return Ok(0)
		}

		let result = unsafe { read(self.0, starting_at as *mut _, extra_items * SizeOfRead) };

		if likely!(result > 0)
		{
			let items_read = (result as usize) / SizeOfRead;
			unsafe { read_into.set_len(length + items_read) };
			Ok(items_read)
		}
		else
		{
			match result
			{
				-1 =>
				{
					let error_number = errno();
					match error_number.0
					{
						EAGAIN => Err(WouldBlock),
						ECANCELED => Err(Cancelled),
						EINTR => Err(Interrupted),
						EIO => Err(Cancelled),
						EBADF => panic!("`fd` is not a valid file descriptor or is not open for reading"),
						EFAULT => panic!("`buf` is outside your accessible address space"),
						EINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
						EISDIR => panic!("`fd` refers to a directory"),

						_ => panic!("Unexpected error `{}`", error_number),
					}
				}

				0 => panic!("End of file but we haven't closed the file descriptor"),

				_ => unreachable!(),
			}
		}
	}

	/// For permission events, the application must write responses.
	///
	/// Returns the number of responses written.
	///
	/// For extremely efficient handling, the `responses` slice should be backed by a virtual ring buffer (mirror buffer).
	#[inline(always)]
	pub fn write_permission(&self, responses: &[fanotify_response]) -> Result<usize, StructWriteError>
	{
		use self::StructWriteError::*;

		const SizeOfWrite: usize = size_of::<fanotify_response>();

		let result = unsafe { write(self.0, responses.as_ptr() as *const _, responses.len() * SizeOfWrite) };

		if likely!(result > 0)
		{
			let items_written = (result as usize) / SizeOfWrite;
			Ok(items_written)
		}
		else
		{
			match result
			{
				-1 =>
				{
					let error_number = errno();
					match error_number.0
					{
						EAGAIN => Err(WouldBlock),
						ECANCELED => Err(Cancelled),
						EINTR => Err(Interrupted),
						EIO | EPIPE => Err(Cancelled),
						EBADF => panic!("`fd` is not a valid file descriptor or is not open for reading"),
						EFAULT => panic!("`buf` is outside your accessible address space"),
						EINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
						ENOSPC => panic!("out of space"),
						EDQUOT => panic!("out of quota"),
						EDESTADDRREQ => panic!("EDESTADDRREQ!"),
						EFBIG => panic!("EFBIG!"),

						_ => panic!("Unexpected error `{}`", error_number),
					}
				}

				0 => panic!("End of file but we haven't closed the file descriptor"),

				_ => unreachable!(),
			}
		}
	}
}
