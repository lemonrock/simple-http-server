// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ServedClientConnectionToken<SCCUF: ServedClientConnectionUserFactory>
{
	socket: TcpStream,
	served_client_connection_user: SCCUF::SCCU,
	last_registration_state: RegistrationState,
	connection_observer: Arc<ConnectionObserver<SCCUF>>,
}

impl<SCCUF: ServedClientConnectionUserFactory> Drop for ServedClientConnectionToken<SCCUF>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.connection_observer.disconnect(self.socket.peer_addr().expect("Could not obtain peer address from socket"));
		self.socket.shutdown(Both);
	}
}

impl<SCCUF: ServedClientConnectionUserFactory> ServedClientConnectionToken<SCCUF>
{
	const Drop: bool = true;

	const DoNotDrop: bool = false;

	pub(crate) fn new(arena: &Arena<Self>, socket: TcpStream, served_client_connection_user: SCCUF::SCCU, connection_observer: &Arc<ConnectionObserver<SCCUF>>) -> Result<Token, ()>
	{
		let arena_item = arena.allocate().ok_or(())?;

		arena_item.set_inner_value
		(
			Self
			{
				socket,
				served_client_connection_user,
				last_registration_state: (Ready::empty(), PollOpt::level()),
				connection_observer: connection_observer.clone(),
			}
		);

		Ok(TokenKind::ServedClientConnection.into_token(arena_item))
	}

	#[inline(always)]
	pub(crate) fn handle_event(&mut self, poll: &Poll, readiness: Ready, our_token: Token) -> bool
	{
		{
			let unix_readiness = UnixReady::from(readiness);

			// HUP is not easy to make use of when using TLS atop of a regular stream.
			if unix_readiness.is_hup() || unix_readiness.is_error()
			{
				return Self::Drop
			}
		}

		self.subsequent_service(poll, our_token)
	}

	pub(crate) fn first_service(&mut self, poll: &Poll, our_token: Token) -> bool
	{
		match self.served_client_connection_user.service(&self.socket).map_err()
		{
			Err(_) => Self::Drop,

			Ok(next_registration_state) => if next_registration_state.is_empty()
			{
				Self::Drop
			}
			else
			{
				self.last_registration_state = next_registration_state;
				if poll.register(&self.socket, our_token, next_registration_state.0, next_registration_state.1).is_err()
				{
					Self::Drop
				}
				else
				{
					Self::DoNotDrop
				}
			}
		}
	}

	pub(crate) fn subsequent_service(&mut self, poll: &Poll, our_token: Token) -> bool
	{
		match self.served_client_connection_user.service(&self.socket).map_err()
		{
			Err(_) => Self::Drop,

			Ok(next_registration_state) => if next_registration_state.is_empty()
			{
				Self::Drop
			}
			else if next_registration_state != self.last_registration_state
			{
				self.last_registration_state = next_registration_state;
				if poll.reregister(&self.socket, our_token, next_registration_state.0, next_registration_state.1).is_err()
				{
					Self::Drop
				}
				else
				{
					Self::DoNotDrop
				}
			}
			else
			{
				Self::DoNotDrop
			}
		}
	}
}
