// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An error that can occur during read of a timer instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructReadError
{
	/// There are no timer events to read at this time.
	WouldBlock,

	/// Timer was cancelled because it depends on the realtime clock and the realtime clock was adjusted.
	Cancelled,

	/// `EINTR` occurred; this can be handled by either re-trying the `read()` or might actual be fatal depending on the signal handling strategy in use.
	Interrupted,
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
}
