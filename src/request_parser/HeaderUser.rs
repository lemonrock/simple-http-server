// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Respond to parsed headers.
pub trait HeaderUser
{
	/// Use a header field.
	fn use_header_field(&mut self, name_starts_at_inclusive: NonNull<u8>, name_ends_at_exclusive: NonNull<u8>, value_starts_at_inclusive: NonNull<u8>, value_ends_at_exclusive: NonNull<u8>) -> Result<(), ()>;

	/// Headers have finished being parsed.
	///
	/// There may then follow a request body.
	fn finished(&mut self) -> Result<(), ()>;
}
