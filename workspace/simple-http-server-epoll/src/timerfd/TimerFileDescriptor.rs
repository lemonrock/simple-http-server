// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a timer instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerFileDescriptor(RawFd);

impl Drop for TimerFileDescriptor
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

impl TimerFileDescriptor
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(clock: TimerClock) -> Result<Self, TimerCreationError>
	{
		let result = unsafe { timerfd_create(clock as i32, TFD_NONBLOCK | TFD_CLOEXEC) };
		if likely!(result != -1)
		{
			Ok(TimerFileDescriptor(result))
		}
		else
		{
			use self::TimerCreationError::*;

			Err
			(
				match errno().0
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOMEM => KernelWouldBeOutOfMemory,
					EINVAL => panic!("Invalid clockid or flags"),
					ENODEV => panic!("Could not mount (internal) anonymous inode device"),
					_ => unreachable!(),
				}
			)
		}
	}

	/// Get the value of the timer.
	#[inline(always)]
	pub fn get(&self) -> itimerspec
	{
		let mut current_value = unsafe { uninitialized() };
		let result = unsafe { timerfd_gettime(self.0, &mut current_value) };
		if likely!(result == 0)
		{
			return current_value
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("`fd` is not a valid file descriptor"),
				EFAULT => panic!("curr_value` is not a valid pointer"),
				EINVAL => panic!("`fd` is not a valid timerfd file descriptor"),
				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}

	/// Set the value of the timer.
	///
	/// Returns the previous value of the timer.
	#[inline(always)]
	pub fn set(&self, new_value: &itimerspec, interpretation_of_new_value: TimerSetChoices) -> itimerspec
	{
		let mut old_value = unsafe { uninitialized() };
		let result = unsafe { timerfd_settime(self.0, interpretation_of_new_value as i32, new_value, &mut old_value) };
		if likely!(result == 0)
		{
			return old_value
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("`fd` is not a valid file descriptor"),
				EFAULT => panic!("`new_value` or `old_value` is not a valid pointer"),
				EINVAL => panic!("arguments were invalid"),
				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}
}
