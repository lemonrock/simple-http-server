// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Constraints to prevent over-use of server resources.
///
/// Note that `rlimit` may still need to be set, particularly on Linux systems, to handle more than about 1020 connections.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServerListenerConstraints
{
	/// Maximum number of served client connections that can be simultaneously open.
	///
	/// Defaults to 4,096.
	pub maximum_connections: usize,

	/// Inbound Internet Protocol Version 4 addresses to permit.
	///
	/// If left empty, nothing will match and all inbound Internet Protocol Version 4 connections will be denied.
	pub permitted_internet_protocol_version_4_addresses: HashSet<PermittedInternetProtocolVersionAddresses<Ipv4Addr>>,

	/// Inbound Internet Protocol Version 6 addresses to permit.
	///
	/// If left empty, nothing will match and all inbound Internet Protocol Version 6 connections will be denied.
	pub permitted_internet_protocol_version_6_addresses: HashSet<PermittedInternetProtocolVersionAddresses<Ipv6Addr>>,

	/// Receive buffer size, in bytes.
	///
	/// Defaults to 16,384 bytes (16Kb)
	pub receive_buffer_size: usize,

	/// Send buffer size, in bytes.
	///
	/// Defaults to 16,384 bytes (16Kb).
	pub send_buffer_size: usize,



	/// Buffer limit, in bytes, passed to rustls; controls internal write buffers and unread plain text buffers.
	///
	/// Zero (0) is interpreted as infinite.
	///
	/// Defaults to 16,384 bytes (16Kb).
	pub rustls_buffer_limit: usize,
}

impl Default for ServerListenerConstraints
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			maximum_connections: 4096,
			permitted_internet_protocol_version_4_addresses: HashSet::default(),
			permitted_internet_protocol_version_6_addresses: HashSet::default(),
			receive_buffer_size: 16_384,
			send_buffer_size: 16_384,

			rustls_buffer_limit: 16_384,
		}
	}
}

impl ServerListenerConstraints
{
//	fn new_server_session(&self) -> ServerSession
//	{
//		let mut server_session = ServerSession::new(&self.server_configuration);
//		self.constraints.set_rustls_buffer_limit(&mut server_session);
//		server_session
//	}


	#[inline(always)]
	pub(crate) fn set_rustls_buffer_limit(&self, server_session: &mut ServerSession) -> Events
	{
		server_session.set_buffer_limit(self.rustls_buffer_limit)
	}
}
