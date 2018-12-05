// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Allocation<Item>(NonNull<Item>);

#[doc(hidden)]
impl<Item> Allocation<Item>
{
	#[doc(hidden)]
	#[inline(always)]
	pub fn allocate(number_of_items: usize) -> Self
	{
		let system = System;
		let allocation = unsafe { system.alloc(Self::layout(number_of_items)) };
		if allocation.is_null()
		{
			panic!("Could not allocate enough memory")
		}
		Allocation(unsafe { NonNull::new_unchecked(allocation as *mut Item) })
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn initialize(self, number_of_items: usize, new_item: impl FnOnce(usize) -> Item)
	{
		assert_ne!(number_of_items, ::std::usize::MAX, "number_of_items can not be ::std::usize::MAX");

		let mut index = 0;
		self.iterate(number_of_items, |item_pointer|
		{
			unsafe { item_pointer.write(Item::new(index)) };
			index += 1;
		})
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn iterate(self, number_of_items: usize, mut callback: impl FnMut(*mut Item))
	{
		let mut item_pointer_usize = self.inclusive_start_pointer_usize();
		let exclusive_end_pointer = self.exclusive_end_pointer_usize(number_of_items);
		
		while item_pointer_usize != exclusive_end_pointer
		{
			callback(item_pointer_usize as *mut Item);
			
			item_pointer_usize += Self::item_size()
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn deallocate(self, number_of_items: usize)
	{
		let system = System;
		unsafe { system.dealloc(self.inclusive_start_pointer() as *mut u8, Self::layout(number_of_items)) }
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn layout(number_of_items: usize) -> Layout
	{
		Layout::from_size_align(Self::size_in_bytes(number_of_items), Self::item_alignment()).unwrap()
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn item_immutable_reference<'a>(self, index: usize) -> &'a Item
	{
		unsafe { &mut * self.item_mutable_pointer(index) }
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn item_immutable_pointer(self, index: usize) -> *const Item
	{
		(self.inclusive_start_pointer_usize() + index * Self::item_size()) as *const Item
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn item_mutable_reference<'a>(self, index: usize) -> &'a mut Item
	{
		unsafe { &mut * self.item_mutable_pointer(index) }
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn item_mutable_pointer(self, index: usize) -> *mut Item
	{
		(self.inclusive_start_pointer_usize() + index * Self::item_size()) as *mut Item
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn exclusive_end_pointer_usize(self, number_of_items: usize) -> usize
	{
		self.inclusive_start_pointer_usize() + Self::size_in_bytes(number_of_items)
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn pointer_usize_is_one_of_ours(self, pointer_usize: usize, number_of_items: usize) -> bool
	{
		self.inclusive_start_pointer_usize() <= pointer_usize && pointer_usize < self.exclusive_end_pointer_usize(number_of_items)
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn pointer_usize_to_index(self, pointer_usize: usize) -> usize
	{
		(pointer_usize - self.inclusive_start_pointer_usize()) / Self::item_size()
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn size_in_bytes(number_of_items: usize) -> usize
	{
		Self::item_size() * number_of_items
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn item_size() -> usize
	{
		size_of::<Item>()
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn item_alignment() -> usize
	{
		align_of::<Item>()
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn inclusive_start_pointer_usize(self) -> usize
	{
		self.inclusive_start_pointer() as usize
	}

	#[doc(hidden)]
	#[inline(always)]
	pub fn inclusive_start_pointer(self) -> *mut Item
	{
		self.0.as_ptr()
	}
}
