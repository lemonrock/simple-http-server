// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


struct ServedClientConnections<'a>
{
	server_configuration: Arc<ServerConfig>,
	poll: &'a Poll,
	constraints: &'a Constraints,
	token_store: &'a TokenStore,
	connections: HashMap<Token, RefCell<ServedClientConnection>>,
}

impl<'a> ServedClientConnections<'a>
{
	pub(crate) fn new(server_configuration: ServerConfig, poll: &'a Poll, token_store: &'a TokenStore, constraints: &'a Constraints) -> Self<'a>
	{
		Self
		{
			server_configuration: Arc::new(server_configuration),
			poll,
			token_store,
			constraints,
			connections: HashMap::with_capacity(EventsCapacity)
		}
	}

	// TODO: Embed lemonrock/treebitmap
	// TODO: Use remote address with a set of known good addresses to act as an access control.
	pub(crate) fn new_served_client_connection(&mut self, (socket, remote_address): (TcpStream, SocketAddr)) -> Result<(), NewServerClientConnectionError>
	{
		Self::prepare_socket(socket)?;

		let client_token = self.token_store.next_token();

		let mut server_session = ServerSession::new(&self.server_configuration);
		self.constraints.set_rustls_buffer_limit(&mut server_session);

		if let Err(error) = server_session.register(self.poll, &socket, client_token)
		{
			return Self::shutdown_socket_ignore_error(socket, NewServerClientConnectionError::CouldNotRegisterWithPoll(error))
		}

		let existing = self.connections.insert(client_token, RefCell::new(ServedClientConnection::new(server_session, socket, self.read_buffer())));
		assert_eq!(existing, None, "Wrap around of tokens")
	}

	pub(crate) fn handle_event(&mut self, client_token: Token, readiness: Ready)
	{
		// Spurious wake-up.
		if readiness.is_empty()
		{
			return
		}

		{
			let unix_readiness = UnixReady::from(readiness);

			// HUP is difficult to understand how to respond to when using TLS atop of a regular stream.
			if unix_readiness.is_hup() || unix_readiness.is_error()
			{
				return self.destroy(client_token)
			}
		}

		let served_client_connection = match self.connections.get(&client_token)
		{
			// It's possible that more than one event is generated for the same socket, and the previous event resulted in an error; hence this may be empty.
			None => return,
			Some(served_client_connection) => served_client_connection,
		};

		if readiness.is_readable()
		{
			let destroy = served_client_connection.borrow_mut().read().is_err();
			if destroy
			{
				return self.destroy(client_token)
			}
		}

		if readiness.is_writable()
		{
			let destroy = served_client_connection.borrow_mut().write().is_err();
			if destroy
			{
				return self.destroy(client_token)
			}
		}

		// TODO: Cost of reregister system call can be avoided if we cache previous registration state (unless we use oneshot).
		let destroy = served_client_connection.borrow().reregister(self.poll, client_token).is_err();
		if destroy
		{
			return self.destroy(client_token)
		}
	}

	#[inline(always)]
	fn prepare_socket(&self, socket: TcpStream) -> Result<(), NewServerClientConnectionError>
	{
		use self::NewServerClientConnectionError::*;

		if self.serving_maximum_number_of_connections()
		{
			return match Self::shutdown_socket(socket)
			{
				Ok(()) => Err(ServingMaximumNumberOfConnections(None)),
				Err(error) => Err(ServingMaximumNumberOfConnections(Some(error))),
			}
		}

		if let Err(error) = socket.set_nodelay(true)
		{
			return Self::shutdown_socket_ignore_error(socket, NoDelay(error))
		}

		if let Err(error) = socket.set_keepalive(None)
		{
			return Self::shutdown_socket_ignore_error(socket, KeepAlive(error))
		}

		if let Err(error) = socket.set_linger(None)
		{
			return Self::shutdown_socket_ignore_error(socket, Linger(error))
		}

		if let Error(error) = socket.set_recv_buffer_size(self.constraints.receive_buffer_size)
		{
			return Self::shutdown_socket_ignore_error(socket, ReceiveBufferSize(error))
		}

		if let Error(error) = socket.set_send_buffer_size(self.constraints.send_buffer_size)
		{
			return Self::shutdown_socket_ignore_error(socket, SendBufferSize(error))
		}

		return Ok(())
	}

	#[inline(always)]
	fn destroy(&mut self, client_token: Token)
	{
		drop(self.connections.remove(&client_token))
	}

	#[inline(always)]
	fn serving_maximum_number_of_connections(&self) -> bool
	{
		self.connections.len() == self.constraints.maximum_connections
	}

	#[inline(always)]
	fn shutdown_socket_ignore_error(socket: TcpStream, error: NewServerClientConnectionError) -> Result<(), NewServerClientConnectionError>
	{
		Self::shutdown_socket(socket);
		Err(error)
	}

	#[inline(always)]
	fn shutdown_socket(socket: TcpStream) -> Result<(), io::Error>
	{
		socket.shutdown(Both)
	}

	#[inline(always)]
	fn read_buffer(&self) -> Vec<u8>
	{
		self.constraints.read_buffer()
	}
}
