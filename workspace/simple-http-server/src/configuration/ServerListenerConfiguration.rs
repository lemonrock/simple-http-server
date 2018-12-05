// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A server listener listens for, and accepts, incoming client connections.
#[derive(Debug)]
pub struct ServerListenerConfiguration<'socket_address, SCCUF: ServedClientConnectionsUserFactory>
{
	/// A string of the form `"127.0.0.1:443"`.
	pub socket_address: &'socket_address str,

	/// Constraints for the server listener, such as maximum buffer sizes and maximum numbers of connections, as well as which IP addresses are allowed.
	pub server_listener_constraints: ServerListenerConstraints,
}

impl<'socket_address, SCCUF: ServedClientConnectionsUserFactory> ServerListenerConfiguration<'socket_address, SCCUF>
{
	#[inline(always)]
	pub(crate) fn new_tcp_listener(&self) -> Result<TcpListener, MainLoopError>
	{
		use self::MainLoopError::*;

		let socket_address = socket_address.parse().map_err(|error| CouldNotParseTcpListenerSocketAddress(error))?;

		TcpListener::bind(&socket_address).map_err(|error| CouldNotBindTcpListener(error))
	}

	#[inline(always)]
	pub(crate) fn maximum_connections(&self) -> usize
	{
		self.server_listener_constraints.maximum_connections
	}

	#[inline(always)]
	pub(crate) fn receive_buffer_size(&self) -> usize
	{
		self.server_listener_constraints.receive_buffer_size
	}

	#[inline(always)]
	pub(crate) fn send_buffer_size(&self) -> usize
	{
		self.server_listener_constraints.send_buffer_size
	}

	#[inline(always)]
	pub(crate) fn new_connection_observer(self, served_client_connection_user_factory: SCCUF) -> Arc<ConnectionObserver<SCCUF>>
	{
		let maximum_connections = self.maximum_connections();

		let server_listener_constraints = self.server_listener_constraints;

		Arc::new(ConnectionObserver::new(self.maximum_connections(), &server_listener_constraints.permitted_internet_protocol_version_4_addresses, &server_listener_constraints.permitted_internet_protocol_version_6_addresses, served_client_connection_user_factory))
	}
}
