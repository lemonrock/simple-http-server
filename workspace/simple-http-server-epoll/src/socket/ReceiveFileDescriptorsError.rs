// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An error that can occur during receive of file descriptors.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveFileDescriptorsError
{
	/// A read error that might be possible to handle.
	Read(StructReadError),

	MoreThanOneHeader,

	WasNotSocketLevelMessage,

	WasNotScmRights,
}

impl Display for StructReadError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<StructReadError as Debug>::fmt(self, f)
	}
}

impl error::Error for StructReadError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::ReceiveFileDescriptorsError::*;

		match self
		{
			&Read(ref error) => Some(error),

			&MoreThanOneHeader => None,

			&WasNotSocketLevelMessage => None,

			&WasNotScmRights => None,
		}
	}
}
