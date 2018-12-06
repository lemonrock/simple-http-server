// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a socket instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketFileDescriptor(RawFd);

impl Drop for SocketFileDescriptor
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

impl SocketFileDescriptor
{
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 4.
	#[inline(always)]
	pub fn new_transmission_control_protocol_over_internet_protocol_version_4() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_INET, SOCK_STREAM)
	}

	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 6.
	#[inline(always)]
	pub fn new_transmission_control_protocol_over_internet_protocol_version_6() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_INET6, SOCK_STREAM)
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4.
	#[inline(always)]
	pub fn new_user_datagram_protocol_over_internet_protocol_version_4() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_INET, SOCK_DGRAM)
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 6.
	#[inline(always)]
	pub fn new_user_datagram_protocol_over_internet_protocol_version_6() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_INET6, SOCK_DGRAM)
	}

	/// Creates a new instance.
	#[inline(always)]
	fn new(domain: c_int, type_: c_int) -> Result<Self, SocketCreationError>
	{
		const Flags: c_int = SOCK_NONBLOCK | SOCK_CLOEXEC;
		const Protocol: c_int = 0;

		let result = unsafe { socket(domain, type_ | Flags, Protocol) };
		if likely!(result != -1)
		{
			Ok(SocketFileDescriptor(result))
		}
		else
		{
			use self::SocketCreationError::*;

			Err
			(
				match errno().0
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOBUFS | ENOMEM => KernelWouldBeOutOfMemory,
					EINVAL => panic!("Invalid arguments"),
					EACCES => panic!("Permission denined"),
					EAFNOSUPPORT => panic!("The implementation does not support the specified address family"),
					EPROTONOSUPPORT => panic!("The protocol type or the specified protocol is not supported within this domain"),
					_ => unreachable!(),
				}
			)
		}
	}
}
