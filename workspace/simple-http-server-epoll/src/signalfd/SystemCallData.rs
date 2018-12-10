// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Contains data relevant to the `SIGSYS` signal (also known as the `SIGUNKNOWN` signal).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemCallData
{
	/// The system call number.
	pub system_call_number: i32,

	/// The address of the fault.
	pub address: u64,

	/// The system call architecture.
	///
	/// Where not supported the value is zero.
	pub architecture: u32,
}

impl SystemCallData
{
	#[inline(always)]
	pub(crate) fn new(ssi: &signalfd_siginfo) -> Self
	{
		Self
		{
			system_call_number: ssi.ssi_syscall,
			address: ssi.ssi_call_addr,
			architecture: ssi.ssi_arch,
		}
	}
}
