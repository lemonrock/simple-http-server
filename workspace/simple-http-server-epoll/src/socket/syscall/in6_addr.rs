// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Copy, Clone, Eq)]
#[repr(C)]
pub union in6_addr
{
	/// 16 bytes.
	pub s6_addr: [uint8_t; 16],

	/// 8 network endian 16-bit integers.
	pub s6_addr16: [uint16_t; 8],

	/// 4 network endian 32-bit integers.
	pub s6_addr32: [uint32_t; 4],
}

impl Default for in6_addr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for in6_addr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{:?}", unsafe { self.s6_addr })
	}
}

impl PartialEq for in6_addr
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.s6_addr == other.s6_addr }
	}
}

impl PartialOrd for in6_addr
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.s6_addr.partial_cmp(&other.s6_addr) }
	}
}

impl Ord for in6_addr
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.s6_addr.cmp(&other.s6_addr) }
	}
}

impl Hash for in6_addr
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		unsafe { self.s6_addr.hash(hasher) }
	}
}
