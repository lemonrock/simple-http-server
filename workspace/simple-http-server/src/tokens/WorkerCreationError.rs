// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A fatal error whilst executing the main loop.
#[derive(Debug)]
pub enum WorkerCreationError
{
	/// Worker loop poll creation failed.
	PollCreation(io::Error),

	/// Could not allocate memory for a receive from worker thread.
	CouldNotAllocateMemoryForAReceiveFromWorkerThread,

	/// Registering receive half of a channel with Poll failed.
	CouldNotRegisterReceiveChannelWithPoll(io::Error),

	/// Could not spawn a worker thread.
	CouldNotSpawnWorkerThread(io::Error),
}

impl Display for WorkerCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for WorkerCreationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::WorkerCreationError::*;

		match self
		{
			&PollCreation(ref error) => Some(error),

			&CouldNotAllocateMemoryForAReceiveFromWorkerThread => None,

			&CouldNotRegisterReceiveChannelWithPoll(ref error) => Some(error),

			&CouldNotSpawnWorkerThread(ref error) => Some(error),
		}
	}
}
