// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A fatal error whilst executing the main loop.
#[derive(Debug)]
pub enum MainLoopError
{
	/// Main loop poll creation failed.
	PollCreation(io::Error),

	/// Could not allocate memory for a server listener.
	CouldNotAllocateMemoryForAServerListener,

	/// Parsing TCP server listener socket address failed.
	CouldNotParseTcpListenerSocketAddress(AddrParseError),

	/// Binding TCP server listener failed.
	CouldNotBindTcpListener(io::Error),

	/// Registering TCP server listener with poll failed.
	CouldNotRegisterTcpListenerWithPoll(io::Error),

	/// Registering channel with poll failed.
	CouldNotRegisterChannelWithPoll(io::Error),

	/// Starting a worker thread failed.
	WorkerCreation(WorkerCreationError),

	/// Poll listen failed.
	PollLoop(io::Error),
}

impl Display for MainLoopError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MainLoopError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::MainLoopError::*;

		match self
		{
			&PollCreation(ref error) => Some(error),

			&CouldNotAllocateMemoryForAServerListener => None,

			&CouldNotParseTcpListenerSocketAddress(ref error) => Some(error),

			&CouldNotBindTcpListener(ref error) => Some(error),

			&CouldNotRegisterTcpListenerWithPoll(ref error) => Some(error),

			&CouldNotRegisterChannelWithPoll(ref error) => Some(error),

			&WorkerCreation(ref error) => Some(error),

			&PollLoop(ref error) => Some(error),
		}
	}
}
