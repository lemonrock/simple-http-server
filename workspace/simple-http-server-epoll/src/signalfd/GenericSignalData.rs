// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Generic signal data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GenericSignalData
{
	/// The signal was generated either by userspace or by the kernel, but has no explicitly set data.
	///
	/// The original signal code was `SI_KERNEL` and the internal kernel representation used for copying data to user space in `kernel/signal.c`'s `__send_signal()`. is `SEND_SIG_PRIV` (`PRIV` is short for private).
	Private,

	/// The signal was generated either by userspace or by the kernel (typically through `raise()` or `kill()`), but has a `pid` (process identifier) and `uid` (user identifier).
	///
	/// The values of `pid` and `uid` should be valid but the kernel signal code has had CVEs and it is advisable to not use them security checks.
	///
	/// The original signal code was `SI_USER` and the internal kernel representation used for copying data to user space in `kernel/signal.c`'s `__send_signal()`. is `SEND_SIG_NOINFO`.
	User
	{
		/// Process identifier.
		pid: pid_t,

		/// User identifier.
		uid: uid_t,
	},

	/// The signal was generated by userspace making a system call via `raise()`, `tkill()` or `tgkill()`.
	///
	/// The values of `pid` and `uid` should be valid but the kernel signal code has had CVEs and it is advisable to not use them security checks.
	///
	/// The original signal code was `SI_TKILL`.
	TKill
	{
		/// Process identifier.
		pid: pid_t,

		/// User identifier.
		uid: uid_t,
	},

	/// The signal was generated in userspace using a system call that allowed near arbitrary data to be passed, or inside the kernel using similar functionality.
	///
	/// The kernel has extremely weak checks validating the structure of such data matches its signal number and code.
	///
	/// The associated data is completely untrustworthy and is not passed.
	///
	/// This makes the use of POSIX timers, asynchronous IO, POSXI message queue notifications and `sigqueue()`'s ability to send additional data and glibc's asynchronous DNS look up unusable.
	Userspace(UserspaceSignalCode),
}
