// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGIO` (also known as `SIGPOLL`) signal.
///
/// Definitions valid as of Linux v4.20-rc5.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum PollCode
{
	/// Data input available.
	///
	/// Known as `POLL_IN` in Linux sources.
	Input = 1,

	/// Data output available.
	///
	/// Known as `POLL_OUT` in Linux sources.
	Output = 2,

	/// Input message available.
	///
	/// Known as `POLL_MSG` in Linux sources.
	InputMessage = 3,

	/// Input / Output (IO) error.
	///
	/// Known as `POLL_IN` in Linux sources.
	IoError = 4,

	/// Priority input available (eg TCP's deprecated out-of-band urgent data).
	///
	/// Known as `POLL_PRI` in Linux sources.
	PriorityInput = 5,

	/// Device disconnected (Hung Up).
	///
	/// Known as `POLL_HUP` in Linux sources.
	HungUp = 6,
}

impl Into<i32> for PollCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for PollCode
{
	type Data = PollData;

	const InclusiveMaximum: Self = PollCode::HungUp;

	#[inline(always)]
	fn convert(code: i32) -> Self
	{
		unsafe { transmute(code) }
	}
}
