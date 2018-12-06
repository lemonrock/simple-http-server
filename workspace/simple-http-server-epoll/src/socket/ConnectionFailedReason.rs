// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Additional information on why a connection failed during accept.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionFailedReason
{
	/// The connection aborted.
	Aborted,

	/// Firewall rules denied permission.
	FirewallPermissionDenied,

	/// Connection establishment timed out.
	TimedOut,

	/// A protocol error (`EPROTO`) occurred.
	Protocol,
}
