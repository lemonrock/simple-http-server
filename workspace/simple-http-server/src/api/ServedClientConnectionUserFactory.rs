// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Creates client connection users (handlers, services).
pub trait ServedClientConnectionUserFactory
{
	/// Type created.
	type User: ServedClientConnectionUser;

	type Error: error::Error;

	/// Create a new served client connection user.
	///
	/// Called after checking the remote address is permitted and the maximum number of connections has not been reached.
	fn connect(&self, remote_address: SocketAddr) -> Result<Self::User, Self::Error>;

	/// Create a new served client connection user.
	///
	/// Called after the number of connections has been reduced by one.
	fn disconnect(&self, remote_address: SocketAddr);
}
