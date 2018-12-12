// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


extern "C"
{
	/// Retrieve the attributes of the message queue referred to by the message queue descriptor `mqdes`.
	///
	/// On success `mq_getattr()` returns `0`
	/// On error, `-1` is returned, with `errno` set to indicate the error.
	///
	/// Errors documented to be returned from `mq_getattr()` in `errno`:-
	///
	/// * `EBADF`: The message queue descriptor specified in `mqdes` is invalid.
	/// * `EINVAL`: `newattr.mq_flags` contained set bits other than `O_NONBLOCK`.
	pub(crate) fn mq_getattr(mqdes: mqd_t, attr: *mut mq_attr) -> c_int;
}
