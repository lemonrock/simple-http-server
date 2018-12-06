// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a socket address that can be used as a server listener or as a client, for streams and data grams.
pub enum SocketAddress<P: AsRef<Path>>
{
	/// An Internet Protocol (IP) version 4 or 6 socket.
	InternetProtocol
	{
		/// An Internet Protocol (IP) version 4 or 6 socket address.
		socket_address: SocketAddr,
	},

	/// An Unix Domain Socket.
	Unix
	{
		/// File path.
		///
		/// Can not be more than 108 bytes long.
		file_path: P
	}
}

impl<P: AsRef<Path>> SocketAddress<P>
{
	/// New streaming server listener.
	///
	/// `back_log` can not exceed `::std::i32::MAX` and is capped by the Operating System to the value in `/proc/sys/net/core/somaxconn`.
	///
	/// The default value in `/proc/sys/net/core/somaxconn` is `128`.
	#[inline(always)]
	pub fn new_streaming_server_listener(&self, back_log: u32) -> Result<(), NewSocketServerListenerError>
	{
		use self::SocketAddress::*;

		match self
		{
			&InternetProtocol { socket_address } => SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_server_listener(socket_address, back_log),
			&Unix { ref file_path } => SocketFileDescriptor::new_streaming_unix_domain_socket_server_listener(file_path),
		}
	}

	/// New streaming client.
	#[inline(always)]
	pub fn new_streaming_client(&self) -> Result<(), NewSocketClientError>
	{
		use self::SocketAddress::*;

		match self
		{
			&InternetProtocol { socket_address } => SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_client(socket_address),
			&Unix { ref file_path } => SocketFileDescriptor::new_streaming_unix_domain_socket_client(file_path),
		}
	}

	/// New datagram server listener.
	#[inline(always)]
	pub fn new_datagram_server_listener(&self) -> Result<(), NewSocketServerListenerError>
	{
		use self::SocketAddress::*;

		match self
		{
			&InternetProtocol { socket_address } => SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_server_listener(socket_address),
			&Unix { ref file_path } => SocketFileDescriptor::new_datagram_unix_domain_socket_server_listener(file_path),
		}
	}

	/// New datagram client.
	#[inline(always)]
	pub fn new_datagram_client(&self) -> Result<(), NewSocketClientError>
	{
		use self::SocketAddress::*;

		match self
		{
			&InternetProtocol { socket_address } => SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_client(socket_address),
			&Unix { ref file_path } => SocketFileDescriptor::new_datagram_unix_domain_socket_client(file_path),
		}
	}
}
