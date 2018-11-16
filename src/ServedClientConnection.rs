// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


struct ServedClientConnection
{
	server_session: ServerSession,
	socket: Option<TcpStream>,
	read_buffer: Vec<u8>,
}

impl Drop for ServedClientConnection
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

impl ServedClientConnection
{
	#[inline(always)]
	pub(crate) fn new(server_session: ServerSession, socket: TcpStream, read_buffer: Vec<u8>) -> Self
	{
		Self
		{
			server_session,
			socket: Some(socket),
			read_buffer,
		}
	}

	pub(crate) fn read(&mut self, constraints: &Constraints) -> Result<(), TlsReadError>
	{
		#[inline(always)]
		const fn is_end_of_file(raw_bytes_read_from_socket: usize) -> bool
		{
			raw_bytes_read_from_socket == 0
		}

		use self::TlsReadError::*;

		let raw_bytes_read_from_socket = match self.server_session.read_tls(self.socket.as_mut().unwrap())
		{
			Err(error) => return Self::would_block(error, SocketReadError),

			Ok(raw_bytes_read_from_socket) => raw_bytes_read_from_socket,
		};

		if is_end_of_file(raw_bytes_read_from_socket)
		{
			return Err(EndOfFile)
		}

		self.server_session.process_new_packets().map_err(|error| ProcessNewPacketsError(error))?;

		if self.server_session.is_handshaking()
		{
			return Ok(())
		}

		self.server_session.read_to_end(&mut self.read_buffer).map_err(|error| ReadToEndError(error))?;

		if constraints.read_buffer_length_exceeded(&self.read_buffer)
		{
			return Err(ReadBufferLengthExceeded)
		}

		self.do_http_read(constraints).map_err(|error| HttpViolation(error))
	}

	#[inline(always)]
	pub(crate) fn write(&mut self) -> Result<(), TlsWriteError>
	{
		match self.server_session.writev_tls(&mut WriteVAdapter(self.socket.as_mut().unwrap()))
		{
			Err(error) => Self::would_block(error, TlsWriteError::SocketVectoredWriteError),

			Ok(_bytes_written) => Ok(())
		}
	}

	#[inline(always)]
	pub(crate) fn reregister(&self, poll: &Poll, client_token: Token) -> io::Result<()>
	{
		self.server_session.reregister(poll, self.socket.as_ref().unwrap(), client_token)
	}

	fn do_http_read(&self, constraints: &Constraints, our_hostname: &str, our_port_string: &str) -> Result<(), HttpServerReadError>
	{
		let mut headers = constraints.header_buffer();
		match Request::parse(&self.read_buffer, &mut headers).map_err(|error| HttpHeadersInvalid(error))?
		{
			Ok(Incomplete) =>
			{
				// Read more and parse again.
				XXXXXXX
			}

			Ok(Complete((ref request, _length))) =>
			{
				fn validate_minor_version(request: &Request) -> Result<(), HttpServerReadError>
				{
					if request.minor_version_is_invalid()
					{
						Err(HttpServerReadError::UnsupportedHttpMinorVersion)
					}
					else
					{
						Ok(())
					}
				}

				fn validate_request_method_is_get(request: &Request) -> Result<(), HttpServerReadError>
				{
					if request.method_is_other_than_get()
					{
						Err(HttpServerReadError::UnsupportedHttpMethod)
					}
					else
					{
						Ok(())
					}
				}

				fn validate_alpn_protocol<'buffer>(request: &Request, server_session: &ServerSession) -> Result<(), HttpServerReadError>
				{
					if let Some(alpn_protocol) = server_session.get_alpn_protocol()
					{
						let minor_version_expected = match alpn_protocol
						{
							TlsConfiguration::AlpnProtocolHttp_1_1 => 1,

							TlsConfiguration::AlpnProtocolHttp_1_0 => 0,
						};

						if request.minor_version != minor_version_expected
						{
							return Err(HttpServerReadError::AlpnProtocolMismatchesHttpMinorVersion)
						}
					}
					Ok(())
				}

				fn validate_host_header<'buffer>(request: &Request, headers: &Vec<HeaderField<'buffer>>, our_hostname: &str, our_port_string: &str) -> Result<(), HttpServerReadError>
				{
					use self::HttpServerReadError::*;

					let mut host_header_missing = true;
					for header_field in headers.iter().rev()
					{
						if header_field.name.eq_ignore_ascii_case("Host")
						{
							let iterator = header_field.value.splitn(2, ':');
							let hostname = Some(iterator.next().unwrap());

							if !hostname.eq_ignore_ascii_case(our_hostname)
								{
									return Err(HostHeaderHostnameMismatch)
								}

							if let Some(port_string) = iterator.next()
							{
								if port_string != our_port_string
								{
									return Err(HostHeaderHasIncorrectPort)
								}
							}

							host_header_missing = false
						}
					}

					if host_header_missing && request.minor_version == 1
					{
						return Err(Http11MissingHostHeader)
					}
				}

				fn validate_sni_hostname(server_session: &ServerSession, our_hostname: &str) -> Result<(), HttpServerReadError>
				{
					if let Some(sni_hostname) = server_session.get_sni_hostname()
					{
						if !sni_hostname.eq_ignore_ascii_case(our_hostname)
						{
							return Err(HttpServerReadError::SniHostnameMismatch)
						}
					}

					Ok(())
				}

				fn validate_target_is_absolute(target: &str) -> Result<(), TargetIsNotAbsolute>
				{
					if request.target.starts_with('/')
					{
						Ok(())
					}
					else
					{
						Err(HttpServerReadError::TargetIsNotAbsolute)
					}
				}

				validate_minor_version(&request)?;

				validate_request_method_is_get(&request)?;

				validate_alpn_protocol(&request, &self.server_session)?;

				validate_host_header(&request, &headers, our_hostname, our_port_string)?;

				validate_sni_hostname(&self.server_session, our_hostname)?;

				validate_target_is_absolute(request.target)?;

				let target_uri = Url::parse(&format!("https://{}:{}", our_hostname, our_port_string)).unwrap().join(request.target).map_err(|error| HttpServerReadError::TargetIsInvalidUri(error))?;

				let client_end_entity_certificate = match self.server_session.get_peer_certificates()
				{
					None => None,

					Some(certificates) => if certificates.is_empty()
					{
						None
					}
					else
					{
						let first = unsafe { certificates.get_unchecked(0) };
						Some(EndEntityCert::from(first.der_encoded_certificate()).map_err(HttpServerReadError::EndEntityClientCertificateInvalid)?)
					},
				};

				// TODO: Use https://crates.io/crates/url-match ?
				self.dispatch(target_uri, client_end_entity_certificate)
			}
		}
	}

	#[inline(always)]
	fn would_block<E>(error: io::Error, f: impl FnOnce(io::Error) -> E) -> Result<(), E>
	{
		if error.kind() == WouldBlock
		{
			Ok(())
		}
		else
		{
			Err(f(error))
		}
	}
}
