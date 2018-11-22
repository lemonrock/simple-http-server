// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A `Last-Modified` header response buffer.
pub struct LastModifiedHeaderResponseBuffer([u8; LastModifiedHeaderResponseBuffer::ArraySize]);

impl Default for LastModifiedHeaderResponseBuffer
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::now()
	}
}

impl HeaderResponseBuffer for LastModifiedHeaderResponseBuffer
{
	#[inline(always)]
	fn report_buffer(&self, buffer_index: usize, from_offset: usize) -> &[u8]
	{
		debug_assert_eq!(buffer_index, 0, "buffer_index `{}` out of range", buffer_index);
		debug_assert!(from_offset < Self::ArraySize, "from_offset `{}` out of range", from_offset);

		&self.0[from_offset .. ]
	}
}

impl LastModifiedHeaderResponseBuffer
{
	const ArraySize: usize = 45;

	/// Assumes the provided time is UTC.
	#[inline(always)]
	pub fn for_time(time: Tm) -> Self
	{
		let week_day = time.three_letter_week_day();

		let month = time.three_letter_month();

		let two_digit_year = three_digits_to_two_digits(four_digits_to_three_digits(tm.tm_year));

		// Last-Modified:<day-name>, <day> <month> <year> <hour>:<minute>:<second> GMT
		// eg Date:Tue, 15 Nov 1994 08:12:31 GMT

		LastModifiedHeaderResponseBuffer
		(
			[
				b'L', b'a', b's', b't', b'-', b'M', b'o', b'd', b'i', b'f', b'i', b'e', b'd', b':',
				week_day[0], week_day[1], week_day[2],
				b',',
				b' ',
				two_digits_to_first_digit(time.tm_mday), two_digits_to_second_digit(time.tm_mday),
				b' ',
				month[0], month[1], month[2],
				b' ',
				four_digits_to_first_digit(tm.tm_year), four_digits_to_second_digit(tm.tm_year), two_digits_to_first_digit(two_digit_year), two_digits_to_second_digit(two_digit_year),
				b' ',
				two_digits_to_first_digit(time.tm_hour), two_digits_to_second_digit(time.tm_hour),
				b':',
				two_digits_to_first_digit(time.tm_min), two_digits_to_second_digit(time.tm_min),
				b':',
				// NOTE: tm_sec can have the value 60 due to leap seconds.
				two_digits_to_first_digit(time.tm_sec), two_digits_to_second_digit(time.tm_sec),
				b' ',
				b'G', b'M', b'T',
				b'\r', b'\n',
			]
		)
	}
}
