// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


pub(crate) trait ServedClientConnectionUser
{
	/// Errors returned by `service()`.
	type Error: error::Error;

	/// Called when an event occurs that a previous `RegistrationState` expressed an interest in.
	///
	/// Return an empty RegistrationState for the connection to be dropped.
	fn service(&mut self, socket: &TcpStream) -> Result<RegistrationState, Self::Error>;
}
