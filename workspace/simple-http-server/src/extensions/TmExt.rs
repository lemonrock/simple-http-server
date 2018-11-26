// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


pub(crate) trait TmExt
{
	fn three_letter_week_day(&self) -> [u8; 3];

	fn three_letter_month(&self) -> [u8; 3];
}

impl TmExt for Tm
{
	#[inline(always)]
	fn three_letter_week_day(&self) -> [u8; 3]
	{
		match time.tm_wday
		{
			0 => byte_string_to_array_of_length_three(b"Sun"),
			1 => byte_string_to_array_of_length_three(b"Mon"),
			2 => byte_string_to_array_of_length_three(b"Tue"),
			3 => byte_string_to_array_of_length_three(b"Wed"),
			4 => byte_string_to_array_of_length_three(b"Thu"),
			5 => byte_string_to_array_of_length_three(b"Fri"),
			6 => byte_string_to_array_of_length_three(b"Sat"),
		}
	}

	#[inline(always)]
	fn three_letter_month(&self) -> [u8; 3]
	{
		match time.tm_mon
		{
			0 => byte_string_to_array_of_length_three(b"Jan"),
			1 => byte_string_to_array_of_length_three(b"Feb"),
			2 => byte_string_to_array_of_length_three(b"Mar"),
			3 => byte_string_to_array_of_length_three(b"Apr"),
			4 => byte_string_to_array_of_length_three(b"May"),
			5 => byte_string_to_array_of_length_three(b"Jun"),
			6 => byte_string_to_array_of_length_three(b"Jul"),
			7 => byte_string_to_array_of_length_three(b"Aug"),
			8 => byte_string_to_array_of_length_three(b"Sep"),
			9 => byte_string_to_array_of_length_three(b"Oct"),
			10 => byte_string_to_array_of_length_three(b"Nov"),
			11 => byte_string_to_array_of_length_three(b"Dev"),
		}
	}
}
