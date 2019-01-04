// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// This is the Kernel lower limit of `SIGRTMIN`.
pub(crate) const SIGRTMIN_Kernel: c_int = 32;

/// In theory, this is a libc-specific value, but both modern glibc and musl define it as `35` (in the past, when using an older threading implementation, glibc defined it as `34`).
///
/// The kernel lower-limit is 32.
pub(crate) const SIGRTMIN: c_int = 35;

/// This value is defined by the kernel.
///
/// There seems to be a bug in musl that defines this value as `127` for MIPS, but the kernel sources disagree.
#[cfg(any(target_arch = "mips", target_arch = "mips64"))] pub(crate) const SIGRTMAX: c_int = 128;
#[cfg(not(any(target_arch = "mips", target_arch = "mips64")))] pub(crate) const SIGRTMAX: c_int = 64;
