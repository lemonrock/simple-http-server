// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


trait NonNullExt<T>
{
	fn from_const(raw: *const T) -> Self;

	fn from_mut(raw: *mut T) -> Self;

	fn increment(&mut self);

	fn next(self) -> Self;

	fn previous(self) -> Self;

	fn as_if(&self, offset: usize) -> Self;

	fn value(self) -> T;

	fn difference(self, lesser: T) -> usize;
}

impl<T> NonNullExt<T> for NonNull<T>
{
	#[inline(always)]
	fn from_const(raw: *const T) -> Self
	{
		unsafe { NonNull::new_unchecked(raw as *mut _) }
	}

	#[inline(always)]
	fn from_mut(raw: *mut T) -> Self
	{
		unsafe { NonNull::new_unchecked(raw) }
	}

	#[inline(always)]
	fn increment(&mut self)
	{
		*self = self.next()
	}

	#[inline(always)]
	fn next(self) -> Self
	{
		unsafe { NonNull::new_unchecked(self.as_ptr().add(1)) }
	}

	#[inline(always)]
	fn previous(self) -> Self
	{
		unsafe { NonNull::new_unchecked(self.as_ptr().offset(-1)) }
	}

	#[inline(always)]
	fn as_if(&self, offset: usize) -> Self
	{
		unsafe { NonNull::new_unchecked(self.as_ptr().add(offset)) }
	}

	#[inline(always)]
	fn value(self) -> T
	{
		unsafe { *self.as_ptr() }
	}

	#[inline(always)]
	fn difference(self, lesser: T) -> usize
	{
		(self.as_ptr() as usize) - (lesser.as_ptr() as usize)
	}
}
