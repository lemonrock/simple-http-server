// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An `Arena` holds enough memory for all the items that need allocating (it can be thought of as pre-allocating).
///
/// Consequently, item allocation and deallocation is very cheap (and so very fast).
#[derive(Debug)]
pub(crate) struct Arena<InnerValue: Sized>
{
	number_of_items: usize,

	/// Is equal to number_of_items when full.
	next_available_slot_index: AtomicUsize,

	allocation: Allocation,

	marker: PhantomData<InnerValue>,
}

impl<InnerValue: Sized> Drop for Arena<InnerValue>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.allocation.iterate(self.number_of_items, |item| (unsafe { &mut * item }).deallocated());

		self.allocation.deallocate(self.number_of_items)
	}
}

impl<InnerValue: Sized> Arena<InnerValue>
{
	/// Create a new instance for `number_of_items`.
	#[inline(always)]
	pub fn with_capacity(number_of_items: usize) -> Arc<Self>
	{
		let allocation = Allocation::allocate(number_of_items);
		allocation.initialize(number_of_items, Item::new);

		Arc::new
		(
			Self
			{
				number_of_items,
				next_available_slot_index: 0,
				allocation,
				marker: PhantomData,
			}
		)
	}

	/// Allocate an item.
	#[inline(always)]
	pub(crate) fn allocate(&self) -> Option<&mut ArenaItem<InnerValue>>
	{
		let mut current_next_available_slot_index = self.next_available_slot_index.load(Relaxed);
		let item = loop
		{
			let is_full = current_next_available_slot_index == self.number_of_items;

			if is_full
			{
				return None
			}

			let item = self.allocation.item_mutable_reference(current_next_available_slot_index);

			let next_available_slot_index = item.next_available_slot_index();

			match self.next_available_slot_index.compare_exchange(current_next_available_slot_index, next_available_slot_index, SeqCst, Relaxed)
			{
				Ok(_) => break item,
				Err(other_next_available_slot_index) => current_next_available_slot_index = other_next_available_slot_index,
			}
		};

		Ok(item)
	}

	#[inline(always)]
	pub(crate) fn deallocate(&self, item_pointer: NonNull<ArenaItem<InnerValue>>)
	{
		let item_pointer_usize = item_pointer.as_ptr() as usize;
		debug_assert!(self.allocation.pointer_usize_is_one_of_ours(item_pointer_usize, self.number_of_items), "item_pointer was not from self BufferArena");

		let item = unsafe { item_pointer.as_mut() };

		item.deallocated();

		let mut current_next_available_slot_index = self.next_available_slot_index.load(Relaxed);
		loop
		{
			item.set_next_available_slot_index(current_next_available_slot_index);

			let item_next_available_slot_index = self.allocation.pointer_usize_to_index(item_pointer_usize);

			match self.next_available_slot_index.compare_exchange(current_next_available_slot_index, item_next_available_slot_index, SeqCst, Relaxed)
			{
				Ok(_) => break,
				Err(other_next_available_slot_index) => current_next_available_slot_index = other_next_available_slot_index,
			}
		}
	}
}
