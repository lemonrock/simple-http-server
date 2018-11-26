// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


trait ReadBufferUser
{
	type Error: error::Error;

	fn wants_more_plain_text(&self) -> bool;

	fn use_read_buffer(&self, constraints: &Constraints, read_buffer: &[u8]) -> Result<(), Self::Error>;
}
