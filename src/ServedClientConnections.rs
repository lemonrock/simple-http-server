// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


struct ServedClientConnections<'a, SCCUF: ServedClientConnectionUserFactory>
{
	server_configuration: Arc<ServerConfig>,
	poll: &'a Poll,
	constraints: &'a Constraints,
	token_store: &'a TokenStore,
	connections: HashMap<Token, RefCell<ServedClientConnection<SCCUF::SCCU::Error>>>,
	served_client_connection_user_factory: &'a SCCUF,
}

impl<'a, SCCUF: ServedClientConnectionUserFactory> ServedClientConnections<'a, SCCUF>
{
	pub(crate) fn new(server_configuration: ServerConfig, poll: &'a Poll, token_store: &'a TokenStore, constraints: &'a Constraints, served_client_connection_user_factory: &'a SCCUF) -> Self<'a>
	{
		Self
		{
			server_configuration: Arc::new(server_configuration),
			poll,
			token_store,
			constraints,
			connections: HashMap::with_capacity(EventsCapacity),
			served_client_connection_user_factory,
		}
	}

	// TODO: Embed lemonrock/treebitmap
	// TODO: Use remote address with a set of known good addresses to act as an access control.
	pub(crate) fn new_served_client_connection(&mut self, (socket, remote_address): (TcpStream, SocketAddr)) -> Result<(), NewServerClientConnectionError<SCCUF::SCCU::Error>>
	{
		use self::NewServerClientConnectionError::*;

		Self::prepare_socket(socket)?;

		let served_client_connection_user = match self.served_client_connection_user_factory.new(remote_address)
		{
			Err(()) => return Self::shutdown_socket_ignore_error(socket, CouldNotCreateNewServedClientConnectionUser(error)),
			Ok(served_client_connection_user) => served_client_connection_user,
		};

		let server_session = self.new_server_session();

		let served_client_connection = ServedClientConnection::new(server_session, socket, served_client_connection_user);

		let registration_state = match served_client_connection.service()
		{
			Err(server_session_process_write_read_error) => return FailedOnFirstUse(server_session_process_write_read_error),
			Ok(server_session_polling) => server_session_polling.registration_state(),
		};

		let client_token = self.token_store.next_token();
		if let Err(error) = self.poll.register(&socket, client_token, registration_state.0, registration_state.1)
		{
			return Self::shutdown_socket_ignore_error(socket, CouldNotRegisterWithPoll(error))
		}

		served_client_connection.initialize_registration_state(registration_state);

		let existing = self.connections.insert(client_token, RefCell::new(served_client_connection));
		assert_eq!(existing, None, "Wrap around of tokens")
	}

	fn new_server_session(&self) -> ServerSession
	{
		let mut server_session = ServerSession::new(&self.server_configuration);
		self.constraints.set_rustls_buffer_limit(&mut server_session);
		server_session
	}

	pub(crate) fn handle_event(&mut self, client_token: Token, readiness: Ready)
	{
		{
			let unix_readiness = UnixReady::from(readiness);

			// HUP is not easy to make use of when using TLS atop of a regular stream.
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

		let result = served_client_connection.borrow_mut().process_write_read();
		let destroy = match result
		{
			Err(_) => return self.destroy(client_token),
			Ok(server_session_polling) =>
			{
				let next_registration_state = server_session_polling.registration_state();
				served_client_connection.borrow_mut().reregister(self.poll, client_token, next_registration_state).is_err()
			},
		};
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
}
