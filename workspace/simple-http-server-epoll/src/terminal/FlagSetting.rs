// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// On or off.
///
/// Default is `FlagSetting::Off`.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlagSetting
{
	/// Set on.
	On,

	/// Set off.
	Off,
}

impl Into<bool> for FlagSetting
{
	#[inline(always)]
	fn into(self) -> bool
	{
		use self::FlagSetting::*;

		match self
		{
			On => true,
			Off => false,
		}
	}
}

impl From<bool> for FlagSetting
{
	#[inline(always)]
	fn from(value: bool) -> Self
	{
		use self::FlagSetting::*;

		match value
		{
			true => On,
			false => Off,
		}
	}
}

impl Default for FlagSetting
{
	#[inline(always)]
	fn default() -> Self
	{
		FlagSetting::Off
	}
}
