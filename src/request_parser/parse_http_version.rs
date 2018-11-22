// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Only supports parsing of HTTP/1.0 and HTTP/1.1.
///
/// HTTP/0.9 is obsolete.
pub(crate) fn parse_http_version(bytes: &mut Bytes) -> Result<Http1MinorVersion, Status>
{
	use self::Http1MinorVersion::*;

	let reentry_point = bytes.current_pointer;

	bytes.is_slice(b"HTTP/1.", reentry_point, HttpVersionNotSupported)?;

	let minor_version = match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
	{
		b'0' => _0,
		b'1' => _1,
	};

	match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
	{
		b'\r' => match bytes.if_has_more_return_current_value_and_increment(reentry_point)?
		{
			b'\n' => Ok(minor_version),

			_ => Err(Invalid(HttpVersionNotSupported)),
		},

		// NOTE: A concession to a robustness concern from RFC 7230.
		b"\n" => Ok(minor_version),
	}
}
