// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum HeaderReentryPoint
{
	Begin(VectoredBufferOffset),

	HeaderNameEnds(VectoredBufferOffset, VectoredBufferOffset),

	HeaderValueStarts(VectoredBufferOffset, VectoredBufferOffset, VectoredBufferOffset),
}

impl HeaderReentryPoint
{
	// RFC 7230, Section 3.2, Header Fields:-
	// ```
	// header-field   = field-name ":" OWS field-value OWS
	//
	// ...
	//
	// field-value    = *( field-content / obs-fold )
	// field-content  = field-vchar [ 1*( SP / HTAB ) field-vchar ]
	// field-vchar    = VCHAR / obs-text
	// ```
	//
	// RFC 5234, Appendix B.1:-
	// ```
	// VCHAR          = %x21-7E; visible (printing) characters
	// ```
	//
	// RFC 7230, Appendix B, Collected ABNF:-
	// ```
	// obs-text       = %x80-FF
	// ```
	//
	// RFC 7230, Section 3.2.3, Whitespace:-
	// ```
	// OWS            = *( SP / HTAB )
	// ```
	fn parse(bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		while
		{
			Self::parse_header_field(bytes, request_user)?
		}
		{
		}

		request_user.headers_finished()
	}

	pub(crate) fn re_enter(self, bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		use self::HeaderReentryPoint::*;

		match self
		{
			Begin(starts_at) =>
			{
				bytes.reset(starts_at);

				if Self::parse_header_field(bytes, request_user)?
				{
					return Ok(())
				}
			}

			HeaderNameEnds(name_starts_at_inclusive, name_ends_at_exclusive) =>
			{
				bytes.reset(name_ends_at_exclusive);

				if Self::parse_after_header_name(bytes, request_user, name_starts_at_inclusive, name_ends_at_exclusive)?
				{
					return Ok(())
				}
			}

			HeaderValueStarts(name_starts_at_inclusive, name_ends_at_exclusive, value_starts_at_inclusive) =>
			{
				bytes.reset(value_starts_at_inclusive);

				if Self::parse_header_value(bytes, request_user, name_starts_at_inclusive, name_ends_at_exclusive, value_starts_at_inclusive)?
				{
					return Ok(())
				}
			}
		}

		Self::parse(bytes, request_user)
	}

	#[inline(always)]
	fn check_amount_parsed_is_not_too_great(name_starts_at_inclusive: VectoredBufferOffset, bytes: &Bytes) -> Result<(), Status<Self>>
	{
		const MaximumAmountToParse: usize = 1024;

		if bytes.current_pointer.difference(name_starts_at_inclusive) > MaximumAmountToParse
		{
			Err(Invalid(RequestHeaderFieldsTooLarge))
		}
		else
		{
			Ok(())
		}
	}

	fn parse_header_field(bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<bool, Status<Self>>
	{
		let name_starts_at_inclusive = bytes.parsing_starts_at;
		let reentry_point = HeaderReentryPoint::Begin(name_starts_at_inclusive);
		let name_ends_at_exclusive = loop
		{
			match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
			{
				b'\r' => match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
				{
					b'\n' => return Ok(false),

					_ => return Err(Invalid(BadRequest("Header field terminating CR not followed by LF")))
				}

				// NOTE: A concession to a robustness concern from RFC 7230.
				b"\n" => return Ok(false),

				'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~' | b'0' ... b'9' | b'A' ... b'Z' | b'a' ... b'z' => (),

				b':' => break bytes.current_pointer,

				_ => return Err(Invalid(BadRequest("Invalid byte in header name token"))),
			}

			Self::check_amount_parsed_is_not_too_great(name_starts_at_inclusive, bytes)?
		};

		Self::parse_after_header_name(bytes, request_user, name_starts_at_inclusive, name_ends_at_exclusive)
	}

	fn parse_after_header_name(bytes: &mut Bytes, request_user: &mut impl RequestUser, name_starts_at_inclusive: VectoredBufferOffset, name_ends_at_exclusive: VectoredBufferOffset) -> Result<bool, Status<Self>>
	{
		let name_starts_at_inclusive = bytes.current_pointer;

		let reentry_point = HeaderReentryPoint::HeaderNameEnds(name_starts_at_inclusive, name_ends_at_exclusive);
		let value_starts_at_inclusive = loop
		{
			match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
			{
				b' ' | b'\t' => continue,

				0x00 ... 0x08 | 0x0A ... 0x1F | 0x7F => return Err(Invalid(BadRequest("Invalid byte in header value token"))),

				_ => break bytes.previous_position(),
			}

			Self::check_amount_parsed_is_not_too_great(name_starts_at_inclusive)?
		};

		bytes.reset(value_starts_at_inclusive);

		Self::parse_header_value(bytes, request_user, name_starts_at_inclusive, name_ends_at_exclusive, value_starts_at_inclusive)
	}

	fn parse_header_value(bytes: &mut Bytes, request_user: &mut impl RequestUser, name_starts_at_inclusive: VectoredBufferOffset, name_ends_at_exclusive: VectoredBufferOffset, value_starts_at_inclusive: VectoredBufferOffset) -> Result<bool, Status<Self>>
	{
		let reentry_point = HeaderReentryPoint::HeaderValueStarts(name_starts_at_inclusive, name_ends_at_exclusive, value_starts_at_inclusive);
		let mut last_whitespace_started_at_exclusive = value_starts_at_inclusive.current_pointer.next();
		loop
		{
			match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
			{
				b'\r' => match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
				{
					b'\n' =>
					{
						last_whitespace_started_at_exclusive;
						request_user.header_field(name_starts_at_inclusive, name_ends_at_exclusive, value_starts_at_inclusive, last_whitespace_started_at_exclusive).map_err(|invalid_reason| Invalid(invalid_reason))?;

						return Ok(true)
					}

					_ => return Err(Invalid(BadRequest("Header field value CR not followed by LF"))),
				},

				// NOTE: A concession to a robustness concern from RFC 7230.
				b"\n" =>
				{
					request_user.finished();
					Ok(false)
				},

				b' ' | b'\t' =>
				{
					last_whitespace_started_at_exclusive = bytes.current_pointer;
				}

				0x00 ... 0x08 | 0x0A ... 0x1F | 0x7F => return Err(Invalid(BadRequest("Invalid byte in header value token"))),

				_ => (),
			}

			Self::check_amount_parsed_is_not_too_great(name_starts_at_inclusive)?
		}
	}
}
