// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A ring buffer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RingBuffer<T>
{
	space_currently_available: usize,
	length: usize,

	insert_index: usize,
	remove_index: usize,

	store: ManuallyDrop<[T; BufferIndex::MaximumSize]>,
}

impl<T> Drop for RingBuffer<T>
{
	/// A more efficient drop() than simply relying on the constructor to drop the contents of `store`, and allows the contents of `store` to be uninitialized rather than of type `Option<T>`.
	#[inline(always)]
	fn drop(&mut self)
	{
		let mut remove_index = self.remove_index;
		let mut space_currently_available = self.space_currently_available;
		while space_currently_available != BufferIndex::MaximumSize
		{
			unsafe { drop_in_place(self.get_item_mutably(remove_index)) };
			let next_remove_index = remove_index + 1;
			remove_index = if unlikely!(next_remove_index == BufferIndex::MaximumSize)
			{
				0
			}
			else
			{
				next_remove_index
			};

			space_currently_available += 1
		}
	}
}

impl<T> Default for RingBuffer<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new()
	}
}

impl<T> Index<usize> for RingBuffer<T>
{
	type Output = T;

	#[inline(always)]
	fn index(&self, index: usize) -> &Self::Output
	{
		self.get(index)
	}
}

impl<T> IndexMut<usize> for RingBuffer<T>
{
	#[inline(always)]
	fn index_mut(&mut self, index: usize) -> &mut Self::Output
	{
		self.get_mut(index)
	}
}

impl<T> RingBuffer<T>
{
	/// Creates a new, empty RingBuffer.
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			space_currently_available: BufferIndex::MaximumSize,
			length: 0,

			insert_index: 0,
			remove_index: 0,

			store: unsafe { uninitialized() }
		}
	}

	/// Tries to get an item at the `only_ever_increasing_index`.
	///
	/// Uses a modulus operation and so is not as efficient as `get()`.
	#[inline(always)]
	pub fn get_using_only_ever_increasing_index(&self, only_ever_increasing_index: usize) -> &T
	{
		let absolute_index = self.buffer_index_to_array_index(BufferIndex::from_only_ever_increasing_index(only_ever_increasing_index));
		self.get_item(absolute_index)
	}

	/// Tries to get an item at the `only_ever_increasing_index`.
	///
	/// Uses a modulus operation and so is not as efficient as `get_mut()`.
	#[inline(always)]
	pub fn get_mut_using_only_ever_increasing_index(&self, only_ever_increasing_index: usize) -> &mut T
	{
		let absolute_index = self.buffer_index_to_array_index(BufferIndex::from_only_ever_increasing_index(only_ever_increasing_index));
		self.get_item_mutably(absolute_index)
	}

	/// Tries to get an item at the `buffer_index`, or panics if out-of-range.
	#[inline(always)]
	pub fn get(&self, buffer_index: BufferIndex) -> &T
	{
		assert!(buffer_index <= BufferIndex::Maximum, "buffer_index '{}' exceeds BufferIndex::Maximum '{}'", buffer_index, BufferIndex::Maximum);

		let absolute_index = self.buffer_index_to_array_index(buffer_index);
		self.get_item(absolute_index)
	}

	/// Tries to get an item at the `buffer_index`, or panics if out-of-range.
	#[inline(always)]
	pub fn get_mut(&mut self, buffer_index: BufferIndex) -> &mut T
	{
		assert!(buffer_index <= BufferIndex::Maximum, "buffer_index '{}' exceeds BufferIndex::Maximum '{}'", buffer_index, BufferIndex::Maximum);

		let absolute_index = self.buffer_index_to_array_index(buffer_index);
		self.get_item_mutably(absolute_index)
	}

	#[inline(always)]
	fn buffer_index_to_array_index(&self, index: BufferIndex) -> usize
	{
		let item_index = self.remove_index + index.0;
		if item_index >= BufferIndex::MaximumSize
		{
			item_index - BufferIndex::MaximumSize
		}
		else
		{
			item_index
		}
	}

	/// Is full?
	#[inline(always)]
	pub fn is_full(&self) -> bool
	{
		self.space_currently_available == 0
	}

	/// Is empty?
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		self.space_currently_available == BufferIndex::MaximumSize
	}

	/// Tries to insert an item at the front.
	///
	/// Fails if full.
	#[inline(always)]
	pub fn insert(&mut self, item: T) -> Result<BufferIndex, ()>
	{
		if self.is_full()
		{
			Err(())
		}
		else
		{
			Ok(self.insert_unchecked(item))
		}
	}

	#[inline(always)]
	pub(crate) fn insert_unchecked(&mut self, item: T) -> BufferIndex
	{
		let insert_index = self.insert_index;

		unsafe { write(self.get_item_mutably(insert_index), item) };

		let new_buffer_index = self.length;

		self.space_currently_available -= 1;
		self.length += 1;

		let next_insert_index = insert_index + 1;
		// This should resolve in x86-64 assembler to a conditional move (CMOV) which is usually faster than a modulus (%) operation.
		self.insert_index = if unlikely!(next_insert_index == BufferIndex::MaximumSize)
		{
			0
		}
		else
		{
			next_insert_index
		};

		Ok(new_buffer_index)
	}

	/// Tries to remove an item from the back.
	///
	/// Fails if empty.
	#[inline(always)]
	pub fn remove(&mut self) -> Result<(), ()>
	{
		if self.is_empty()
		{
			Err(())
		}
		else
		{
			self.remove_unchecked();
			Ok(())
		}
	}

	/// Tries to remove an item from the back.
	///
	/// Fails if empty; returns the item otherwise.
	#[inline(always)]
	pub fn remove_undropped(&mut self) -> Result<T, ()>
	{
		if self.is_empty()
		{
			Err(())
		}
		else
		{
			Ok(self.remove_unchecked_and_undropped())
		}
	}

	#[inline(always)]
	pub(crate) fn remove_unchecked(&mut self)
	{
		let remove_index = self.remove_unchecked_internal();
		unsafe { drop_in_place(self.get_item_mutably(remove_index)) }
	}

	#[inline(always)]
	pub(crate) fn remove_unchecked_and_undropped(&mut self) -> T
	{
		let remove_index = self.remove_unchecked_internal();
		unsafe { read(self.store.get_unchecked(remove_index)) }
	}

	#[inline(always)]
	fn remove_unchecked_internal(&mut self) -> usize
	{
		let remove_index = self.remove_index;
		self.space_currently_available += 1;
		self.length -= 1 ;

		let next_remove_index = remove_index + 1;
		self.remove_index = if unlikely!(next_remove_index == BufferIndex::MaximumSize)
		{
			0
		}
		else
		{
			next_remove_index
		};

		remove_index
	}

	/// Space currently available in the buffer.
	#[inline(always)]
	pub fn space_currently_available(&self) -> usize
	{
		self.space_currently_available
	}

	/// Number of items in the buffer.
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		self.length
	}

	#[inline(always)]
	fn get_item(&self, index: usize) -> &T
	{
		unsafe { self.store.get_unchecked(index) }
	}

	#[inline(always)]
	fn get_item_mutably(&mut self, index: usize) -> &mut T
	{
		unsafe { self.store.get_unchecked_mut(index) }
	}
}
