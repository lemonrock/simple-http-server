// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Contains fault data relevant to certain signals.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaultData
{
	/// The address of the fault.
	pub address: u64,

	/// The trap number of the fault (only supported on the Alpha, MIPS and SPARC architectures; Rust does not support the Alpha architecture).
	///
	/// Where not supported the value is zero.
	pub trap_number: u32,
}

impl FaultData
{
	#[inline(always)]
	pub(crate) fn new(ssi: &signalfd_siginfo) -> Self
	{
		Self
		{
			address: ssi.ssi_addr,
			trap_number: ssi.ssi_trapno,
		}
	}
}
