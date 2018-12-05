// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Assumes a mio token is also a tagged pointer.
pub(crate) trait TokenExt
{
	// With a little more hacking, we can use bit 47, too, as long as we know whether we're in user space or kernel space.
	#[cfg(target_pointer_width = 64)] const TopBitsMask: usize = 0xFFFF << 48;
	#[cfg(not(target_pointer_width = 64))] const TopBitsMask: usize = 0;

	#[cfg(target_pointer_width = 64)] const BottomBitsMask: usize = 0b111;
	#[cfg(target_pointer_width = 32)] const BottomBitsMask: usize = 0b011;
	#[cfg(target_pointer_width = 16)] const BottomBitsMask: usize = 0b001;

	#[inline(always)]
	fn as_box<T>(self) -> Box<T>
	{
		unsafe { Box::from_raw(self.as_raw_mutable_pointer::<T>()) }
	}

	#[inline(always)]
	fn as_ref<'a, T: 'a>(self) -> &'a T
	{
		unsafe { & * self.as_non_null_pointer::<T>().as_ptr() }
	}

	#[inline(always)]
	fn as_mut<'a, T: 'a>(self) -> &'a mut T
	{
		unsafe { & mut * self.as_non_null_pointer::<T>().as_ptr() }
	}

	#[inline(always)]
	fn as_non_null_pointer<T>(self) -> NonNull<T>
	{
		let raw_mutable_pointer = self.as_raw_mutable_pointer();
		debug_assert!(!raw_mutable_pointer.is_null(), "Should not be a null pointer");
		unsafe { NonNull::new_unchecked(raw_mutable_pointer) }
	}

	#[inline(always)]
	fn as_raw_immutable_pointer<T>(self) -> *const T
	{
		self.as_raw_mutable_pointer() as *const T
	}

	#[inline(always)]
	fn as_raw_mutable_pointer<T>(self) -> *mut T
	{
		const CleanPointerMask: usize = !(Self::TopBitsMask | Self::BottomBitsMask);

		(self.tagged_pointer() & CleanPointerMask) as *mut T
	}

	#[inline(always)]
	fn tag_enum<E: From<u8>>(self) -> E
	{
		E::from(self.tag())
	}

	/// On a 64-bit platform, returns the bottom 3 bits.
	///
	/// On a 32-bit platform returns the bottom 2 bits.
	///
	/// On a 16-bit platform returns the bottom bit.
	#[inline(always)]
	fn tag(self) -> u8
	{
		(self.tagged_pointer() & Self::BottomBitsMask) as u8
	}

	fn tagged_pointer(self) -> usize;
}

impl TokenExt for Token
{
	#[inline(always)]
	fn tagged_pointer(self) -> usize
	{
		self.0 as usize
	}
}
