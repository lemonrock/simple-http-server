// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ServerListenerToken<SCCUF: ServedClientConnectionUserFactory>
{
	tcp_listener: TcpListener,
	receive_buffer_size: usize,
	send_buffer_size: usize,
	connection_observer: Arc<ConnectionObserver<SCCUF>>,
}

impl<SCCUF: ServedClientConnectionUserFactory> ServerListenerToken<SCCUF>
{
	pub(crate) fn new<'socket_address>(poll: &Poll, server_listener_configuration: ServerListenerConfiguration<'socket_address, SCCUF>, served_client_connection_user_factory: SCCUF) -> Result<(), MainLoopError>
	{
		let mut this = Box::new
		(
			Self
			{
				tcp_listener: server_listener_configuration.new_tcp_listener()?,
				receive_buffer_size: server_listener_configuration.receive_buffer_size(),
				send_buffer_size: server_listener_configuration.send_buffer_size(),
				connection_observer: server_listener_configuration.new_connection_observer(served_client_connection_user_factory),
			}
		);

		let tcp_listener = &this.tcp_listener as *const TcpListener;

		let token = TokenKind::ServedClientConnection.into_token_from_box(this);

		match poll.register(unsafe { & * tcp_listener }, token, Ready::readable(), PollOpt::edge())
		{
			Ok(()) => Ok(token),

			Err(error) =>
			{
				drop(token.as_box::<Self>());
				Err(MainLoopError::CouldNotRegisterTcpListenerWithPoll(error))
			}
		}
	}

	#[inline(always)]
	pub(crate) fn handle_event(&mut self, poll: &Poll) -> bool
	{
		if let Some(connection) = self.tcp_listener.accept()
		{
			self.new_served_client_connection(connection);
		}

		false
	}

	#[inline(always)]
	fn new_served_client_connection(&mut self, (socket, remote_address): (TcpStream, SocketAddr)) -> Result<(), NewServerClientConnectionError<SCCUF>>
	{
		use self::NewServerClientConnectionError::*;

		Self::prepare_socket(socket)?;

		let served_client_connection_user = match self.connection_observer.connect(remote_address)
		{
			Err(error) => return Self::shutdown_socket_ignore_error(socket, CouldNotCreateNewServedClientConnectionUser(error)),
			Ok(served_client_connection_user) => served_client_connection_user,
		};

		let client_token = ServedClientConnectionToken::new(socket, served_client_connection_user, &self.connection_observer).ok_or(CouldNotAllocateMemory)?;

		let drop = self.first_service(poll, client_token);

		if unlikely!(drop)
		{
			Err(FailedOnFirstUse)
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn prepare_socket(&self, socket: TcpStream) -> Result<(), NewServerClientConnectionError>
	{
		use self::NewServerClientConnectionError::*;

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

		if let Err(error) = socket.set_recv_buffer_size(self.receive_buffer_size)
		{
			return Self::shutdown_socket_ignore_error(socket, ReceiveBufferSize(error))
		}

		if let Err(error) = socket.set_send_buffer_size(self.send_buffer_size)
		{
			return Self::shutdown_socket_ignore_error(socket, SendBufferSize(error))
		}

		Ok(())
	}

	#[inline(always)]
	fn shutdown_socket_ignore_error(socket: TcpStream, error: NewServerClientConnectionError) -> Result<(), NewServerClientConnectionError>
	{
		socket.shutdown(Both);
		Err(error)
	}
}
