// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SimpleHttpServedClientConnectionUserSettings
{
	pub(crate) our_hostname: String,

	pub(crate) our_port_string: String,

	pub(crate) our_url: Url,
}

impl SimpleHttpServedClientConnectionUserSettings
{
	#[inline(always)]
	pub fn new(our_hostname: &str, our_port: u16) -> Self
	{
		let our_port_string = format!("{:?}", our_port);
		Self
		{
			our_url: Url::parse(&format!("https://{}:{}", our_hostname, &our_port_string)).unwrap(),
			our_hostname: our_hostname.to_string(),
			our_port_string,
		}
	}
}
