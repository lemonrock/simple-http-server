// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An error that can occur during creation of a socket listener.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NewSocketServerListenerError
{
	/// Creation.
	Creation(SocketCreationError),

	/// Bind.
	Bind(SocketBindError),
}

impl Display for NewSocketServerListenerError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<NewSocketServerListenerError as Debug>::fmt(self, f)
	}
}

impl error::Error for NewSocketServerListenerError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::NewSocketServerListenerError::*;

		match self
		{
			&Creation(ref error) => Some(error),
			&Bind(ref error) => Some(error),
		}
	}
}

impl From<SocketCreationError> for NewSocketServerListenerError
{
	#[inline(always)]
	fn from(error: SocketCreationError) -> Self
	{
		NewSocketServerListenerError::Creation(error)
	}
}

impl From<SocketBindError> for NewSocketServerListenerError
{
	#[inline(always)]
	fn from(error: SocketBindError) -> Self
	{
		NewSocketServerListenerError::Bind(error)
	}
}
