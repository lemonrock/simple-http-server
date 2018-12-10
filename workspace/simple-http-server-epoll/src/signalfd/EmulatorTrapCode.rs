// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGEMT` signal.
///
/// This signal only occurs for the Alpha, MIPS and SPARC architectures (but Alpha isn't supported by Rust).
///
/// Definitions valid as of Linux v4.20-rc5.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum EmulatorTrapCode
{
	/// Tag overflow.
	///
	/// Known as `EMT_TAGOVF` in Linux sources.
	TagOverflow = 1,
}

impl Into<i32> for EmulatorTrapCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for EmulatorTrapCode
{
	type Data = FaultData;

	const InclusiveMaximum: Self = EmulatorTrapCode::TagOverflow;

	#[inline(always)]
	fn convert(code: i32) -> Self
	{
		unsafe { transmute(code) }
	}
}
