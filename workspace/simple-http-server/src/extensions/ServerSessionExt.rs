// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


trait ServerSessionExt
{
	/// Returns true if can not continue with reading.
	fn process_write_read(&mut self, socket: &mut TcpStream, try_to_read_until_there_is_some_plain_text: bool) -> Result<(), Result<RegistrationState, ServerSessionProcessWriteReadError>>;

	#[doc(hidden)]
	#[inline(always)]
	fn would_block(&self, error: io::Error, wants_more_plain_text: bool, f: impl FnOnce(io::Error) -> ServerSessionProcessWriteReadError) -> Result<(), Result<RegistrationState, ServerSessionProcessWriteReadError>>
	{
		if error.kind() == WouldBlock
		{
			Err(Ok(self.registration_state(wants_more_plain_text)))
		}
		else
		{
			Err(Err(f(error)))
		}
	}

	#[inline(always)]
	fn registration_state(&self, wants_more_plain_text: bool) -> RegistrationState
	{
		RegistrationState(self.readiness(wants_more_plain_text), PollOpt::level())
	}

	#[doc(hidden)]
	fn readiness(&self, wants_more_plain_text: bool) -> Ready;

	#[doc(hidden)]
	fn process_read_after_handshaking(&mut self, socket: &mut TcpStream, complete_handshaking: bool) -> Result<(), Result<RegistrationState, ServerSessionProcessWriteReadError>>;

	#[doc(hidden)]
	fn complete_handshaking(&mut self, socket: &mut TcpStream) -> Result<(), Result<RegistrationState, InputOutputError>>;
}

impl ServerSessionExt for ServerSession
{
	fn process_write_read(&mut self, socket: &mut TcpStream, try_to_read_until_there_is_some_plain_text: bool) -> Result<(), Result<RegistrationState, ServerSessionProcessWriteReadError>>
	{
		use self::ServerSessionProcessWriteReadError::*;

		self.complete_handshaking(socket)?;

		// NOTE: This is equivalent to a do-while loop.
		while
		{
			while self.wants_write()
			{
				match self.writev_tls(WriteVAdapter(socket))
				{
					Err(io_error) => return self.would_block(io_error, try_to_read_until_there_is_some_plain_text, SocketVectoredWrite),
					Ok(_bytes_written) => (),
				}
			}

			// NOTE: Opportunistic read, required to handle TLS messages that do not change the size of the plain text buffer.
			self.process_read_after_handshaking(try_to_read_until_there_is_some_plain_text)?;

			if try_to_read_until_there_is_some_plain_text
			{
				// NOTE: `wants_read()` will be ***false*** when there is at least one byte of plain text to make use of.
				while self.wants_read()
				{
					self.process_read_after_handshaking(try_to_read_until_there_is_some_plain_text)?
				}
			}

			self.wants_write()
		}
		{
		}

		Ok(())
	}

	fn readiness(&self, wants_more_plain_text: bool) -> Ready
	{
		let wants_read = self.wants_read();
		let wants_write = self.wants_write();

		let mut readiness = Ready::empty();

		if wants_more_plain_text || self.wants_read()
		{
			readiness |= Ready::readable();
		}

		if self.wants_write()
		{
			readiness |= Ready::writable();
		}

		debug_assert_ne!(readiness, Ready::empty(), "Session wants neither read nor write");

		readiness | UnixReady::error() | UnixReady::hup()
	}

	#[inline(always)]
	fn process_read_after_handshaking(&mut self, socket: &mut TcpStream, wants_more_plain_text: bool) -> Result<(), Result<RegistrationState, ServerSessionProcessWriteReadError>>
	{
		match self.read_tls(socket)
		{
			Err(io_error) => self.would_block(io_error, wants_more_plain_text, ServerSessionProcessWriteReadError::SocketRead),

			Ok(0) => return if wants_more_plain_text
			{
				Err(EndOfFile)
			}
			else
			{
				Ok(())
			},

			Ok(_bytes_read) => if let Err(tls_error) = self.process_new_packets()
			{
				// In case we have an alert to send describing this error try a last-gasp write.
				let io_error_including_would_block = self.writev_tls(WriteVAdapter(socket)).err();

				Err(ProcessNewPackets(error, io_error_including_would_block))
			}
			else
			{
				Ok(())
			}
		}
	}

	fn complete_handshaking(&mut self, socket: &mut TcpStream) -> Result<(), Result<RegistrationState, ServerSessionProcessWriteReadError>>
	{
		use self::ServerSessionProcessWriteReadError::*;

		let mut end_of_file = false;
		while self.is_handshaking()
		{
			if end_of_file
			{
				return Err(Err(EndOfFileWhilstHandshaking))
			}

			while self.wants_write()
			{
				if let Err(io_error) = self.writev_tls(WriteVAdapter(socket))
				{
					return self.would_block(io_error, false, SocketVectoredWrite)
				}
			}

			if self.wants_read()
			{
				match self.read_tls(socket)
				{
					Err(io_error) => return self.would_block(io_error, false, SocketRead),

					Ok(0) => end_of_file = true,

					Ok(_bytes_read) => (),
				}
			}

			if let Err(tls_error) = self.process_new_packets()
			{
				// In case we have an alert to send describing this error try a last-gasp write.
				let io_error_including_would_block = self.writev_tls(WriteVAdapter(socket)).err();

				return Err(Err(ProcessNewPackets(error, io_error_including_would_block)));
			}
		}

		Ok(())
	}
}
