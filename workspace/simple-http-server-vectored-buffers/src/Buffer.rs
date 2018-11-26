// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Buffer
{
	start_pointer: NonNull<u8>,
	size: usize,
}

impl Drop for Buffer
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// TODO: drop !
	}
}

impl AsRef<[u8]> for Buffer
{
	#[inline(always)]
	fn as_ref(&self) -> &[u8]
	{
		unsafe { from_raw_parts(self.start_pointer.as_ptr() as *const u8, self.size) }
	}
}

impl AsMut<[u8]> for Buffer
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [u8]
	{
		unsafe { from_raw_parts_mut(self.start_pointer.as_ptr(), self.size) }
	}
}

impl Deref for Buffer
{
	type Target = [u8];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.as_ref()
	}
}

impl DerefMut for Buffer
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.as_mut()
	}
}
