// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


struct ServedClientConnection<SCCU: ServedClientConnectionUser>
{
	server_session: ServerSession,
	socket: Option<TcpStream>,
	last_registration_state: RegistrationState,
	served_client_connection_user: SCCU,
}

impl<SCCU: ServedClientConnectionUser> Drop for ServedClientConnection<SCCU>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some(socket) = self.socket.take()
		{
			socket.shutdown(Both);
		}
	}
}

impl<SCCU: ServedClientConnectionUser> ServedClientConnection<SCCU>
{
	#[inline(always)]
	pub(crate) fn new(server_session: ServerSession, socket: TcpStream, registration_state: RegistrationState, served_client_connection_user: SCCU) -> Self
	{
		Self
		{
			server_session,
			socket: Some(socket),
			last_registration_state: (Ready::empty(), PollOpt::level()),
			served_client_connection_user,
		}
	}

	#[inline(always)]
	pub(crate) fn initialize_registration_state(&mut self, registration_state: RegistrationState)
	{
		self.last_registration_state = registration_state
	}

	#[inline(always)]
	pub(crate) fn service(&mut self) -> Result<RegistrationState, ServerSessionProcessWriteReadError<SCCU::Error>>
	{
		let result = served_client_connection_user.service(SimplifiedServerSession(&self.server_session, self.socket.as_mut().unwrap()));
		match result
		{
			Ok(()) => Ok(self.server_session.registration_state(false)),
			Err(result) => match result
			{
				Ok(registration_state) => Ok(registration_state),
				Err(sccu_error) => Err(ServerSessionProcessWriteReadError::ServedClientConnectionUser(sccu_error)),
			}
		}
	}

	#[inline(always)]
	pub(crate) fn reregister(&mut self, poll: &Poll, client_token: Token, next_registration_state: RegistrationState) -> io::Result<()>
	{
		if self.last_registration_state != next_registration_state
		{
			poll.reregister(self.socket.as_ref().unwrap(), client_token, next_registration_state.0, next_registration_state.1)?;
			self.last_registration_state = next_registration_state;
		}

		Ok(())
	}

}
