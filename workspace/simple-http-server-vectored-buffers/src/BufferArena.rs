// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A `BufferArena` holds enough memory for all the buffers that need allocating (it can be thought of as pre-allocating).
///
/// Consequently, buffer allocation and deallocation is very cheap (and so very fast).
///
/// References to buffers use a `BufferReference`, which is logically similar to a `Rc<Buffer>`, but without any need to allocate the `Rc` on additional heap memory.
///
/// Currently, because Rust does not permit generic array parameters (or the construction of variably sized structs), it is not possible to modify the size of buffers except at compile time.
#[derive(Debug)]
pub struct BufferArena<AO: AllocationObserver>
{
	allocation_observer: AO,

	number_of_buffers: usize,

	/// Is equal to number_of_buffers when full.
	next_available_slot_index: AtomicUsize,

	allocation: Allocation,
}

impl<AO: AllocationObserver> Drop for BufferArena<AO>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.allocation.deallocate(self.number_of_buffers)
	}
}

impl<AO: AllocationObserver> BufferArena<AO>
{
	/// Create a new instance for `number_of_buffers`.
	///
	/// Allocations (and deallocations) are observed by `allocation_observer`, who can intervene and prevent an allocation being made.
	#[inline(always)]
	pub fn with_capacity(allocation_observer: AO, number_of_buffers: usize) -> Arc<Self>
	{
		let allocation = Allocation::allocate(number_of_buffers);
		allocation.initialize(number_of_buffers, Buffer::new);

		Arc::new
		(
			Self
			{
				allocation_observer,
				number_of_buffers,
				next_available_slot_index: 0,
				allocation,
			}
		)
	}

	/// Allocate a buffer.
	///
	/// Returns `None` if full or the allocation observer did not permit allocation for `allocation_observer_identifier`.
	#[inline(always)]
	pub(crate) fn allocate(this: &Arc<Self>, allocation_observer_identifier: AllocationObserverIdentifier) -> Option<BufferReference>
	{
		if this.allocation_observer.observe_and_veto_forthcoming_allocation(allocation_observer_identifier)
		{
			return None
		}

		let mut current_next_available_slot_index = this.next_available_slot_index.load(Relaxed);
		let buffer = loop
		{
			let is_full = current_next_available_slot_index == this.number_of_buffers;

			if is_full
			{
				return None
			}

			let buffer = this.allocation.buffer_immutable_reference(current_next_available_slot_index);

			let next_available_slot_index = buffer.next_available_slot_index();

			match this.next_available_slot_index.compare_exchange(current_next_available_slot_index, next_available_slot_index, SeqCst, Relaxed)
			{
				Ok(_) => break buffer,
				Err(other_next_available_slot_index) => current_next_available_slot_index = other_next_available_slot_index,
			}
		};

		buffer.increment_reference_count();

		buffer.set_allocation_observer_identifier(allocation_observer_identifier);

		Some(BufferReference::reused(buffer, this))
	}

	#[inline(always)]
	pub(crate) fn deallocate(this: Arc<Self>, item_pointer: NonNull<Buffer>)
	{
		let item_pointer_usize = item_pointer.as_ptr() as usize;
		debug_assert!(this.allocation.pointer_usize_is_one_of_ours(item_pointer_usize, this.number_of_buffers), "item_pointer was not from this BufferArena");

		let buffer = unsafe { item_pointer.as_ref() };

		this.allocation_observer.observe_forthcoming_deallocation(buffer.allocation_observer_identifier());

		let mut current_next_available_slot_index = this.next_available_slot_index.load(Relaxed);
		loop
		{
			buffer.set_next_available_slot_index(current_next_available_slot_index);

			let item_next_available_slot_index = this.allocation.pointer_usize_to_index(item_pointer_usize);

			match this.next_available_slot_index.compare_exchange(current_next_available_slot_index, item_next_available_slot_index, SeqCst, Relaxed)
			{
				Ok(_) => break,
				Err(other_next_available_slot_index) => current_next_available_slot_index = other_next_available_slot_index,
			}
		}
	}
	
	/*
		pub trait ArenaItem
		{
			#[inline(always)]
			fn next_available_slot_index(&self) -> usize;

			#[inline(always)]
			fn set_next_available_slot_index(&self, next_available_slot_index: usize);

			#[inline(always)]
			fn increment_reference_count(&self);

			#[inline(always)]
			fn decrement_reference_count(&self) -> bool;
		}

	*/
}
