// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


const UsizeToBytesMaximumLength: usize = 39;

pub(crate) trait VecExt
{
	#[inline(always)]
	fn with_capacity_for_usize(preamble: usize, postamble: usize) -> usize
	{
		preamble + UsizeToBytesMaximumLength + postamble
	}

	fn write_usize(&mut self, n: usize);
}

impl VecExt
{
	fn write_usize(&mut self, mut n: usize)
	{
		const DecimalDigitsLut: &'static[u8] =
		b"\
			0001020304050607080910111213141516171819\
			2021222324252627282930313233343536373839\
			4041424344454647484950515253545556575859\
			6061626364656667686970717273747576777879\
			8081828384858687888990919293949596979899\
		";

		let mut buffer: [u8; UsizeToBytesMaximumLength] = unsafe { uninitialized() };

		let mut current = UsizeToBytesMaximumLength;
		let buffer_pointer = buffer.as_mut_ptr();
		let decimal_digits_lut_pointer = DecimalDigitsLut.as_ptr();

		// eagerly decode 4 characters at a time
		while n >= 10000
		{
			let rem = n % 10000;
			n /= 10000;

			let decimal_digit_index_1 = (rem / 100) << 1;
			let decimal_digit_index_2 = (rem % 100) << 1;
			current -= 4;
			unsafe { copy_nonoverlapping(decimal_digits_lut_pointer.add(decimal_digit_index_1), buffer_pointer.add(current), 2) };
			unsafe { copy_nonoverlapping(decimal_digits_lut_pointer.add(decimal_digit_index_2), buffer_pointer.add(current + 2), 2) };
		}

		// If we reach here then n <= 9999, ie it is at most 4 characters long.

		// decode 2 more characters, if > 2 characters remain.
		if n >= 100
		{
			let decimal_digit_index = (n % 100) << 1;
			n /= 100;
			current -= 2;
			unsafe { copy_nonoverlapping(decimal_digits_lut_pointer.add(decimal_digit_index), buffer_pointer.add(current), 2) };
		}

		// Decode last 1 or 2 characters.
		if n < 10
		{
			current -= 1;
			unsafe { *buffer_pointer.add(current) = (n as u8) + b'0' };
		}
		else
		{
			let decimal_digit_index = n << 1;
			current -= 2;
			unsafe { copy_nonoverlapping(decimal_digits_lut_pointer.add(decimal_digit_index), buffer_pointer.add(current), 2) };
		}

		let start_pointer = unsafe { buffer_pointer.add(current) };
		let actual_length = MaximumLength - current as usize;

		insert_into.extend_from_slice(&buffer[current ..]);
	}
}
