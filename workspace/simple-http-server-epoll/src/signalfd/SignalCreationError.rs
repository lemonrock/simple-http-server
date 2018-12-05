// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An error that can occur during creation of a signal instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SignalCreationError
{
	/// The per-process limit on the number of open file descriptors would be exceeded
	PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,

	/// The system-wide limit on the total number of open files would be exceeded.
	SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,

	/// Kernel would be out of memory.
	KernelWouldBeOutOfMemory,
}

impl Display for SignalCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<SignalCreationError as Debug>::fmt(self, f)
	}
}

impl error::Error for SignalCreationError
{
}