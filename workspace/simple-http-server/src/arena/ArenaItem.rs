// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An `ArenaItem` is one that can be stored in an Arena.
#[derive(Debug)]
pub(crate) struct ArenaItem<InnerValue: Sized>
{
	next_available_slot_index: Cell<usize>,
	inner_value: Option<ManuallyDrop<InnerValue>>,
}

impl<InnerValue: Sized> ArenaItem<InnerValue>
{
	#[inline(always)]
	pub(crate) fn set_inner_value(&mut self, inner_value: InnerValue)
	{
		self.inner_value = Some(ManuallyDrop::new(inner_value))
	}

	#[inline(always)]
	fn new(index: usize) -> Self
	{
		Self
		{
			next_available_slot_index:
			{
				assert_ne!(index, ::std::usize::MAX - 1, "index can not be ::std::usize::MAX - 1 otherwise overflow occurs");
				Cell::new(index + 1)
			},
			inner_value: None,
		}
	}

	#[inline(always)]
	fn next_available_slot_index(&self) -> usize
	{
		self.next_available_slot_index.get()
	}

	#[inline(always)]
	fn set_next_available_slot_index(&self, next_available_slot_index: usize)
	{
		self.next_available_slot_index.set(next_available_slot_index)
	}

	#[inline(always)]
	fn deallocated(&mut self)
	{
		if let Some(inner_value) = item.inner_value.take()
		{
			drop(ManuallyDrop::into_inner(inner_value))
		}
	}
}

impl<InnerValue: Sized> Deref for ArenaItem<InnerValue>
{
	type Target = InnerValue;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.inner_value.as_ref().unwrap().deref()
	}
}

impl<InnerValue: Sized> DerefMut for ArenaItem<InnerValue>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.inner_value.as_mut().unwrap().deref_mut()
	}
}
