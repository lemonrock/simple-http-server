// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


bitflags!
{
	/// Flags returned by an epoll event.
	pub struct EPollEventFlags: u32
	{
		/// The associated file descriptor is available for read operations.
		const Input = EPOLLIN | EPOLLRDNORM;

		/// The associated file descriptor has urgent or out-of-band data available.
		///
		/// For TCP socket file descriptors, this usually means that out-of-band data (which is a deprecated concept) is available.
		const InputPriority = EPOLLPRI;

		/// The associated file descriptor is available for write operations.
		const Output = EPOLLOUT | EPOLLWRNORM;

		/// Stream socket peer closed connection, or shut down writing half of connection.
		///
		/// This flag is especially useful for writing simple code to detect peer shutdown when using Edge Triggered monitoring.
		///
		/// Valid since Linux 2.6.17.
		/// Valid on Solaris.
		const ReadShutdown = EPOLLRDHUP;

		/// Out-of-band (eg TCP urgent) data can be read using `MSG_OOB` to `recvfrom()`, say.
		const OutOfBandDataCanBeRead = EPOLLRDBAND;

		/// Out-of-band (eg TCP urgent) data can be read using `MSG_OOB` to `sendto()`, say.
		const OutOfBandDataCanBeWritten = EPOLLWRBAND;

		/// An unexpected (socket) error occurred.
		///
		/// This can include an unclean disconnection by a remote peer.
		const Error = EPOLLERR;

		/// The remote peer disconnected cleanly.
		const HangUp = EPOLLHUP;

		/// The specified endpoint descriptor is invalid.
		///
		/// Treat as an error.
		const OtherErrorOrNoBuffersQueued = EPOLLNVAL;
	}
}

impl Default for EPollEventFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
