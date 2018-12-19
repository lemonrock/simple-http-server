// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a datagram client socket instance between two peers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatagramClientSocketFileDescriptorEnum
{
	/// An Internet Protocol (IP) version 4 streaming socket.
	InternetProtocolVersion4(DatagramClientSocketFileDescriptor<sockaddr_in>),

	/// An Internet Protocol (IP) version 6 streaming socket.
	InternetProtocolVersion6(DatagramClientSocketFileDescriptor<sockaddr_in6>),

	/// An Unix Domain streaming socket.
	UnixDomain(DatagramClientSocketFileDescriptor<sockaddr_un>),
}

impl AsRawFd for DatagramClientSocketFileDescriptorEnum
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		use self::DatagramClientSocketFileDescriptorEnum::*;

		match self
		{
			&InternetProtocolVersion4(ref datagram_client_socket_file_descriptor) => datagram_client_socket_file_descriptor.as_raw_fd(),
			&InternetProtocolVersion6(ref datagram_client_socket_file_descriptor) => datagram_client_socket_file_descriptor.as_raw_fd(),
			&UnixDomain(ref datagram_client_socket_file_descriptor) => datagram_client_socket_file_descriptor.as_raw_fd(),
		}
	}
}
