// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A simple HTTP implementation.
#[derive(Debug)]
pub struct SimpleHttpServedClientConnectionUserFactory
{
	settings: Rc<SimpleHttpServedClientConnectionUserSettings>,
}

impl ServedClientConnectionUserFactory for SimpleHttpServedClientConnectionUserFactory
{
	type SCCU = SimpleHttpServedClientConnectionUser;

	fn new(&self, _remote_address: SocketAddr) -> Result<Self::SCCU, ()>
	{
		Ok(SimpleHttpServedClientConnectionUser::new(&self.settings))
	}
}

impl SimpleHttpServedClientConnectionUserFactory
{
	pub fn new(settings: SimpleHttpServedClientConnectionUserSettings) -> Self
	{
		Self
		{
			settings: Rc::new(settings),
		}
	}
}
