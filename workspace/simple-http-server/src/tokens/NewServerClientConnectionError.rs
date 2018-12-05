// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum NewServerClientConnectionError<SCCUF: ServerClientConnectionUserFactory>
{
	ServingMaximumNumberOfConnections(Option<io::Error>),

	NoDelay(io::Error),

	KeepAlive(io::Error),

	Linger(io::Error),

	ReceiveBufferSize(io::Error),

	SendBufferSize(io::Error),

	CouldNotCreateNewServedClientConnectionUser(ConnectionObserverConnectError<SCCUF::Error>),

	CouldNotAllocateMemory,

	FailedOnFirstUse,
}

impl<SCCUF: ServerClientConnectionUserFactory> Display for NewServerClientConnectionError<SCCUF>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<SCCUF: ServerClientConnectionUserFactory> error::Error for NewServerClientConnectionError<SCCUF>
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::NewServerClientConnectionError::*;

		match self
		{
			&ServingMaximumNumberOfConnections(ref option_error) => match option_error.as_ref()
			{
				None => None,
				Some(error) => error,
			},

			&NoDelay(ref error) => Some(error),

			&KeepAlive(ref error) => Some(error),

			&Linger(ref error) => Some(error),

			&ReceiveBufferSize(ref error) => Some(error),

			&SendBufferSize(ref error) => Some(error),

			&CouldNotCreateNewServedClientConnectionUser(ref error) => Some(error),

			&CouldNotAllocateMemory => None,

			&FailedOnFirstUse => None,
		}
	}
}
