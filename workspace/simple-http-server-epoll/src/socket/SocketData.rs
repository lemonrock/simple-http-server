// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.



use ::std::convert::AsRef;
use ::std::path::Path;

trait SocketData: Sized
{
	fn family(&self) -> sa_family_t;
}

impl SocketData for sockaddr_in
{
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sin_family
	}
}

impl SocketData for sockaddr_in6
{
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sin6_family
	}
}

impl SocketData for sockaddr_un
{
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sun_family
	}
}
