// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.



use ::std::convert::AsRef;
use ::std::path::Path;


/// Represents socket data.
pub trait SocketData: Sized + Default
{
	/// Socket family (eg `AF_UNIX`).
	#[inline(always)]
	fn family(&self) -> sa_family_t;

	#[doc(hidden)]
	#[inline(always)]
	fn specialized_drop(socket_file_descriptor: &mut SocketFileDescriptor<Self>)
	{
		socket_file_descriptor.0.close()
	}
}
