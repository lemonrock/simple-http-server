// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	pub(crate) fn tcflow(fd: c_int, action: c_int) -> c_int;
}

pub(crate) const TCOOFF: c_int = 1;

pub(crate) const TCOON: c_int = 2;

pub(crate) const TCIOFF: c_int = 3;

pub(crate) const TCION: c_int = 4;
