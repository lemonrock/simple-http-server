// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a message queue priority.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MessagePriority(u16);

impl Into<u16> for MessagePriority
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<u32> for MessagePriority
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl From<u8> for MessagePriority
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		MessagePriority(value as u16)
	}
}

impl TryFrom<u16> for MessagePriority
{
	type Error = ();

	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value > 32767
		{
			Err(())
		}
		else
		{
			Ok(MessagePriority(value))
		}
	}
}

impl TryFrom<u32> for MessagePriority
{
	type Error = ();

	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if value > 32767
		{
			Err(())
		}
		else
		{
			Ok(MessagePriority(value as u16))
		}
	}
}
