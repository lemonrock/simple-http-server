// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
struct BufferHeader
{
	allocation_observer_identifier: Cell<AllocationObserverIdentifier>,
	next_available_slot_index: Cell<usize>,
	reference_count: AtomicUsize,
}

impl BufferHeader
{
	pub(crate) const BufferSize: usize = 8 * 1024 - size_of::<BufferHeader>();

	#[inline(always)]
	fn new(index: usize) -> Self
	{
		Self
		{
			allocation_observer_identifier: Cell::new(0),
			next_available_slot_index:
			{
				assert_ne!(index, ::std::usize::MAX - 1, "index can not be ::std::usize::MAX - 1 otherwise overflow occurs");
				Cell::new(index + 1)
			},
			reference_count: AtomicUsize::new(0),
		}
	}

	#[inline(always)]
	fn allocation_observer_identifier(&self) -> AllocationObserverIdentifier
	{
		self.allocation_observer_identifier.get()
	}

	#[inline(always)]
	fn set_allocation_observer_identifier(&self, allocation_observer_identifier: AllocationObserverIdentifier)
	{
		self.allocation_observer_identifier.set(allocation_observer_identifier)
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
	fn increment_reference_count(&self)
	{
		self.reference_count.fetch_add(1, SeqCst);
	}

	#[inline(always)]
	fn decrement_reference_count(&self) -> bool
	{
		self.reference_count.fetch_sub(1, SeqCst) == 1
	}
}
