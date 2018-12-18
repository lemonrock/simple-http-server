// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An error that can occur during binding of a socket instance.
#[derive(Debug)]
pub enum SocketBindError
{
	/// Permission denied.
	///
	/// For an Unix Domain Socket, some part of the path not might searchable.
	PermissionDenied,

	/// The address is already in use.
	AddressInUse,

	/// Kernel would be out of memory.
	KernelWouldBeOutOfMemory,

	/// Specifically, for an Unix Domain Socket, additional failures are possible.
	FilePathInvalid(FilePathInvalidReason)
}

impl Display for SocketBindError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<SocketBindError as Debug>::fmt(self, f)
	}
}

impl error::Error for SocketBindError
{
}
