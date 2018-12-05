// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A fatal error whilst connecting a new connection.
#[derive(Debug)]
pub(crate) enum ConnectionObserverConnectError<SCCUFE: error::Error>
{
	RemoteAddressBlocked,

	MaximumConnections,

	ServedClientConnectionUserFactoryErr(SCCUFE)
}

impl<SCCUFE: error::Error> Display for ConnectionObserverConnectError<SCCUFE>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<SCCUFE: error::Error> error::Error for ConnectionObserverConnectError<SCCUFE>
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::ConnectionObserverConnectError::*;

		match self
		{
			&RemoteAddressBlocked => None,

			&MaximumConnections => None,

			&ServedClientConnectionUserFactoryErr(ref error) => Some(error),
		}
	}
}
