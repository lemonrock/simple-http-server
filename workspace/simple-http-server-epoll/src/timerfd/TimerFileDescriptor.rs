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

	/// Reads the time from the timer.
	///
	/// Use this only after a read-ready event notification is received (using edge-triggered events).
	#[inline(always)]
	pub fn read(&self) -> Result<u64, TimerReadError>
	{
		use self::TimerReadError::*;

		let mut value: u64 = unsafe { uninitialized() };

		const SizeOfRead: usize = size_of::<u64>();

		let result = unsafe { read(self.0, &mut value as *mut _ as *mut _, SizeOfRead) };

		if likely!(result == SizeOfRead as isize)
		{
			Ok(value)
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

	/// Disarms the timer and returns the old value of it.
	#[inline(always)]
	pub fn disarm(&self) -> itimerspec
	{
		static Disarm: itimerspec = itimerspec
		{
			it_interval: timespec
			{
				tv_sec: 0,
				tv_nsec: 0,
			},
			it_value: timespec
			{
				tv_sec: 0,
				tv_nsec: 0,
			},
		};
		self.set(&Disarm, TimerSetChoices::Relative)
	}

	/// Arms the timer to go off once and returns the old value of it.
	#[inline(always)]
	pub fn arm_as_one_off(&self, alarm_goes_off_at: &timespec, interpretation_of_new_value: TimerSetChoices) -> itimerspec
	{
		debug_assert_ne!((alarm_goes_off_at.tv_sec == 0 && alarm_goes_off_at.tv_nsec == 0), true, "alarm_goes_off_at.tv_sec and alarm_goes_off_at.tv_nsec can not both be zero");

		self.set
		(
			&itimerspec
			{
				it_interval: timespec
				{
					tv_sec: 0,
					tv_nsec: 0,
				},
				it_value: timespec
				{
					tv_sec: alarm_goes_off_at.tv_sec,
					tv_nsec: alarm_goes_off_at.tv_nsec,
				},
			},
			interpretation_of_new_value
		)
	}

	/// Arms the timer to go off once and returns the old value of it.
	#[inline(always)]
	pub fn arm_to_go_off_repeatedly(&self, alarm_goes_off_at_repeatedly: &timespec, interpretation_of_new_value: TimerSetChoices) -> itimerspec
	{
		debug_assert_ne!((alarm_goes_off_at_repeatedly.tv_sec == 0 && alarm_goes_off_at_repeatedly.tv_nsec == 0), true, "alarm_goes_off_at_repeatedly.tv_sec and alarm_goes_off_at_repeatedly.tv_nsec can not both be zero");

		self.set
		(
			&itimerspec
			{
				it_interval: timespec
				{
					tv_sec: alarm_goes_off_at_repeatedly.tv_sec,
					tv_nsec: alarm_goes_off_at_repeatedly.tv_nsec,
				},
				it_value: timespec
				{
					tv_sec: alarm_goes_off_at_repeatedly.tv_sec,
					tv_nsec: alarm_goes_off_at_repeatedly.tv_nsec,
				},
			},
			interpretation_of_new_value
		)
	}

	/// Arms or disarms the timer.
	///
	/// Set both fields of `new_value.it_value to disarm the timer`.
	///
	/// If both fields of `new_value.it_interval` are zero, the timer expires just once, at the time specified by `new_value.it_value`.
	///
	/// Returns the previous value of the timer.
	#[inline(always)]
	fn set(&self, new_value: &itimerspec, interpretation_of_new_value: TimerSetChoices) -> itimerspec
	{
		debug_assert!(new_value.it_interval.tv_nsec <= 999_999_999, "new_value.it_interval must not exceed 999,999,999");
		debug_assert!(new_value.it_value.tv_nsec <= 999_999_999, "new_value.it_value must not exceed 999,999,999");

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