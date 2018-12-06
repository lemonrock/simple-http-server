// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Whilst this is present in libc, it is not always consistently defined.
#[repr(C)]
pub(crate) struct sockaddr_storage
{
	/// Socket address family.
	pub(crate) ss_family: sa_family_t,

	/// Alignment.
	__ss_align: size_t,

	/// Padding to 128 bytes.
	__ss_pad2: [u8; 128 - size_of::<sa_family_t>() - size_of::<size_t>()],
}
