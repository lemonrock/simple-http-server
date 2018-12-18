// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a socket address that can be used as a server listener or as a client, for streams and data grams.
pub enum SocketAddress<FilePath: AsRef<Path>>
{
	/// An Internet Protocol (IP) version 4 or 6 socket.
	InternetProtocol(SocketAddr),

	/// An Unix Domain Socket, either as a file or as an abstract name.
	Unix(UnixSocketAddress),
}


impl<FilePath: AsRef<Path>> SocketAddress<FilePath>
{
	/// New streaming server listener.
	///
	/// `back_log` can not exceed `::std::i32::MAX` and is capped by the Operating System to the value in `/proc/sys/net/core/somaxconn`.
	///
	/// The default value in `/proc/sys/net/core/somaxconn` is `128`.
	///
	/// `back_log` is ignored for Unix domain sockets.
	#[inline(always)]
	pub fn new_streaming_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, back_log: u32) -> Result<ServerListenerSocketFileDescriptorEnum, NewSocketServerListenerError>
	{
		use self::ServerListenerSocketFileDescriptorEnum::*;
		use self::SocketAddr::*;
		use self::SocketAddress::*;

		Ok
		(
			match self
			{
				&InternetProtocol(V4(socket_address)) => InternetProtocolVersion4(SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_4_server_listener(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits, back_log)?),
				&InternetProtocol(V6(socket_address)) => InternetProtocolVersion6(SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_6_server_listener(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits, back_log)?),
				&Unix(ref unix_socket_address) => UnixDomain(SocketFileDescriptor::new_streaming_unix_domain_socket_server_listener(unix_socket_address, send_buffer_size_in_bytes)?),
			}
		)
	}

	/// New streaming client.
	#[inline(always)]
	pub fn new_streaming_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16) -> Result<(), NewSocketClientError>
	{
		use self::SocketAddr::*;
		use self::SocketAddress::*;

		match self
		{
			&InternetProtocol(V4(socket_address)) => SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_4_client(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits),
			&InternetProtocol(V6(socket_address)) => SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_6_client(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits),
			&Unix(ref unix_socket_address) => SocketFileDescriptor::new_streaming_unix_domain_socket_client(unix_socket_address, send_buffer_size_in_bytes),
		}
	}

	/// New datagram server listener.
	#[inline(always)]
	pub fn new_datagram_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<(), NewSocketServerListenerError>
	{
		use self::SocketAddr::*;
		use self::SocketAddress::*;

		match self
		{
			&InternetProtocol(V4(socket_address)) => SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_version_4_server_listener(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes),
			&InternetProtocol(V6(socket_address)) => SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_version_6_server_listener(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes),
			&Unix(ref unix_socket_address) => SocketFileDescriptor::new_datagram_unix_domain_socket_server_listener(unix_socket_address, send_buffer_size_in_bytes),
		}
	}

	/// New datagram client.
	#[inline(always)]
	pub fn new_datagram_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<(), NewSocketClientError>
	{
		use self::SocketAddr::*;
		use self::SocketAddress::*;

		match self
		{
			&InternetProtocol(V4(socket_address)) => SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_version_4_client(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes),
			&InternetProtocol(V6(socket_address)) => SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_version_6_client(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes),
			&Unix(ref unix_socket_address) => SocketFileDescriptor::new_datagram_unix_domain_socket_client(unix_socket_address, send_buffer_size_in_bytes),
		}
	}
}
