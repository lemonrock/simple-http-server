// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum NewServerClientConnectionError
{
	ServingMaximumNumberOfConnections(Option<io::Error>),

	NoDelay(io::Error),

	KeepAlive(io::Error),

	Linger(io::Error),

	CouldNotRegisterWithPoll(io::Error),
}

impl Display for NewServerClientConnectionError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NewServerClientConnectionError
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
		}
	}
}
