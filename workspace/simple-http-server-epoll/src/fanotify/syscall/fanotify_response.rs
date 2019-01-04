// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A response.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct fanotify_response
{
	/// This is the file descriptor from the structure `fanotify_event_metadata`.
	fd: RawFd,

	/// This field indicates whether or not the permission is to be granted.
	///
	/// Its value must be either `FAN_ALLOW` to allow the file operation or `FAN_DENY` to deny the file operation.
	response: c_uint,
}

impl fanotify_response
{
	/// An allow response.
	#[inline(always)]
	pub const fn allow(fd: RawFd) -> Self
	{
		Self
		{
			fd,
			response: FAN_ALLOW,
		}
	}

	/// A deny response.
	#[inline(always)]
	pub const fn deny(fd: RawFd) -> Self
	{
		Self
		{
			fd,
			response: FAN_DENY,
		}
	}
}

/// Allow grant of permission.
///
/// Use for field `response` of `fanotify_response`.
pub(crate) const FAN_ALLOW: c_uint = 0x01;

/// Deny grant of permission.
///
/// Use for field `response` of `fanotify_response`.
pub(crate) const FAN_DENY: c_uint = 0x02;
