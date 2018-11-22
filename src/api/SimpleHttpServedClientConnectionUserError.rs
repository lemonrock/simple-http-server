// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub enum SimpleHttpServedClientConnectionUserError
{
	CloseNotify,

	ReadBufferLengthEqualed,

	HttpHeadersInvalid(::thhp::Error),

	UnsupportedHttpMinorVersion,

	UnsupportedHttpMethod,

	AlpnProtocolMismatchesHttpMinorVersion,

	Http11MissingHostHeader,

	HostHeaderHostnameMismatch,

	HostHeaderHasIncorrectPort,

	SniHostnameMismatch,

	TargetIsNotAbsolute,

	TargetIsInvalidUri(::url::ParseError),

	/// This should not occur as it is supposed to have been validated by rustls.
	EndEntityClientCertificateInvalid(webpki::Error),

	HttpGetUser(E),
}

impl Display for SimpleHttpServedClientConnectionUserError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for SimpleHttpServedClientConnectionUserError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::SimpleHttpServedClientConnectionUserError::*;

		match self
		{
			&CloseNotify => None,

			&ReadBufferLengthEqualed => None,

			&HttpHeadersInvalid(ref error) => Some(error),

			&UnsupportedHttpMinorVersion => None,

			&UnsupportedHttpMethod => None,

			&AlpnProtocolMismatchesHttpMinorVersion => None,

			&Http11MissingHostHeader => None,

			&HostHeaderHostnameMismatch => None,

			&HostHeaderHasIncorrectPort => None,

			&SniHostnameMismatch => None,

			&TargetIsNotAbsolute => None,

			&TargetIsInvalidUri(ref error) => Some(error),

			&EndEntityClientCertificateInvalid(ref error) => Some(error),

			&HttpGetUser(ref error) => Some(error),
		}
	}
}
