// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Abstracts carriage return (`CR`, `\r`) delay.
///
/// Defaults to zero.
///
/// Values other than zero are only supported on Android, ?Fuschia, iOS, Linux and macos.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum CarriageReturnDelay
{
	/// Zero.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] Zero = CR0,
	#[cfg(not(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos")))] Zero = 0,

	/// One.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] One = CR1,

	/// Two.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] Two = CR2,

	/// Three.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] Three = CR3,
}

impl Into<tcflag_t>
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}

impl Default for CarriageReturnDelay
{
	#[inline(always)]
	fn default() -> Self
	{
		CarriageReturnDelay::Zero
	}
}

impl MultipleBits for CarriageReturnDelay
{
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] const Bitmask: tcflag_t = CRDLY;
}
