// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// One of three possible server listener socket file descriptors.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServerListenerSocketFileDescriptorEnum
{
	/// An Internet Protocol (IP) version 4 listening socket.
	InternetProtocolVersion4(ServerListenerSocketFileDescriptor<sockaddr_in>),

	/// An Internet Protocol (IP) version 6 listening socket.
	InternetProtocolVersion6(ServerListenerSocketFileDescriptor<sockaddr_in6>),

	/// An Unix Domain listening socket.
	UnixDomain(ServerListenerSocketFileDescriptor<sockaddr_un>),
}

impl AsRawFd for ServerListenerSocketFileDescriptorEnum
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		use self::ServerListenerSocketFileDescriptorEnum::*;

		match self
		{
			&InternetProtocolVersion4(ref server_listener_socket_file_descriptor) => server_listener_socket_file_descriptor.as_raw_fd(),
			&InternetProtocolVersion6(ref server_listener_socket_file_descriptor) => server_listener_socket_file_descriptor.as_raw_fd(),
			&UnixDomain(ref server_listener_socket_file_descriptor) => server_listener_socket_file_descriptor.as_raw_fd(),
		}
	}
}

impl IntoRawFd for ServerListenerSocketFileDescriptorEnum
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		use self::ServerListenerSocketFileDescriptorEnum::*;

		match self
		{
			InternetProtocolVersion4(server_listener_socket_file_descriptor) => server_listener_socket_file_descriptor.into_raw_fd(),
			InternetProtocolVersion6(server_listener_socket_file_descriptor) => server_listener_socket_file_descriptor.into_raw_fd(),
			UnixDomain(server_listener_socket_file_descriptor) => server_listener_socket_file_descriptor.into_raw_fd(),
		}
	}
}

impl ServerListenerSocketFileDescriptorEnum
{
	/// Accepts a pending connections.
	#[inline(always)]
	pub fn accept(&self) -> Result<AcceptedConnectionEnum, SocketAcceptError>
	{
		use self::ServerListenerSocketFileDescriptorEnum::*;

		Ok
		(
			match self
			{
				InternetProtocolVersion4(server_listener_socket_file_descriptor) => AcceptedConnectionEnum::InternetProtocolVersion4(server_listener_socket_file_descriptor.accept()?),
				InternetProtocolVersion6(server_listener_socket_file_descriptor) => AcceptedConnectionEnum::InternetProtocolVersion6(server_listener_socket_file_descriptor.accept()?),
				UnixDomain(server_listener_socket_file_descriptor) => AcceptedConnectionEnum::UnixDomain(server_listener_socket_file_descriptor.accept()?),
			}
		)
	}
}
