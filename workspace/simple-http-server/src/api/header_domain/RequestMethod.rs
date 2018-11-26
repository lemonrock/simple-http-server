// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Request method.
#[deny(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequestMethod
{
	GET,

	HEAD,

	POST,

	PUT,

	DELETE,

	/// Support for a server-wide `OPTIONS` request with a target-uri of `*` (RFC 7230 Section 5.3.4) is not supported as it is a (very slight) potential security vulnerability.
	///
	/// Support for CORS OPTIONS requests is still possible.
	OPTIONS,

	PATCH,
}

impl RequestMethod
{
	#[inline(always)]
	fn buffer(self) -> &'static [u8]
	{
		use self::RequestMethod::*;

		match self
		{
			GET => b"GET",

			HEAD => b"HEAD",

			POST => b"POST",

			PUT => b"PUT",

			DELETE => b"DELETE",

			CONNECT => b"CONNECT",

			OPTIONS => b"OPTIONS",

			TRACE => b"TRACE",

			PATCH => b"PATCH",
		}
	}

	#[inline(always)]
	fn with_leading_comma_buffer(self) -> &'static [u8]
	{
		use self::RequestMethod::*;

		match self
		{
			GET => b",GET",

			HEAD => b",HEAD",

			POST => b",POST",

			PUT => b",PUT",

			DELETE => b",DELETE",

			CONNECT => b",CONNECT",

			OPTIONS => b",OPTIONS",

			TRACE => b",TRACE",

			PATCH => b",PATCH",
		}
	}
}
