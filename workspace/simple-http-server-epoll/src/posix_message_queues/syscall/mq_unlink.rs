// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `mq_unlink()` removes the specified message queue name `name`.
	///
	/// The message queue name `name` is removed immediately.
	/// The queue itself is destroyed once any other processes that have the queue open close their descriptors referring to the queue.
	///
	/// Errors documented to be returned from `mq_unlink()` in `errno`:-
	///
	/// * `EACCES`: The caller does not have permission to unlink this message queue.
	/// * `ENAMETOOLONG`: `name` was too long.
	/// * `ENOENT`: There is no message queue with the given `name`.
	pub(crate) fn mq_unlink(name: *const c_char) -> c_int;
}
