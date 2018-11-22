// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Constraints to prevent over-use of server resources.
///
/// Note that `rlimit` may still need to be set, particularly on Linux systems, to handle more than about 1020 connections.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Constraints
{
	/// How many events to be capable of processing at once?
	///
	/// Defaults to 1,024.
	pub events_capacity: usize,

	/// Maximum number of served client connections that can be simultaneously open.
	///
	/// Defaults to 4,096.
	pub maximum_connections: usize,

	/// Buffer limit, in bytes, passed to rustls; controls internal write buffers and unread plain text buffers.
	///
	/// Zero (0) is interpreted as infinite.
	///
	/// Defaults to 16,384 bytes (16Kb).
	pub rustls_buffer_limit: usize,

	/// Time out during polling to allow for processing of other events (eg signals).
	///
	/// Defaults to 1 millisecond.
	pub poll_time_out: Duration,

	/// Receive buffer size, in bytes.
	///
	/// Defaults to 16,384 bytes (16Kb)
	pub receive_buffer_size: usize,

	/// Send buffer size, in bytes.
	///
	/// Defaults to 16,384 bytes (16Kb).
	pub send_buffer_size: usize,
}

impl Default for Constraints
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			events_capacity: 1024,
			maximum_connections: 4096,
			rustls_buffer_limit: 16_384,
			poll_time_out: Duration::from_millis(1),
			receive_buffer_size: 16_384,
			send_buffer_size: 16_384,
		}
	}
}

impl Constraints
{
	#[inline(always)]
	pub(crate) fn poll_time_out(&self) -> Option<Duration>
	{
		Some(self.constraints.poll_time_out)
	}

	#[inline(always)]
	pub(crate) fn events(&self) -> Events
	{
		Events::with_capacity(self.constraints.events_capacity)
	}

	#[inline(always)]
	pub(crate) fn set_rustls_buffer_limit(&self, server_session: &mut ServerSession) -> Events
	{
		server_session.set_buffer_limit(self.rustls_buffer_limit)
	}
}
