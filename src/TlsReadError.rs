// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum TlsReadError
{
	SocketReadError(io::Error),

	EndOfFile,

	ProcessNewPacketsError(TLSError),

	ReadToEndError(io::Error),

	ReadBufferLengthExceeded,

	HttpViolation(HttpServerReadError),
}

impl Display for TlsReadError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TlsReadError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::TlsReadError::*;

		match self
		{
			&SocketReadError(ref error) => Some(error),

			&EndOfFile => None,

			&ProcessNewPacketsError(ref error) => Some(error),

			&ReadToEndError(ref error) => Some(error),

			&ReadBufferLengthExceeded => None,

			&HttpViolation(ref error) => Some(error),
		}
	}
}
