// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


pub(crate) struct Bytes
{
	current_pointer: NonNull<u8>,
	end_pointer: NonNull<u8>,
}

impl Bytes
{
	#[inline(always)]
	pub(crate) fn is_slice<ReentryPoint>(&mut self, compare_to: &[u8], reentry_point: ReentryPoint, invalid_reason: InvalidReason) -> Result<(), Status<ReentryPoint>>
	{
		let length = compare_to.len();

		debug_assert!(length > 1, "compare_to.len() must be more than 1");

		use self::Status::*;

		let end_pointer = self.as_if(length);
		if end_pointer <= self.end_pointer
		{
			let mut compare_to_pointer = NonNull::from_const(compare_to.as_ptr());
			let mut pointer = self.current_pointer;
			while
			{
				if compare_to.value() != pointer.value()
				{
					return Err(Invalid(invalid_reason))
				}

				compare_to_pointer.increment();
				pointer.increment();
				pointer != end_pointer
			}
			{
			}

			self.current_pointer = end_pointer;
			Ok(())
		}
		else
		{
			Err(RunOutOfBytes(reentry_point))
		}
	}

	#[inline(always)]
	pub(crate) fn is<ReentryPoint>(&mut self, compare_to: u8, reentry_point: ReentryPoint, invalid_reason: InvalidReason) -> Result<(), Status<ReentryPoint>>
	{
		if self.has_more()
		{
			let current = self.current();
			if current == compare_to
			{
				self.increment();
				Ok(())
			}
			else
			{
				Err(Invalid(invalid_reason))
			}
		}
		else
		{
			Err(RunOutOfBytes(reentry_point))
		}
	}

	#[inline(always)]
	pub(crate) fn if_has_more_return_current_value_and_increment<ReentryPoint>(&self, reentry_point: ReentryPoint) -> Result<u8, Status<ReentryPoint>>
	{
		if self.has_more()
		{
			let current = self.current();
			self.increment();
			Ok(current)
		}
		else
		{
			Err(RunOutOfBytes(reentry_point))
		}
	}

	#[inline(always)]
	pub(crate) fn reset(&mut self, pointer: NonNull<u8>)
	{
		self.current_pointer = pointer;
	}

	#[inline(always)]
	pub(crate) fn previous(&self) -> NonNull<u8>
	{
		self.current_pointer.previous()
	}

	#[inline(always)]
	fn has_more(&self) -> bool
	{
		self.current_pointer != self.end_pointer
	}

	#[inline(always)]
	fn increment(&mut self)
	{
		self.current_pointer.increment()
	}

	#[inline(always)]
	fn current(&self) -> u8
	{
		self.current_pointer.value()
	}
}
