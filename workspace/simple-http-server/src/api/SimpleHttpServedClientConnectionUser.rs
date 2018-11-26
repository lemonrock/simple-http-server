// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A simple HTTP implementation.
pub struct SimpleHttpServedClientConnectionUser
{
	read_buffer: [u8; SimpleHttpServedClientConnectionUser::ReadBufferSize],
	read_buffer_offset: usize,
	reading_request: bool,

	settings: Rc<SimpleHttpServedClientConnectionUserSettings>,
}

impl ServedClientConnectionUser for SimpleHttpServedClientConnectionUser
{
	type Error = SimpleHttpServedClientConnectionUserError;

	fn service<'a>(&mut self, simplified_server_session: SimplifiedServerSession<'a>) -> Result<(), Result<RegistrationState, Self::Error>>
	{
		use self::SimpleHttpServedClientConnectionUserError::*;

		simplified_server_session.write_then_read(self.reading_request)?;

		// TODO: Pipelined requests.

		if self.reading_request
		{
			let bytes_read = simplified_server_session.copy_plain_text_into_buffer(&mut self.read_buffer[self.read_buffer_offset .. ])?;
			self.read_buffer_offset += bytes_read;
			if self.read_buffer_offset == Self::ReadBufferSize
			{
				return Err(Err(ReadBufferLengthEqualed))
			}

			//parse_request_method(bytes: &mut Bytes) -> Result<RequestMethod, Status<NonNull<u8>>>
			//TargetUriReentryPoint::parse
			//parse_http_version
			//







			let mut headers: Vec<HeaderField> = Vec::with_capacity(LikelyMaximumRequestHeaders);
			match Request::parse(&self.read_buffer, &mut headers).map_err(|error| HttpHeadersInvalid(error))?
			{
				Ok(Incomplete) =>
				{
					Err(Ok(simplified_server_session.read_registration_state()))
				}

				Ok(Complete((ref request, length))) =>
				{
					self.reading_request = false;
					let (headers, target_uri, client_end_entity_certificate) = self.request_headers_completely_read(simplified_server_session, headers, request, length);

					// TODO: Call some method that returns a data buffer, the later of which can be got as a slice
				},
			}
		}

		pub type ResponseDataBufferIdentifier = usize;

		/// A response, consisting of one or more buffers.
		///
		/// Designed to make it easy to resume writes, work with large static files and utilise fixed http.
		pub trait Response
		{
			/// Provide a tuple of a data buffer to be written out and its identifier; this will be passed back with the amount of data written (this call will be made within the same stack frame).
			///
			/// Return None if no more data is to be provided.
			fn provide_data(&mut self) -> Option<Result<(&[u8], DataBufferIdentifier), ()>>;

			fn data_written(&mut self, bytes_written: usize, data_buffer_identifier: DataBufferIdentifier);
		}

		struct LargeFileResponse
		{
			buffer: [u8; 1024 * 32],
			buffer_end: usize,
			file: Option<File>,
		}

		impl Response for FileResponse
		{
			fn provide_data(&mut self) -> Option<Result<(&[u8], DataBufferIdentifier), ()>>
			{
				match self.file.as_mut()
				{
					None => Some(Ok((&self.buffer[ .. self.buffer_end], 0))),
					Some(file) =>
					{

					}
				}
				unimplemented!()
			}

			fn data_written(&mut self, bytes_written: usize, data_buffer_identifier: ResponseDataBufferIdentifier)
			{
				unimplemented!()
			}
		}

		;
	}
}

impl SimpleHttpServedClientConnectionUser
{
	const ReadBufferSize: usize = 4096;

	const LikelyMaximumRequestHeaders: usize = 16;

	pub(crate) fn new(settings: &Rc<SimpleHttpServedClientConnectionUserSettings>) -> Self
	{
		Self
		{
			reading_request: true,
			read_buffer: unsafe { uninitialized() },
			read_buffer_offset: 0,

			settings: settings.clone(),
		}
	}

	fn request_headers_completely_read<'a>(&mut self, simplified_server_session: SimplifiedServerSession<'a>, headers: Vec<HeaderField>, request: &Request, _length: usize) -> Result<(Vec<HeaderField>, Url, Option<EndEntityCert>), SimpleHttpServedClientConnectionUserError>
	{
		Self::validate_minor_version(&request)?;

		Self::validate_request_method_is_get(&request)?;

		Self::validate_alpn_protocol(&request, &self.server_session)?;

		Self::validate_host_header(&request, &headers, self.settings.our_hostname, self.settings.our_port_string)?;

		Self::validate_sni_hostname(&self.server_session, self.settings.our_hostname)?;

		Self::validate_target_is_absolute(request.target)?;

		let target_uri = self.settings.our_base_url.join(request.target).map_err(|error| SimpleHttpServedClientConnectionUserError::TargetIsInvalidUri(error))?;

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
				Some(EndEntityCert::from(first.der_encoded_certificate()).map_err(SimpleHttpServedClientConnectionUserError::EndEntityClientCertificateInvalid)?)
			},
		};

		Ok((headers, target_uri, client_end_entity_certificate))
	}

	fn validate_minor_version(request: &Request) -> Result<(), SimpleHttpServedClientConnectionUserError>
	{
		if request.minor_version_is_invalid()
		{
			Err(SimpleHttpServedClientConnectionUserError::UnsupportedHttpMinorVersion)
		}
		else
		{
			Ok(())
		}
	}

	fn validate_request_method_is_get(request: &Request) -> Result<(), SimpleHttpServedClientConnectionUserError>
	{
		if request.method_is_other_than_get()
		{
			Err(SimpleHttpServedClientConnectionUserError::UnsupportedHttpMethod)
		}
		else
		{
			Ok(())
		}
	}

	fn validate_alpn_protocol<'buffer>(request: &Request, server_session: &ServerSession) -> Result<(), SimpleHttpServedClientConnectionUserError>
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
				return Err(SimpleHttpServedClientConnectionUserError::AlpnProtocolMismatchesHttpMinorVersion)
			}
		}
		Ok(())
	}

	fn validate_host_header<'buffer>(request: &Request, headers: &Vec<HeaderField<'buffer>>, our_hostname: &str, our_port_string: &str) -> Result<(), SimpleHttpServedClientConnectionUserError>
	{
		use self::SimpleHttpServedClientConnectionUserError::*;

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

	fn validate_sni_hostname(server_session: &ServerSession, our_hostname: &str) -> Result<(), SimpleHttpServedClientConnectionUserError>
	{
		if let Some(sni_hostname) = server_session.get_sni_hostname()
		{
			if !sni_hostname.eq_ignore_ascii_case(our_hostname)
			{
				return Err(SimpleHttpServedClientConnectionUserError::SniHostnameMismatch)
			}
		}

		Ok(())
	}

	fn validate_target_is_absolute(target: &str) -> Result<(), SimpleHttpServedClientConnectionUserError>
	{
		if request.target.starts_with('/')
		{
			Ok(())
		}
		else
		{
			Err(SimpleHttpServedClientConnectionUserError::TargetIsNotAbsolute)
		}
	}
}
