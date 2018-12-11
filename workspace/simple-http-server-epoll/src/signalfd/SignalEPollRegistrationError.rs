// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An error that can occur during registration of a signal instance with epoll.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SignalEPollRegistrationError
{
	/// Error on creation.
	Creation(CreationError),

	/// Error during registration.
	Registration(EPollAddError),
}

impl Display for SignalEPollRegistrationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<SignalEPollRegistrationError as Debug>::fmt(self, f)
	}
}

impl error::Error for SignalEPollRegistrationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::SignalEPollRegistrationError::*;

		match self
			{
				&Creation(ref error) => Some(error),
				&Registration(ref error) => Some(error),
			}
	}
}

impl From<CreationError> for SignalEPollRegistrationError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		SignalEPollRegistrationError::Creation(error)
	}
}

impl From<EPollAddError> for SignalEPollRegistrationError
{
	#[inline(always)]
	fn from(error: EPollAddError) -> Self
	{
		SignalEPollRegistrationError::Registration(error)
	}
}
