// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


struct HttpReadBufferUser<HGU: HttpGetUser>
{
	our_hostname: String,
	our_port_string: String,
	our_base_url: Url,
	http_get_user: HGU
}

impl<HGU: HttpGetUser> ReadBufferUser for HttpReadBufferUser<HGU>
{
	type Error = HttpServerReadError<HGU::Error>;

	fn use_read_buffer(&self, constraints: &Constraints, read_buffer: &[u8]) -> Result<(), Self::Error>
	{
	}
}

impl HttpReadBufferUser
{
}
