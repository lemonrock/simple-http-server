// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InvalidReason
{
	/// 400 Bad Request
	BadRequest(&'static str),

	/// 405 Method Not Allowed
	MethodNotAllowed,

	/// 414 URI Too Long
	UriTooLong,

	/// 431 Request Header Fields Too Large
	RequestHeaderFieldsTooLarge,

	/// 505 HTTP Version Not Supported
	HttpVersionNotSupported,


// 408 Request Timeout
// 411 Length Required
// 413 Payload Too Large
// 415 Unsupported Media Type

// ?421 Misdirected Request
}
