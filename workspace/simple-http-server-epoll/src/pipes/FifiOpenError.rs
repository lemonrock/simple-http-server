// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An error that can occur when opening one end of a FIFO (a named pipe).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FifiOpenError
{
	/// Errors common to opening or creation of most file descriptors.
	Common(CreationError),

	/// A (possibly mandatory) file lock is held on the FIFO.
	///
	/// Rationally, this would not seem to make sense but the Linux documentation doesn't make it clear if it is possible or not.
	///
	/// If this is encountered then an orderly shutdown is probably the only course of action as it is not possible to epoll for lock status changes on files that haven't even be opened.
	WouldBlock,

	/// `EINTR` occurred; this can be handled by either re-trying the open of the FIFO or might actual be fatal depending on the signal handling strategy in use.
	Interrupted,

	/// Invalid FIFO path
	InvalidFifoPath(InvalidFifoPathReason),
}

impl Display for FifiOpenError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<FifiOpenError as Debug>::fmt(self, f)
	}
}

impl error::Error for FifiOpenError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::FifiOpenError::*;

		match self
		{
			&Common(ref error) => Some(error),

			WouldBlock => None,

			Interrupted => None,

			&InvalidFifoPath(_) => None,
		}
	}
}
