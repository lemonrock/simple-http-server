// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum ServerSessionProcessWriteReadError<E: error::Error>
{
	EndOfFileWhilstHandshaking,

	SocketVectoredWrite(io::Error),

	SocketRead(io::Error),

	ProcessNewPackets(TLSError, Option<io::Error>),

	EndOfFile,

	ServedClientConnectionUser(E)
}

impl<E: error::Error> Display for ServerSessionProcessWriteReadError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: error::Error> error::Error for ServerSessionProcessWriteReadError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::ServerSessionProcessWriteReadError::*;

		match self
		{
			&EndOfFileWhilstHandshaking => None,

			&SocketVectoredWrite(ref error) => Some(error),

			&SocketRead(ref error) => Some(error),

			&ProcessNewPackets(ref error, ..) => Some(error),

			&EndOfFile => None,

			&ServedClientConnectionUser(ref error) => Some(error),
		}
	}
}
