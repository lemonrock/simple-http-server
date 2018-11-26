// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Tag used in `X-Robots-Tag` header.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum RobotTag
{
	/// There are no restrictions for indexing or serving.
	/// This directive is the default value and has no effect if explicitly listed.
	all,

	/// Do not show this page in search results and do not show a "Cached" link in search results.
	noindex,

	/// Do not follow the links on this page.
	nofollow,

	/// Equivalent to `noindex, nofollow`.
	none,

	/// Do not show a "Cached" link in search results.
	noarchive,

	/// Do not show a text snippet or video preview in the search results for this page.
	/// A static thumbnail (if available) will still be visible.
	nosnippet,

	/// Do not offer translation of this page in search results.
	notranslate,

	/// Do not index images on this page.
	noimageindex,

	/// Unavailable after.
	///
	/// Whilst Google states with should be RFC-850 formatted, not even their own [examples](https://developers.google.com/search/reference/robots_meta_tag) follow that format.
	unavailable_after(Tm),
}

impl PartialEq for RobotTag
{
	fn eq(&self, other: &Self) -> bool
	{
		use self::RobotTag::*;

		match (self, other)
		{
			(all, all) => true,

			(noindex, noindex) => true,

			(nofollow, nofollow) => true,

			(none, none) => true,

			(noarchive, noarchive) => true,

			(nosnippet, nosnippet) => true,

			(notranslate, notranslate) => true,

			(noimageindex, noimageindex) => true,

			(unavailable_after(_), unavailable_after(_)) => true,

			(_, _) => false,
		}
	}
}

impl Eq for RobotTag
{
}

impl RobotTag
{
	#[inline(always)]
	fn buffer(&self) -> Cow<'static, [u8]>
	{
		use self::RobotTag::*;
		use self::Cow::*;

		match *self
		{
			all => Borrowed(b"all"),

			noindex => Borrowed(b"noindex"),

			nofollow => Borrowed(b"nofollow"),

			none => Borrowed(b"none"),

			noarchive => Borrowed(b"noarchive"),

			nosnippet => Borrowed(b"nosnippet"),

			notranslate => Borrowed(b"notranslate"),

			noimageindex => Borrowed(b"noimageindex"),

			unavailable_after(ref time) =>
			{
				// Examples seen elsewhere:-
				//
				// 25 Jun 2010 15:00:00 PST
				// 23-Jul-2007 18:00:00 EST
				// 7 Jul 2007 16:30:00 GMT

				let time = time.to_utc();

				let month = time.three_letter_month();

				let two_digit_year = three_digits_to_two_digits(four_digits_to_three_digits(tm.tm_year));

				let array = vec!
				[
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
				];

				Owned(array)
			},
		}
	}

	#[inline(always)]
	fn with_leading_comma_buffer(&self) -> Cow<'static, [u8]>
	{
		use self::RobotTag::*;
		use self::Cow::*;

		match *self
		{
			all => Borrowed(b",all"),

			noindex => Borrowed(b",noindex"),

			nofollow => Borrowed(b",nofollow"),

			none => Borrowed(b",none"),

			noarchive => Borrowed(b",noarchive"),

			nosnippet => Borrowed(b",nosnippet"),

			notranslate => Borrowed(b",notranslate"),

			noimageindex => Borrowed(b",noimageindex"),

			unavailable_after(ref time) =>
			{
				// Examples seen elsewhere:-
				//
				// 25 Jun 2010 15:00:00 PST
				// 23-Jul-2007 18:00:00 EST
				// 7 Jul 2007 16:30:00 GMT

				let time = time.to_utc();

				let month = time.three_letter_month();

				let two_digit_year = three_digits_to_two_digits(four_digits_to_three_digits(tm.tm_year));

				let array = vec!
				[
					b',',
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
				];

				Owned(array)
			},
		}
	}
}
