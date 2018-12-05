// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum TargetUriReentryPoint
{
	Begin(VectoredBufferOffset),

	SegmentStartsFrom(VectoredBufferOffset, VectoredBufferOffset),

	QueryStartsFrom(VectoredBufferOffset, VectoredBufferOffset),
}

impl TargetUriReentryPoint
{
	/// Support for a server-wide `OPTIONS` request with a target-uri of `*` (RFC 7230 Section 5.3.4) is not implemented as the use of the `OPTIONS` method has a (very slight) potential security vulnerability.
	///
	/// Support for RFC 7230 Section 5.3.3 authority-form target-uris is not implemented because the `CONNECT` method is unsupported.
	///
	/// Support for RFC 7230 5.3.2 absolute-form target-uris is not implemented because we are not a proxy and we do not support clients connecting to us as if we were a proxy.
	/// Additionally, despite being a MUST requirement, the cost of additional processing and validation (checking that the `Host` header, our server details and the authority in the absolute-uri all match) isn't worthwhile.
	pub(crate) fn re_enter(self, bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		use self::Status::*;
		use self::TargetUriReentryPoint::*;

		match self
		{
			Begin(uri_starts_at_inclusive) =>
			{
				bytes.reset(uri_starts_at_inclusive);

				Self::parse_start(bytes, request_user)
			}

			SegmentStartsFrom(uri_starts_at_inclusive, start_at) =>
			{
				bytes.reset(start_at);

				Self::parse_segments(uri_starts_at_inclusive, bytes, request_user)
			}

			QueryStartsFrom(uri_starts_at_inclusive, start_at) =>
			{
				bytes.reset(start_at);

				Self::parse_query(uri_starts_at_inclusive, bytes, request_user)
			}
		}
	}

	#[inline(always)]
	fn check_amount_parsed_is_not_too_great(uri_starts_at_inclusive: VectoredBufferOffset, bytes: &Bytes) -> Result<(), Status<Self>>
	{
		const MaximumAmountToParse: usize = 8 * 1024;

		if bytes.current_pointer.difference(uri_starts_at_inclusive) > MaximumAmountToParse
		{
			Err(Invalid(UriTooLong))
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn parse_start(bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		let uri_starts_at_inclusive = bytes.current_pointer;
		let mut reentry_point = Begin(uri_starts_at_inclusive);

		match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
		{
			b'/' => Self::parse_segments(uri_starts_at_inclusive, bytes, request_user),

			_ => Err(Invalid(BadRequest("Only absolute request target-uris are supported"))),
		}
	}

	#[inline(always)]
	fn parse_segments(uri_starts_at_inclusive: VectoredBufferOffset, bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		use self::Status::Invalid;

		let mut reentry_point = TargetUriReentryPoint::SegmentStartsFrom(uri_starts_at_inclusive, bytes.current_pointer);
		loop
		{
			match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
			{
				b' ' =>
				{
					request_user.use_segment(reentry_point.0, bytes.current_position()).map_err(|invalid_reason| Invalid(invalid_reason))?;
					return request_user.finished().map_err(|invalid_reason| Invalid(invalid_reason))
				}

				b'#' => return Err(Invalid(BadRequest("Fragment in request target-uri"))),

				b'/' =>
				{
					request_user.use_segment(reentry_point.0, bytes.current_position()).map_err(|invalid_reason| Invalid(invalid_reason))?;
					reentry_point = SegmentStartsFrom(uri_starts_at_inclusive, bytes.current_pointer);
				}

				b'?' =>
				{
					request_user.use_segment(reentry_point.0, bytes.current_position()).map_err(|invalid_reason| Invalid(invalid_reason))?;
					return Self::parse_query(uri_starts_at_inclusive, bytes, request_user)
				}

				// RFC 3986, Section 3.3, Path:-
				// ```
				// segment       = *pchar
				// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
				// ```
				//
				// RFC 3986, Section 2.2, Reserved Characters:-
				// ```
				// sub-delims  = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
				// ```
				//
				// RFC 3986, Section 2.3, Unreserved Characters:-
				// ```
				// unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"
				// ```
				//
				// RFC 3986, Section 2.1, Percent-Encoding:-
				// ```
				// pct-encoded = "%" HEXDIG HEXDIG
				// ```
				//
				// RFC 5234 Appendix B.1, Core Rules:-
				//
				// ```
				// ALPHA          =  %x41-5A / %x61-7A   ; A-Z / a-z
				// ...
				// DIGIT          =  %x30-39
				// ```
				// Note that `HEXDIG` can be upper or lower cased and in subsumed by the `ALPHA` and `DIGIT` rules.
				b'A' ... b'Z' | b'a' ... b'z' | b'0' ... b'9' | '-' | b'.' | b'_' | b'~' | b'%' | b'!' | b'$' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b',' | b';' | b'=' | b':' | b'@' => (),

				_ => return Err(Invalid(BadRequest("Invalid byte in request target-uri query token")))
			}

			Self::check_amount_parsed_is_not_too_great(uri_starts_at_inclusive)?
		}
	}

	#[inline(always)]
	fn parse_query(uri_starts_at_inclusive: VectoredBufferOffset, bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		let reentry_point = TargetUriReentryPoint::QueryStartsFrom(uri_starts_at_inclusive, bytes.current_pointer);
		loop
		{
			match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
			{
				b' ' =>
				{
					request_user.use_query(reentry_point.0, bytes.current_position()).map_err(|invalid_reason| Invalid(invalid_reason))?;
					return request_user.finished().map_err(|invalid_reason| Invalid(invalid_reason))
				},

				b'#' => return Err(Invalid(BadRequest("Fragment in request target-uri"))),

				// RFC 3986, Section 3.4, Query:-
				// ```
				// query       = *( pchar / "/" / "?" )
				// ```
				b'A' ... b'Z' | b'a' ... b'z' | b'0' ... b'9' | '-' | b'.' | b'_' | b'~' | b'%' | b'!' | b'$' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b',' | b';' | b'=' | b':' | b'@' | b'/' | b'?' => (),
			}

			Self::check_amount_parsed_is_not_too_great(uri_starts_at_inclusive)?
		}
	}
}
