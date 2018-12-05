// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a reference to a buffer, in a similar way to `Rc`.
#[derive(Debug)]
struct BufferReference
{
	buffer_pointer: NonNull<Buffer>,
	buffer_arena: Arc<BufferArena>,
}

impl Drop for BufferReference
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let no_more_references = self.decrement_reference_count();

		if no_more_references
		{
			BufferArena::deallocate(self.buffer_arena, self.buffer_pointer)
		}
	}
}

impl Clone for BufferReference
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		self.increment_reference_count();

		Self
		{
			buffer_pointer: self.buffer_pointer,
			buffer_arena: self.buffer_arena.clone(),
		}
	}
}

impl<'b> BufferReference
{
	#[inline(always)]
	pub fn borrow(&'b self) -> Ref<'b, [u8]>
	{
		self.immutable_buffer_reference().borrow()
	}

	#[inline(always)]
	pub fn try_borrow(&'b self) -> Result<Ref<'b, [u8]>, BorrowError>
	{
		self.immutable_buffer_reference().try_borrow()
	}

	#[inline(always)]
	pub fn borrow_mut(&'b self) -> RefMut<'b, [u8]>
	{
		self.immutable_buffer_reference().borrow_mut()
	}

	#[inline(always)]
	pub fn try_borrow_mut(&'b self) -> Result<RefMut<'b, [u8]>, BorrowMutError>
	{
		self.immutable_buffer_reference().try_borrow_mut()
	}
}

impl BufferReference
{
	#[inline(always)]
	fn reused(buffer: &Buffer, buffer_arena: &Arc<BufferArena>) -> Self
	{
		Self
		{
			buffer_pointer: unsafe { NonNull::new_unchecked(buffer as *const _ as *mut _) },
			buffer_arena: buffer_arena.clone(),
		}
	}

	#[inline(always)]
	fn increment_reference_count(&self)
	{
		self.immutable_buffer_reference().increment_reference_count()
	}

	#[inline(always)]
	fn decrement_reference_count(&self) -> bool
	{
		self.immutable_buffer_reference().decrement_reference_count()
	}

	#[inline(always)]
	fn immutable_buffer_reference(&self) -> &Buffer
	{
		unsafe { self.buffer_pointer.as_ref() }
	}
}
