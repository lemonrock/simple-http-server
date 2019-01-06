// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Linux and BSD extension that sets input baud rate and output baud rate to the same value.
	///
	/// Linux always forces the two baud rates to the same value in any event.
	pub(crate) fn cfsetspeed(termios_p: *mut termios, speed: speed_t) -> c_int;
}
