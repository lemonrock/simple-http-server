// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Received signal information.
#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct signalfd_siginfo
{
	/// Signal number.
	pub ssi_signo: uint32_t,

	/// Error number (unused).
	pub ssi_errno: int32_t,

	/// Signal code.
	pub ssi_code: int32_t,

	/// PID of sender (for signals `SIGCHLD` and `sigqueue()`).
	pub ssi_pid: int32_t,

	/// Real UID of sender (for signals `SIGCHLD` and `sigqueue()`).
	pub ssi_uid: uint32_t,

	/// File descriptor (for signal `SIGIO` (for `poll()`)).
	pub ssi_fd: int32_t,

	/// Kernel timer ID (for a POSIX timer signal).
	pub ssi_tid: uint32_t,

	/// Band event (for signal `SIGIO` (for `poll()`)).
	pub ssi_band: uint32_t,

	/// POSIX timer overrun count.
	pub ssi_overrun: uint32_t,

	/// Trap number that caused signal.
	pub ssi_trapno: uint32_t,

	/// Exit status or signal (for signal `SIGCHLD`).
	pub ssi_status: int32_t,

	/// Integer sent by `sigqueue()` and POSIX timers.
	pub ssi_int: int32_t,

	/// Pointer sent by `sigqueue()` and POSIX timers.
	pub ssi_ptr: uint64_t,

	/// User CPU time consumed (for signal `SIGCHLD`).
	pub ssi_utime: uint64_t,

	/// System CPU time consumed (for signal `SIGCHLD`).
	pub ssi_stime: uint64_t,

	/// Address that generated signal (for hardware-generated signals).
	pub ssi_addr: uint64_t,

	/// Least significant bit of address (for signal SIGBUS).
	///
	/// Since Linux 2.6.37.
	pub ssi_addr_lsb: uint16_t,

	_pad2: uint16_t,

	/// System call number.
	pub ssi_syscall: int32_t,

	/// System call address.
	pub ssi_call_addr: uint64_t,

	/// System call architecture.
	pub ssi_arch: uint32_t,

	_pad: [uint8_t; 28],
}
