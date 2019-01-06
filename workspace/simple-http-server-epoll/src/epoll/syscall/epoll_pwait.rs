// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Like `epoll()`, but takes a `sigmask` to atomically change the current pthread's signal mask for the duration of the call to `epoll_wait()`.
	pub(crate) fn epoll_pwait(epfd: RawFd, events: *mut epoll_event, maxevents: c_int, timeout: c_int, sigmask: *const sigset_t) -> c_int;
}
