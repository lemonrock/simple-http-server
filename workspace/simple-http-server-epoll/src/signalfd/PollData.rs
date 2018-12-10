// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Contains data relevant to the `SIGIO` signal (also known as the `SIGPOLL` signal).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PollData
{
	/// Equivalent to the `revents` field in the struct `pollfd` (see `man 2 poll`).
	///
	/// Also known as `ssi_band` and `si_band`.
	pub revents: u32,

	/// The file descriptor for which the events are appropriate.
	pub file_descriptor: RawFd,
}

impl PollData
{
	#[inline(always)]
	pub(crate) fn new(ssi: &signalfd_siginfo) -> Self
	{
		Self
		{
			revents: ssi.ssi_band,
			file_descriptor: ssi.ssi_fd as RawFd,
		}
	}
}
