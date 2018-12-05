// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents an epoll instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPollFileDescriptor(RawFd);

impl Drop for EPollFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// Please see <http://austingroupbugs.net/view.php?id=529> and <http://austingroupbugs.net/view.php?id=529> for why ignoring the `EINTR` error on close is actually sane.
		//
		// Frankly, the defects here are those of POSIX: (a) signals, and (b) using a file descriptor so small that it isn't thread safe.
		//
		// To be far, both signals and file descriptors predate threads by a long way.
		unsafe { close(self.0) };
	}
}

impl EPollFileDescriptor
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new() -> Result<Self, EPollCreationError>
	{
		use self::EPollCreationError::*;

		let result = unsafe { epoll_create1(EPOLL_CLOEXEC) };
		if likely!(result >= 0)
		{
			Ok(EPollFileDescriptor(result))
		}
		else if likely!(result == -1)
		{
			Err
			(
				match errno().0
				{
					EMFILE => PerUseLimitOnNumberOfEpollInstancesOrPerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,

					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,

					ENOMEM => KernelWouldBeOutOfMemory,

					EINVAL => panic!("Invalid value specified in flags"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// Adds a file descriptor to an EPoll instance.
	#[inline(always)]
	pub fn add(&self, fd: RawFd, events: u32, token: u64) -> Result<(), EPollAddError>
	{
		let mut event = epoll_event
		{
			events,
			data: epoll_data_t
			{
				u64: token,
			},
		};

		use self::EPollAddError::*;

		match unsafe { epoll_ctl(self.0, EPOLL_CTL_ADD, fd, &mut event) }
		{
			0 => Ok(()),

			-1 => Err
			(
				match errno().0
				{
					ENOMEM => ThereWasInsufficientKernelMemory,

					ENOSPC => LimitOnWatchesWouldBeExceeded,

					EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
					EEXIST => panic!("The supplied file descriptor was already registered with this epoll instance"),
					EINVAL => panic!("Can not add epoll file descriptor to its self, or can not make wait on an epoll file descriptor `EPOLLEXCLUSIVE`"),
					ELOOP => panic!("The supplied file descriptor is for an epoll instance and this operation would result in a circular loop of epoll instances monitoring one another"),
					EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),

					_ => unreachable!(),
				}
			),

			_ => unreachable!(),
		}
	}

	/// Modifies a file descriptor in an EPoll instance.
	#[inline(always)]
	pub fn modify(&self, fd: RawFd, events: u32, token: u64) -> Result<(), EPollModifyError>
	{
		let mut event = epoll_event
		{
			events,
			data: epoll_data_t
			{
				u64: token,
			},
		};

		match unsafe { epoll_ctl(self.0, EPOLL_CTL_MOD, fd, &mut event) }
		{
			0 => Ok(()),

			-1 => match errno().0
			{
				ENOMEM => Err(EPollModifyError::ThereWasInsufficientKernelMemory),

				EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
				EINVAL => panic!("Supplied file descriptor was not usable or there was the presence or absence of `EPOLLEXCLUSIVE` when required"),
				ENOENT => panic!("The supplied file descriptor is not registered with this epoll instance"),
				EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),

				_ => unreachable!(),
			},

			_ => unreachable!(),
		}
	}

	/// Deletes a file descriptor in an EPoll instance.
	#[inline(always)]
	pub fn delete(&self, fd: RawFd) -> Result<(), EPollDeleteError>
	{
		match unsafe { epoll_ctl(self.0, EPOLL_CTL_DEL, fd, null_mut()) }
		{
			0 => Ok(()),

			-1 => match errno().0
			{
				ENOMEM => Err(EPollDeleteError::ThereWasInsufficientKernelMemory),

				EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
				EINVAL => panic!("Supplied file descriptor was not usable"),
				ENOENT => panic!("The supplied file descriptor is not registered with this epoll instance"),
				EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),

				_ => unreachable!(),
			},

			_ => unreachable!(),
		}
	}

	/// Waits for events.
	///
	/// Fills `events` as much as possible and returns it as a slice.
	///
	/// Returns an error if interrupted.
	///
	/// No error occurs if a time out occurred.
	#[inline(always)]
	pub fn wait<'a>(&self, events: &'a mut [epoll_event], time_out: EPollTimeOut) -> Result<&'a [epoll_event], EPollWaitError>
	{
		let length = events.len();

		debug_assert_ne!(length, 0, "events.len() can not be zero");
		debug_assert!(length <= ::std::i32::MAX as usize, "events.len() can not exceed ::std::i32::MAX");

		let result = unsafe { epoll_wait(self.0, events.as_mut_ptr(), length as i32, time_out.into()) };
		if likely!(result >= 0)
		{
			Ok(&events[0 .. result as usize])
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => Err(EPollWaitError::Interrupted),

				EBADF => panic!("`epfd` is not a valid file descriptor"),
				EFAULT => panic!("Memory for events was not writable"),
				EINVAL => panic!("`epfd` is not an epoll file descriptor"),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!();
		}
	}

	/// Similar to `wait()` but atomically changes the signal mask to `signal_mask` for the duration of the call.
	#[inline(always)]
	pub fn wait_signalled<'a>(&self, events: &'a mut [epoll_event], time_out: EPollTimeOut, signal_mask: &sigset_t) -> Result<&'a [epoll_event], EPollWaitError>
	{
		let length = events.len();

		debug_assert_ne!(length, 0, "events.len() can not be zero");
		debug_assert!(length <= ::std::i32::MAX as usize, "events.len() can not exceed ::std::i32::MAX");

		let result = unsafe { epoll_pwait(self.0, events.as_mut_ptr(), length as i32, time_out.into(), signal_mask) };
		if likely!(result >= 0)
		{
			Ok(&events[0 .. result as usize])
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => Err(EPollWaitError::Interrupted),

				EBADF => panic!("`epfd` is not a valid file descriptor"),
				EFAULT => panic!("Memory for events was not writable"),
				EINVAL => panic!("`epfd` is not an epoll file descriptor"),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!();
		}
	}
}


// TODO Share a file descriptor across threads
// SO_REUSEPORT with SO_INCOMING_CPU
// EPOLLEXCLUSIVE

