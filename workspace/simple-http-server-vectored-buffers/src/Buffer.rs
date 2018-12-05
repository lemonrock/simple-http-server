// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
struct Buffer
{
	header: BufferHeader,
	bytes: RefCell<[u8; BufferHeader::BufferSize]>,
}

impl<'b> Buffer
{
	#[inline(always)]
	fn borrow(&'b self) -> Ref<'b, [u8]>
	{
		let borrow = self.bytes.borrow();
		Ref::map(borrow, |borrow| &borrow[..])
	}

	#[inline(always)]
	fn try_borrow(&'b self) -> Result<Ref<'b, [u8]>, BorrowError>
	{
		let borrow = self.bytes.try_borrow()?;
		Ok(Ref::map(borrow, |borrow| &borrow[..]))
	}

	#[inline(always)]
	fn borrow_mut(&'b self) -> RefMut<'b, [u8]>
	{
		let borrow_mut = self.bytes.borrow_mut();
		RefMut::map(borrow_mut, |borrow_mut| &mut borrow_mut[..])
	}

	#[inline(always)]
	fn try_borrow_mut(&'b self) -> Result<RefMut<'b, [u8]>, BorrowMutError>
	{
		let borrow_mut = self.bytes.try_borrow_mut()?;
		Ok(RefMut::map(borrow_mut, |borrow_mut| &mut borrow_mut[..]))
	}
}

impl Buffer
{
	#[inline(always)]
	fn new(index: usize) -> Self
	{
		Self
		{
			header: BufferHeader::new(index),
			bytes: UnsafeCell::new(unsafe { uninitialized() }),
		}
	}

	#[inline(always)]
	fn next_available_slot_index(&self) -> usize
	{
		self.header.next_available_slot_index()
	}

	#[inline(always)]
	fn set_next_available_slot_index(&self, next_available_slot_index: usize)
	{
		self.header.set_next_available_slot_index(next_available_slot_index)
	}

	#[inline(always)]
	fn allocation_observer_identifier(&self) -> AllocationObserverIdentifier
	{
		self.header.allocation_observer_identifier()
	}

	#[inline(always)]
	fn set_allocation_observer_identifier(&self, allocation_observer_identifier: AllocationObserverIdentifier)
	{
		self.header.set_allocation_observer_identifier(next_available_slot_index)
	}

	#[inline(always)]
	fn increment_reference_count(&self)
	{
		self.header.increment_reference_count();
	}

	#[inline(always)]
	fn decrement_reference_count(&self) -> bool
	{
		self.header.decrement_reference_count()
	}
}
