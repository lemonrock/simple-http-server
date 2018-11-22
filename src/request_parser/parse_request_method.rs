// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// `TRACE` and `CONNECT` are not supported.
///
/// WebDAV verbs are also not supported.
#[inline(always)]
pub(crate) fn parse_request_method(bytes: &mut Bytes) -> Result<RequestMethod, Status<NonNull<u8>>>
{
	use self::RequestMethod::Invalid;

	let reentry_point = bytes.current_pointer;

	match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
	{
		b'G' => bytes.is_slice(b"ET ", reentry_point, MethodNotAllowed).map(|_| GET),

		b'H' => bytes.is_slice(b"EAD ", reentry_point, MethodNotAllowed).map(|_| HEAD),

		b'P' => match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
		{
			'A' => bytes.is_slice(b"TCH ", reentry_point, MethodNotAllowed).map(|_| PATCH),

			'O' => bytes.is_slice(b"ST ", reentry_point, MethodNotAllowed).map(|_| POST),

			'U' => bytes.is_slice(b"T ", reentry_point, MethodNotAllowed).map(|_| PUT),

			_ => Err(Invalid(MethodNotAllowed)),
		}

		b'D' => bytes.is_slice(b"ELETE ", reentry_point, MethodNotAllowed).map(|_| DELETE),

		b'O' => bytes.is_slice(b"PTIONS ", reentry_point, MethodNotAllowed).map(|_| OPTIONS),

		_ => Err(Invalid(MethodNotAllowed))
	}
}
