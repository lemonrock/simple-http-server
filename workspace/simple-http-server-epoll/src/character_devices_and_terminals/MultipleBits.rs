// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


trait MultipleBits: Default + Copy + Into<tcflag_t>
{
	const Bitmask: tcflag_t = 0;

	#[inline(always)]
	fn from_mode_flags(mode_flags: tcflag_t) -> Self
	{
		unsafe { transmute(mode_flags | Self::Bitmask) }
	}

	#[inline(always)]
	fn change_mode_flags(this: Option<Self>, current_flags: tcflag_t) -> tcflag_t
	{
		match this
		{
			None => current_flags,
			Some(multiple_bits) =>
			{
				let bits = multiple_bits.into();
				(current_flags & !Self::Bitmask) | bits
			},
		}
	}
}
