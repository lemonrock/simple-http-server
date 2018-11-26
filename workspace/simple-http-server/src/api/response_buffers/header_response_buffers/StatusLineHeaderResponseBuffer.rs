// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A status line response buffer with trailing CRLF, eg `HTTP/1.1 200 X`.
///
/// Status message, which is deprecated, is always `X`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct StatusLineHeaderResponseBuffer([u8; StatusLineHeaderResponseBuffer::ArraySize]);

impl Default for StatusLineHeaderResponseBuffer
{
	#[inline(always)]
	fn default() -> Self
	{
		ResponseStatusLineBuffer::Http11Ok
	}
}

impl HeaderResponseBuffer for StatusLineHeaderResponseBuffer
{
	#[inline(always)]
	fn report_buffer(&self, buffer_index: usize, from_offset: usize) -> &[u8]
	{
		debug_assert_eq!(buffer_index, 0, "buffer_index `{}` out of range", buffer_index);
		debug_assert!(from_offset < Self::ArraySize, "from_offset `{}` out of range", from_offset);

		&self.0[from_offset .. ]
	}
}

impl StatusLineHeaderResponseBuffer
{
	const ArraySize: usize = 16;

	pub const Http11Ok: Self = Self::new(200);

	#[inline(always)]
	const fn new(status_code: u16) -> Self
	{
		const fn last_two_digits(status_code: u16) -> u16
		{
			status_code - ((status_code / 100) * 100)
		}

		const fn last_digit(status_code: u16) -> u16
		{
			last_two_digits(status_code) - ((last_two_digits(status_code) / 10) * 10)
		}

		const fn digit_to_ascii(digit: u16) -> u8
		{
			const AsciiZero: u8 = 48;

			digit as u8 + AsciiZero
		}

		const fn digits_to_digit(digits: u16, scalar: u16) -> u16
		{
			digits / scalar
		}

		Self
		(
			[
				b'H', b'T', b'T', b'P', b'/', b'1', b'.', b'1',
				b' ',
				digit_to_ascii(digits_to_digit(status_code, 100)),
				digit_to_ascii(digits_to_digit(last_two_digits(status_code), 10)),
				digit_to_ascii(digits_to_digit(last_digit(status_code), 1)),
				b' ',
				b'X',
				b'\r', b'\n',
			]
		)
	}
}
