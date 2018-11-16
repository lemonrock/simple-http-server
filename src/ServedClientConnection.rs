// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


struct ServedClientConnection
{
	server_session: ServerSession,
	socket: TcpStream,
	remote_address: SocketAddr,
	read_buffer: Vec<u8>,
}

impl ServedClientConnection
{
	/// NOTE: We do not send CloseNotify using this technique; this is only a security vulnerability in some old protocols (eg HTTP/0.9) and only an issue if using STARTTLS-like protocols (which HTTP isn't).
	#[inline(always)]
	const fn tear_down(&mut self) -> bool
	{
		true
	}

	pub(crate) fn do_tls_read(&mut self, client_token: Token, poll: &Poll, constraints: &Constraints) -> bool
	{
		#[inline(always)]
		const fn is_end_of_file(bytes_read: usize) -> bool
		{
			bytes_read == 0
		}

		match self.server_session.read_tls(&mut self.socket)
		{
			Err(_) => self.tear_down(),

			Ok(bytes_read) => if is_end_of_file(bytes_read)
			{
				self.tear_down()
			}
			else
			{
				if self.server_session.process_new_packets().is_err()
				{
					return self.tear_down()
				}

				match self.server_session.read_to_end(&mut self.read_buffer)
				{
					Err(_) => self.tear_down(),

					Ok(bytes_read) =>
					{
						if self.server_session.

							// send_close_notify
							// is_handshaking
							// get_peer_certificates
							// get_alpn_protocol
							// ?get_protocol_version

						// NOTE: It is not possible to prevent a reallocation (unless we fork rustls) or size to be exceeded.
						if constraints.read_buffer_length_exceed(&self.read_buffer)
						{
							return self.tear_down()
						}

						let mut headers = constraints.header_buffer();
						match Request::parse(&self.read_buffer, &mut headers)
						{
							Err(_) => return self.tear_down(),

							Ok(Incomplete) =>
							{
								// Read more and parse again.
								XXXXXXX
							}

							Ok(Complete((ref request, _length))) =>
							{
								if request.minor_version_is_invalid() || request.method_is_other_than_get()
								{
									return self.tear_down()
								}

								// validate the hostname matches the 'Host:' header if present (mandatory is HTTP/1.1)
								get_sni_hostname(&self)



								match request.target
								{
									// Validate incoming certificate.

									"/libertine-linux.vmlinuz" => XXX (),

									"/libertine-linux.vmlinuz.sig" => XXX (),

									_ => return self.tear_down(),
								}
							}
						}

						XXXXXXXXXXXX  poll.reregister(&self.socket, client_token, self.server_session.readiness(), self.server_session.poll_opt())
					}
				}
			},
		}
	}

	pub(crate) fn do_tls_write(&mut self) -> bool
	{
		if self.server_session.writev_tls(&mut WriteVAdapter::new(&mut self.socket)).is_err()
		{
			return self.tear_down()
		}
		else
		{
			XXXXXXXXXXXX  poll.reregister(&self.socket, client_token, self.server_session.readiness(), self.server_session.poll_opt())
			false
		}
	}
}
