// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


pub struct Request<RU: RequestUser>
{
	bytes: Bytes,
	state: RequestState,
	request_user: RU,
}

impl<RU: RequestUser> Request<RU>
{
	pub fn re_enter(&mut self)
	{
		match self.state.re_enter(&mut self.bytes, &mut self.request_user)
		{
			Ok(()) => (),

			Err(status) => match status
			{

			}
		}
	}

}





enum RequestState
{
	RequestMethod(VectoredBufferOffset),

	TargetUri(TargetUriReentryPoint),

	HttpVersion(VectoredBufferOffset),

	Headers(HeaderReentryPoint),
}

impl RequestState
{
	pub(crate) fn re_enter(&self, bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		use self::RequestState::*;
		use self::Status::*;

		match *self
		{
			RequestMethod(starts_at) =>
			{
				bytes.reset(starts_at);

				Self::parse_request_method(bytes, request_user)
			}

			TargetUri(reentry_point) => Self::parse_target_uri(bytes, request_user, reentry_point),

			HttpVersion(starts_at) =>
			{
				bytes.reset(starts_at);

				Self::parse_http_version(bytes, request_user)
			}

			Headers(reentry_point) => Self::parse_headers(bytes, request_user, reentry_point),
		}
	}

	fn parse_request_method(bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		use self::RequestMethod::*;

		let reentry_point = RequestState::RequestMethod(bytes.current_pointer);

		let request_method = match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
		{
			b'G' => bytes.is_slice(b"ET ", reentry_point, MethodNotAllowed, GET),

			b'H' => bytes.is_slice(b"EAD ", reentry_point, MethodNotAllowed, HEAD),

			b'P' => match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
			{
				'A' => bytes.is_slice(b"TCH ", reentry_point, MethodNotAllowed, PATCH),

				'O' => bytes.is_slice(b"ST ", reentry_point, MethodNotAllowed, POST),

				'U' => bytes.is_slice(b"T ", reentry_point, MethodNotAllowed, PUT),

				_ => return Err(Invalid(MethodNotAllowed)),
			}

			b'D' => bytes.is_slice(b"ELETE ", reentry_point, MethodNotAllowed, DELETE),

			b'O' => bytes.is_slice(b"PTIONS ", reentry_point, MethodNotAllowed, OPTIONS),

			_ => return Err(Invalid(MethodNotAllowed))
		}?;

		request_user.method(request_method).map_err(|invalid_reason| Invalid(invalid_reason))?;

		Self::parse_target_uri(bytes, request_user, TargetUriReentryPoint::Begin(bytes.current_pointer))
	}

	#[inline(always)]
	fn parse_target_uri(bytes: &mut Bytes, request_user: &mut impl RequestUser, reentry_point: TargetUriReentryPoint) -> Result<(), Status<Self>>
	{
		reentry_point.re_enter(bytes, request_user).map_err(|status| match status
		{
			Invalid(invalid_reason) => Invalid(invalid_reason),
			RanOutOfBytes(reentry_point) => RanOutOfBytes(RequestState::TargetUri(reentry_point)),
		})?;

		Self::parse_http_version(bytes, request_user)
	}

	/// Only supports parsing of HTTP/1.1.
	///
	/// Why?
	///
	/// * HTTP/1.0 is obsolescent and complicates Host and header handling for very little gain.
	/// * HTTP/0.9 is extremely obsolete.
	fn parse_http_version(bytes: &mut Bytes, request_user: &mut impl RequestUser) -> Result<(), Status<Self>>
	{
		let reentry_point = RequestState::HttpVersion(bytes.current_pointer);

		bytes.is_slice(b"HTTP/1.1", reentry_point, HttpVersionNotSupported, ())?;

		match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
		{
			b'\r' => match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
			{
				b'\n' => (),

				_ => return Err(Invalid(HttpVersionNotSupported)),
			},

			// NOTE: A concession to a robustness concern from RFC 7230.
			b"\n" => (),

			_ => return Err(Invalid(BadRequest(b"Status line not terminated correctly"))),
		}

		request_user.status_line_finished().map_err(|invalid_reason| Invalid(invalid_reason))?;

		Self::parse_headers(bytes, request_user, HeaderReentryPoint::Begin(bytes.current_pointer))
	}

	fn parse_headers(bytes: &mut Bytes, request_user: &mut impl RequestUser, reentry_point: HeaderReentryPoint) -> Result<(), Status<Self>>
	{
		reentry_point.re_enter(bytes, request_user).map_err(|status| match status
		{
			Invalid(invalid_reason) => Invalid(invalid_reason),
			RanOutOfBytes(reentry_point) => RanOutOfBytes(RequestState::Headers(reentry_point)),
		})?;

		Ok(())
	}
}

/// An user of request handling events as they occur during parsing of a request.
///
/// The order of events is:-
///
/// * `method()`.
/// * `target_uri_segment()` (at least once, and possible several times for each segment in the request's target-uri).
/// * `target_uri_query()` (either once or not at all).
/// * `target_uri_finished()`
/// * (No event for HTTP version; the HTTP version is always assumed to be 1.1 currently).
/// * `status_line_finished()`
/// * `header_field()` (may be called once, many times or not at all (although in that event, the HTTP/1.1 request is invalid as a `Host:` header should have been provided).
/// * `headers_finished()`
pub trait RequestUser
{
	/// Called exactly once.
	///
	/// Arbitarary request methods are not supported.
	///
	/// Of the well-known request methods, the following are not supported:-
	///
	/// * `CONNECT` (we are not a proxy; proxies are evil in any event).
	/// * `TRACE` (a slight security risk).
	///
	/// Additionally, the request method `OPTIONS` will never be followed by a request target-uri of `*` as it can be used to 'scope out' a server for attack.
	fn method(&mut self, request_method: RequestMethod) -> Result<(), InvalidReason>;

	/// Will be called at least once.
	///
	/// Will be followed by either a call to `target_uri_query()` or to `target_uri_finished()`.
	///
	/// Absolute, authority-only and `*` request-uris are not supported.
	///
	/// No percent-decoding is performed but values are checked for byte validity.
	///
	/// A decoder of the segment could assume that it is %-encoded UTF-8 string.
	fn target_uri_segment(&mut self, segment_starts_at_inclusive: VectoredBufferOffset, segment_ends_at_exclusive: VectoredBufferOffset) -> Result<(), InvalidReason>;

	/// Will only be called either never or once (will not be called if there is no query).
	///
	/// Will be called if there is an empty query (a query of zero length).
	///
	/// Will be followed by a call to `uri_target_finished()`.
	///
	/// No percent-decoding is performed but values are checked for byte validity.
	///
	/// A decoder of the query could assume that it is %-encoded UTF-8 string.
	fn target_uri_query(&mut self, query_starts_at_inclusive: VectoredBufferOffset, query_ends_at_exclusive: VectoredBufferOffset) -> Result<(), InvalidReason>;

	/// Will be called exactly once.
	///
	/// Called either after the final call to `target_uri_segment()` or after the call to `target_uri_query()`.
	fn target_uri_finished(&mut self) -> Result<(), InvalidReason>;

	/// Will be called exactly once.
	///
	/// Called after the call to `target_uri_finished()` and after successful parsing of the HTTP version (only HTTP/1.1 is supported).
	fn status_line_finished(&mut self) -> Result<(), InvalidReason>;

	/// Will be called for each header encountered.
	///
	/// May never be called.
	///
	/// Header field names and values are checked for token validity.
	fn header_field(&mut self, name_starts_at_inclusive: VectoredBufferOffset, name_ends_at_exclusive: VectoredBufferOffset, value_starts_at_inclusive: VectoredBufferOffset, value_ends_at_exclusive: VectoredBufferOffset) -> Result<(), ()>;

	/// Will be called once headers have been parsed and before any request body is examined.
	///
	/// There may then follow a request body.
	fn headers_finished(&mut self) -> Result<(), ()>;
}
