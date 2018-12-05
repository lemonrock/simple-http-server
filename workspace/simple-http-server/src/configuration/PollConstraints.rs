// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Constraints to prevent over-use of poll resources.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PollConstraints
{
	/// How many events to be capable of processing at once?
	///
	/// Defaults to 1,024.
	///
	/// Will be floored to a minimum of 128.
	pub events_capacity: usize,

	/// Time out during polling to allow for processing of other events (eg signals).
	///
	/// Defaults to 1 millisecond.
	///
	/// Will be capped to a maximum of 100 milliseconds.
	///
	/// Will be floored to a minimum of 1 microsecond.
	pub poll_time_out: Duration,
}

impl Default for PollConstraints
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			events_capacity: 1024,
			poll_time_out: Duration::from_millis(1),
		}
	}
}

impl PollConstraints
{
	#[inline(always)]
	pub(crate) fn poll_time_out(&self) -> Option<Duration>
	{
		const Minimum: Duration = Duration::from_micros(1);

		const Maximum: Duration = Duration::from_millis(100);

		min(max(self.poll_time_out, Minimum), Maximum);

		Some(min(max(self.poll_time_out, Minimum), Maximum))
	}

	#[inline(always)]
	pub(crate) fn events(&self) -> Events
	{
		const Minimum: usize = 128;

		Events::with_capacity(max(self.events_capacity, Minimum))
	}
}
