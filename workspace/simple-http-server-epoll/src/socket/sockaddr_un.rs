// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Clone)]
#[repr(C)]
pub struct sockaddr_un
{
	/// Socket address family.
	sun_family: sa_family_t,

	/// Zero-terminated C String.
	///
	/// ***Caution!***
	///
	/// If the string is exactly `sockaddr_un::PathLength` bytes, it is not ASCII NUL terminated.
	pub sun_path: [c_char; sockaddr_un::PathLength]
}

impl Default for sockaddr_un
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			sun_family: AF_UNIX as sa_family_t,
			sun_path: unsafe { zeroed() },
		}
	}
}

impl Debug for sockaddr_un
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "sockaddr_un {{ sun_family: {}, sun_path: {:?} }}", self.sun_family, &self.sun_path[..])
	}
}

impl PartialEq for sockaddr_un
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.sun_family == other.sun_family && (&self.sun_path[..]) == (&other.sun_path[..])
	}
}

impl Eq for sockaddr_un
{
}

impl PartialOrd for sockaddr_un
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for sockaddr_un
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.sun_family.cmp(&other.sun_family).then_with(|| (&self.sun_path[..]).cmp(&other.sun_path[..]))
	}
}

impl Hash for sockaddr_un
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.sun_family.hash(hasher);
		(&self.sun_path[..]).hash(hasher);
	}
}

impl SocketData for sockaddr_un
{
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sun_family
	}
}

impl sockaddr_un
{
	/// Length of the `sun_path` entry.
	pub const PathLength: usize = 108;
}
