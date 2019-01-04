// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Supposedly generated from within the kernel without libc intervention.
pub(crate) const SI_KERNEL: c_int = 128;

/// Caused by the syscall the libc function `kill()` calls.
pub(crate) const SI_USER: c_int = 0;

/// Supposedly caused by the libc function `sigqueue()`, but the syscall this calls can take any *negative* `SI_*` value (ie not `SI_KERNEL` or `SI_USER`) apart from `SI_TKILL`.
pub(crate) const SI_QUEUE: c_int = -1;

/// Caused by libc & kernel implementations of POSIX timers.
///
/// Constant value which is defined differently for the MIPS architecture for no good reason.
#[cfg(not(any(target_arch = "mips", target_arch = "mips64")))] pub(crate) const SI_TIMER: c_int = -2;
#[cfg(any(target_arch = "mips", target_arch = "mips64"))] pub(crate) const SI_TIMER: c_int = -3;

/// Caused by libc implementations of POSIX message queues.
///
/// Constant value which is defined differently for the MIPS architecture for no good reason.
#[cfg(not(any(target_arch = "mips", target_arch = "mips64")))] pub(crate) const SI_MESGQ: c_int = -3;
#[cfg(any(target_arch = "mips", target_arch = "mips64"))] pub(crate) const SI_MESGQ: c_int = -4;

/// Caused by libc implementations of POSIX AIO.
///
/// Constant value which is defined differently for the MIPS architecture for no good reason.
#[cfg(not(any(target_arch = "mips", target_arch = "mips64")))] pub(crate) const SI_ASYNCIO: c_int = -4;
#[cfg(any(target_arch = "mips", target_arch = "mips64"))] pub(crate) const SI_ASYNCIO: c_int = -2;

/// Obsolete and should not occur.
pub(crate) const SI_SIGIO: c_int = -5;

/// Caused by the syscalls the libc functions `raise()`, ``tkill()` and `tgkill()` call.
pub(crate) const SI_TKILL: c_int = -6;

/// Sent by `execve()` killing subsidiary threads.
///
/// Not obvious if this can escape the kernel.
pub(crate) const SI_DETHREAD: c_int = -7;

/// glibc asynchronous (DNS) name look up.
pub(crate) const SI_ASYNCNL: c_int = -60;
