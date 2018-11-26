// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Wrapper around a ServerSession to simplify its usage.
///
/// Only exists for state AFTER handshaking.
pub struct SimplifiedServerSession<'a>(&'a ServerSession, &'a mut TcpStream);

impl SimplifiedServerSession
{
	/// Will always complete outstanding writes and then read from the network at least once.
	///
	/// Should be called every time a poll event occurs that indicates read or write is possible.
	///
	/// if `try_to_read_until_there_is_some_plain_text` is specified, then will try to read from the network repeatedly until either there is at least one byte of plain text or the reads would block.
	#[inline(always)]
	pub fn write_then_read(&self, try_to_read_until_there_is_some_plain_text: bool) -> Result<(), Result<RegistrationState, ServerSessionProcessWriteReadError>>
	{
		self.0.process_write_read(self.1, try_to_read_until_there_is_some_plain_text)
	}

	/// Call this after `write_then_read(true)`.
	///
	/// Will only fail if the TLS message `close_notify` has been received.
	#[inline(always)]
	pub fn copy_plain_text_into_buffer(&mut self, buffer_to_copy_into: &mut [u8]) -> Result<usize, SimpleHttpServedClientConnectionUserError>
	{
		// The only error read() can return is ConnectionAborted which caused by CloseNotify being received.
		self.0.read(buffer_to_copy_into).map_err(|_| SimpleHttpServedClientConnectionUserError::CloseNotify)
	}

	/// A registration state that includes a read available notification request.
	#[inline(always)]
	pub fn read_registration_state(&self) -> RegistrationState
	{
		self.0.registration_state(true)
	}

	/// Queues a close_notify fatal alert to be sent in the next `write_tls` call.
	///
	/// This informs the peer that the connection is being closed.
	#[inline(always)]
	pub fn send_close_notify(&mut self)
	{
		self.0.send_close_notify()
	}

	/// Gets the client's end entity certificate from its presented certificate chain.
	///
	/// This function does 'work', so the result should be cached rather than calling this method more than once.
	#[inline(always)]
	pub fn get_client_end_entity_certificate(&self) -> Option<Result<EndEntityCert, webpki::Error>>
	{
		match self.0.get_peer_certificates()
		{
			None => None,

			Some(certificates) => if certificates.is_empty()
			{
				None
			}
			else
			{
				let first = unsafe { certificates.get_unchecked(0) };
				Some(EndEntityCert::from(first.der_encoded_certificate()))
			},
		}
	}

	/// Agreed ALPN protocol with peer.
	///
	/// None if no protocol was agreed (because no protocols were offered or accepted by the peer).
	#[inline(always)]
	pub fn get_alpn_protocol(&self) -> Option<&str>
	{
		self.0.get_alpn_protocol()
	}

	/// Exports keying material.
	#[inline(always)]
	pub fn export_keying_material(&self, output: &mut [u8], label: &[u8], context: Option<&[u8]>) -> Result<(), TLSError>
	{
		self.0.export_keying_material(output, label, context)
	}

	/// Retrieves the SNI hostname, if any, used to select the certificate and private key.
	///
	/// This is useful for application protocols that need to enforce that the SNI hostname matches an application layer protocol hostname. For example, HTTP/1.1 servers commonly expect the `Host:` header field of every request on a connection to match the hostname in the SNI extension when the client provides the SNI extension.
	#[inline(always)]
	pub fn get_sni_hostname(&self) -> Option<&str>
	{
		self.0.get_sni_hostname()
	}
}
