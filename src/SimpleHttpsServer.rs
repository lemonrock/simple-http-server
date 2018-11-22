// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A simple HTTPS server.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleHttpsServer<'a, SCCUF: ServedClientConnectionsUserFactory>
{
	/// Constraints.
	pub constraints: &'a Constraints,

	/// Essential TLS configuration.
	pub tls_configuration: &'a TlsConfiguration,

	/// Factory.
	pub served_client_connections_user_factory: &'a SCCUF,
}

impl<'a, SCCUF: ServedClientConnectionsUserFactory> SimpleHttpsServer<'a, SCCUF>
{
	/// Main loop; may exit with an error if it can't be created.
	///
	/// * `should_finish` is checked every loop. When it is true, the method will return.
	/// * `socket_address` is a string of the form `"127.0.0.1:443"`.
	pub fn main_loop(&self, should_finish: &AtomicBool, socket_address: &str) -> Result<(), MainLoopError>
	{
		use self::MainLoopError::*;

		let mut server_config = self.server_configuration()?;
		let poll = Self::poll()?;
		let token_store = TokenStore::default();

		let mut served_client_connections = ServedClientConnections::new(server_config, &poll, &token_store, self.constraints, self.served_client_connections_user_factory)?;

		let (server, server_token) = Self::register_server(&token_store, &poll, socket_address)?;

		let mut events = self.events();
		let poll_time_out = self.poll_time_out();

		while !should_finish.get()
		{
			poll.poll(&mut events, poll_time_out).map_err(|error| PollLoop(error))?;

			for event in events.iter()
			{
				let token = event.token();

				if token == server_token
				{
					if let Some(connection) = server.accept()
					{
						served_client_connections.new_served_client_connection(connection);
					}
				}
				else
				{
					served_client_connections.handle_event(token, event.readiness())
				}
			}
		}

		Ok(())
	}

	fn register_server(token_store: &TokenStore, poll: &Poll, socket_address: &str) -> Result<(TcpListener, Token), MainLoopError>
	{
		use self::MainLoopError::*;

		let socket_address = socket_address.parse().map_err(|error| CouldNotParseTcpListenerSocketAddress(error))?;
		let server = TcpListener::bind(&socket_address).map_err(|error| CouldNotBindTcpListener(error))?;
		let server_token = token_store.next_token();
		poll.register(&server, server_token, Ready::readable(), PollOpt::edge()).map_err(|error| CouldNotRegisterTcpListenerWithPoll(error))?;
		Ok((server, server_token))
	}

	#[inline(always)]
	fn server_configuration(&self) -> Result<ServerConfig, MainLoopError>
	{
		self.tls_configuration.server_configuration().map_err(|error| ServerConfiguration(error))
	}

	#[inline(always)]
	fn poll() -> Result<Poll, MainLoopError>
	{
		Poll::new().map_err(|error| PollCreation(error))
	}

	#[inline(always)]
	fn poll_time_out(&self) -> Option<Duration>
	{
		self.constraints.poll_time_out()
	}

	#[inline(always)]
	fn events(&self) -> Events
	{
		self.constraints.events()
	}
}
